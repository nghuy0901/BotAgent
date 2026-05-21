pub(crate) mod agent;
pub(crate) mod approval;
pub(crate) mod config;
pub(crate) mod tools;
pub(crate) mod util;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use std::{env, process, sync::Arc};

use serde_json::json;
use serenity::{
    Client,
    all::{
        Context, CreateEmbed, CreateEmbedFooter, CreateInteractionResponseFollowup, EditMessage,
        EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait,
};
use tracing::instrument;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    agent::{
        Agent,
        channel::AgentChannel,
        context::{AgentContext, DedicatedContext},
        event::{AgentEvent, EventContent},
    },
    approval::NeededPermission,
    tools::{container::ToolContainer, impls::AllTools},
};

struct Handler {
    pub agent_context: Arc<AgentContext>,
}

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip_all)]
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("started as {}", ready.user.name);
    }

    #[instrument(skip_all, fields(message_id = %message.id, author_id=%message.author.id, channel_id=%message.channel_id))]
    async fn message(&self, ctx: Context, message: Message) {
        let config = &self.agent_context.configuration;
        let author_id = message.author.id;

        let channel_id = message.channel_id.get();
        let channel_override = config.channel.overrides.iter().find(|o| o.id == channel_id);

        if author_id == ctx.cache.current_user().id {
            return;
        }

        if let Some(o) = channel_override {
            if !o.enable {
                tracing::debug!("channel is disabled via override: skipping message");

                return;
            }
        } else {
            if !config.channel.whitelist.is_empty() {
                if !config.channel.whitelist.contains(&channel_id) {
                    tracing::debug!("non-whitelisted channel: skipping message");

                    return;
                }
            } else if config.channel.blacklist.contains(&channel_id) {
                tracing::debug!("blacklisted channel: skipping message");

                return;
            }
        }

        if !config.users.whitelist.is_empty() {
            if !config.users.whitelist.contains(&author_id.get()) {
                tracing::debug!("non-whitelisted user: skipping message");

                return;
            }
        } else if config.users.blacklist.contains(&author_id.get()) {
            tracing::debug!("blacklisted user: skipping message");

            return;
        }

        let guild_id = message.guild_id;
        let channel_id = message.channel_id;

        let agent = match self.agent_context.agents.get(&channel_id) {
            Some(a) => a.clone(),
            None => {
                tracing::info!("creating agent");

                let mut dedicated_context =
                    DedicatedContext::new(self.agent_context.clone(), channel_id, channel_override);

                dedicated_context.guild_id = guild_id;

                let agent = Agent::new(Arc::new(dedicated_context));
                let new_agent = Arc::new(AgentChannel::new(agent));

                self.agent_context
                    .agents
                    .insert(channel_id, new_agent.clone());

                new_agent
            }
        };

        let author = message.author;
        let content = message.content.replace(
            format!("<@{}>", ctx.cache.current_user().id).as_str(),
            "Sweep",
        );

        if let Err(err) = agent.tx.try_send(
            AgentEvent::new(EventContent::Message {
                channel_id: channel_id.to_string(),
                message_id: message.id.to_string(),
                author: json!({
                    "username": author.name,
                    "display_name": author.display_name(),
                    "user_id": author.id.get()
                }),
                content,
            })
            .with_timestamp(message.timestamp.timestamp() as u64),
        ) {
            tracing::error!(
                "unable to send event to agent for channel {}: {:?}",
                channel_id,
                err
            )
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Component(mut component) = interaction {
            let custom_id = component.data.custom_id.clone();
            let channel_id = component.channel_id;

            if !custom_id.starts_with("approve-") && !custom_id.starts_with("deny-") {
                tracing::warn!("unknown component id: {}", custom_id);

                return;
            }

            let _ = component
                .create_response(
                    &ctx.http,
                    serenity::all::CreateInteractionResponse::Acknowledge,
                )
                .await;

            let is_approved = custom_id.starts_with("approve-");
            let approval_id = {
                let chars = custom_id.chars().rev().take(12).collect::<Vec<_>>();

                String::from_iter(chars.into_iter().rev())
            };

            let needed_permissions = match self
                .agent_context
                .approval_manager
                .get_needed_permission(&approval_id)
            {
                Some(a) => a,
                None => {
                    tracing::error!("unable to find approval with id: {}", approval_id);

                    return;
                }
            };

            let has_permission = match needed_permissions {
                NeededPermission::Basic(permissions) => component
                    .member
                    .as_ref()
                    .and_then(|m| m.permissions)
                    .map(|p| p.contains(permissions))
                    .unwrap_or(false),
                NeededPermission::InChannel(channel_id, permissions) => {
                    if let Some(guild_id) = component.guild_id {
                        let guild_channel = match channel_id
                            .to_channel(&ctx.http)
                            .await
                            .ok()
                            .and_then(|c| c.guild())
                        {
                            Some(c) => c,
                            None => {
                                tracing::error!(
                                    "unable to convert interaction channel to guild channel"
                                );

                                return;
                            }
                        };

                        let guild = match guild_id.to_partial_guild(&ctx.http).await {
                            Ok(g) => g,
                            Err(err) => {
                                tracing::error!(
                                    "unable to get guild of id {}: {:?}",
                                    guild_id,
                                    err
                                );

                                return;
                            }
                        };

                        let member = match guild.member(&ctx.http, component.user.id).await {
                            Ok(m) => m,
                            Err(err) => {
                                tracing::error!(
                                    "unable to get guild member with id {} in guild {}: {:?}",
                                    component.user.id,
                                    guild_id,
                                    err
                                );

                                return;
                            }
                        };

                        guild
                            .user_permissions_in(&guild_channel, &member)
                            .contains(permissions)
                    } else {
                        false
                    }
                }
            };

            if !has_permission {
                let _ = component
                    .create_followup(
                        &ctx.http,
                        CreateInteractionResponseFollowup::new()
                            .content("You are lacking permission to approve this request.")
                            .ephemeral(true),
                    )
                    .await;

                return;
            }

            let channel_agent = match self.agent_context.agents.get(&channel_id) {
                Some(a) => a,
                None => {
                    tracing::error!("unable to find agent for channel {}", channel_id);

                    return;
                }
            };

            // we only take it here because if all the other fail, the approval should still persist
            let mut approval = match self.agent_context.approval_manager.take(&approval_id) {
                Some(a) => a,
                None => {
                    tracing::error!("unable to find basic approval with id: {}", approval_id);

                    return;
                }
            };

            let action = approval.metadata.action.clone();

            if is_approved {
                let data = match approval.approval_callback.take() {
                    Some(callback) => callback(channel_agent.dedicated_context.clone()).await,
                    None => Ok(None),
                };

                let data = match data {
                    Ok(d) => d,
                    Err(err) => {
                        tracing::error!(
                            "error while executing callback for approval with id {}: {:?}",
                            approval_id,
                            err
                        );

                        return;
                    }
                };

                if let Err(err) =
                    channel_agent
                        .tx
                        .try_send(AgentEvent::new(EventContent::RequestApproved {
                            approval_id,
                            metadata: approval.metadata,
                            data,
                        }))
                {
                    tracing::error!(
                        "unable to send request approval message to agent in channel {}: {:?}",
                        channel_id,
                        err
                    );
                }
            } else {
                if let Err(err) =
                    channel_agent
                        .tx
                        .try_send(AgentEvent::new(EventContent::RequestDenied {
                            approval_id,
                            metadata: approval.metadata,
                        }))
                {
                    tracing::error!(
                        "unable to send request denial message to agent in channel {}: {:?}",
                        channel_id,
                        err
                    );
                }
            }

            if let Err(err) = component
                .message
                .edit(
                    &ctx.http,
                    EditMessage::new()
                        .components(vec![])
                        .remove_all_attachments()
                        .embed(
                            CreateEmbed::new()
                                .title(if is_approved {
                                    "✅ Approved"
                                } else {
                                    "🚫 Denied"
                                })
                                .description(format!(
                                    "The action to **{}** was **{}**.",
                                    action,
                                    if is_approved { "approved" } else { "denied" }
                                ))
                                .footer(CreateEmbedFooter::new("")),
                        ),
                )
                .await
            {
                tracing::error!(
                    "unable to edit approval message with id {}: {:?}",
                    component.message.id,
                    err
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("sweep=info")))
        .init();

    // Read environment variables from .env. Ignore file errors
    let _ = dotenvy::dotenv();

    tracing::info!("loading configuration");

    let config = match config::load() {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                "invalid configuration: {}{}",
                err.kind,
                if !err.path.is_empty() {
                    format!(" in key `{}`", err.path.join("."))
                } else {
                    String::new()
                }
            );

            process::exit(1);
        }
    };

    if env::var("SWEEP__DISCORD__TOKEN").is_err() {
        tracing::warn!(
            "SWEEP__DISCORD__TOKEN not set in environment: consider using an env variable"
        );
    }

    if config.llm.endpoint.trim().is_empty() {
        tracing::error!(
            "invalid OpenAI base url: set the llm.endpoint config to a non-empty string"
        );

        process::exit(1);
    }

    if !config.channel.blacklist.is_empty() && !config.channel.whitelist.is_empty() {
        tracing::warn!(
            "both channel.blacklist and channel.whitelist are set: the whitelist will be preferred"
        );
    }

    if !config.users.blacklist.is_empty() && !config.users.whitelist.is_empty() {
        tracing::warn!(
            "both users.blacklist and users.whitelist are set: the whitelist will be preferred"
        );
    }

    let bot_token = config.discord.token.clone();

    let handler_arc = Arc::new(Handler {
        agent_context: Arc::new(AgentContext::new(
            config,
            ToolContainer::default().with_domain::<AllTools>(),
        )),
    });

    let mut client = Client::builder(
        bot_token,
        GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILDS,
    )
    .event_handler_arc(handler_arc.clone())
    .await
    .expect("error while creating client");

    handler_arc
        .agent_context
        .setup(client.cache.clone(), client.http.clone());

    if let Err(err) = client.start().await {
        tracing::error!("client errored: {:?}", err);
    }
}
