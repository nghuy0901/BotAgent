pub(crate) mod agent;
pub(crate) mod approval;
pub(crate) mod config;
pub(crate) mod constant;
pub(crate) mod tools;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use std::{env, sync::Arc, time::Duration};

use serde_json::json;
use serenity::{
    Client,
    all::{
        Context, CreateEmbed, CreateEmbedFooter, CreateInteractionResponseFollowup, EditMessage,
        EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait,
};
use tracing::{Level, error, info, warn};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    agent::{
        Agent,
        channel::AgentChannel,
        context::{AgentContext, DedicatedContext},
        event::{AgentEvent, EventContent},
    },
    approval::NeededPermission,
    config::Configuration,
    constant::SYSTEM_PROMPT,
    tools::{basic::BasicTools, container::ToolContainer, discord::DiscordTools},
};

struct Handler {
    pub agent_context: Arc<AgentContext>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("started as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.id == ctx.cache.current_user().id {
            return;
        }

        let guild_id = message.guild_id;
        let channel_id = message.channel_id;

        let agent = match self.agent_context.agents.get(&channel_id) {
            Some(a) => a.clone(),
            None => {
                info!("creating agent for channel {}", channel_id);

                let mut dedicated_context =
                    DedicatedContext::new(self.agent_context.clone(), channel_id);

                dedicated_context.guild_id = guild_id;

                let agent =
                    Agent::new(Arc::new(dedicated_context)).with_system_prompt(SYSTEM_PROMPT);

                let new_agent = Arc::new(AgentChannel::new(agent));

                self.agent_context
                    .agents
                    .insert(channel_id, new_agent.clone());

                new_agent
            }
        };

        let author = message.author;

        if let Err(err) = agent.tx.try_send(
            AgentEvent::new(EventContent::Message {
                channel_id: channel_id.to_string(),
                message_id: message.id.to_string(),
                author: json!({
                    "username": author.name,
                    "display_name": author.display_name(),
                    "user_id": author.id.get()
                }),
                content: message.content,
            })
            .with_timestamp(message.timestamp.timestamp() as u64),
        ) {
            error!(
                "unable to send event to agent for channel {}: {:?}",
                channel_id, err
            )
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Component(mut component) = interaction {
            let custom_id = component.data.custom_id.clone();
            let channel_id = component.channel_id;

            if !custom_id.starts_with("approve-") && !custom_id.starts_with("deny-") {
                warn!("unknown component id: {}", custom_id);

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
                    error!("unable to find approval with id: {}", approval_id);

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
                                error!("unable to convert interaction channel to guild channel");

                                return;
                            }
                        };

                        let guild = match guild_id.to_partial_guild(&ctx.http).await {
                            Ok(g) => g,
                            Err(err) => {
                                error!("unable to get guild of id {}: {:?}", guild_id, err);

                                return;
                            }
                        };

                        let member = match guild.member(&ctx.http, component.user.id).await {
                            Ok(m) => m,
                            Err(err) => {
                                error!(
                                    "unable to get guild member with id {} in guild {}: {:?}",
                                    component.user.id, guild_id, err
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
                    error!("unable to find agent for channel {}", channel_id);

                    return;
                }
            };

            // we only take it here because if all the other fail, the approval should still persist
            let mut approval = match self.agent_context.approval_manager.take(&approval_id) {
                Some(a) => a,
                None => {
                    error!("unable to find basic approval with id: {}", approval_id);

                    return;
                }
            };

            if is_approved {
                let data = match approval.approval_callback.take() {
                    Some(callback) => callback(channel_agent.dedicated_context.clone()).await,
                    None => Ok(None),
                };

                let data = match data {
                    Ok(d) => d,
                    Err(err) => {
                        error!(
                            "error while executing callback for approval with id {}: {:?}",
                            approval_id, err
                        );

                        return;
                    }
                };

                if let Err(err) =
                    channel_agent
                        .tx
                        .try_send(AgentEvent::new(EventContent::RequestApproved {
                            approval_id,
                            data,
                        }))
                {
                    error!(
                        "unable to send request approval message to agent in channel {}: {:?}",
                        channel_id, err
                    );
                }
            } else {
                if let Err(err) = channel_agent
                    .tx
                    .try_send(AgentEvent::new(EventContent::RequestDenied { approval_id }))
                {
                    error!(
                        "unable to send request denial message to agent in channel {}: {:?}",
                        channel_id, err
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
                                    approval.display_description,
                                    if is_approved { "approved" } else { "denied" }
                                ))
                                .footer(CreateEmbedFooter::new("")),
                        ),
                )
                .await
            {
                error!(
                    "unable to edit approval message with id {}: {:?}",
                    component.message.id, err
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Ignore all logs that don't belong to Sweep
    let filter = filter::Targets::new().with_target("sweep", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // Read environment variables from .env. Ignore file errors
    let _ = dotenvy::dotenv();

    let bot_token = env::var("DISCORD_TOKEN")
        .expect("unable to find the \"DISCORD_TOKEN\" environment variable");

    let model = env::var("MODEL").expect("unable to find the \"MODEL\" environment variable");
    let handler_arc = Arc::new(Handler {
        agent_context: Arc::new(AgentContext::new(
            Configuration {
                model,
                collect_duration: Duration::from_secs(1),
            },
            ToolContainer::default()
                .with_domain::<BasicTools>()
                .with_domain::<DiscordTools>(),
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
        error!("client errored: {:?}", err);
    }
}
