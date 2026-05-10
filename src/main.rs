pub(crate) mod agent;
pub(crate) mod constant;

use std::{collections::HashMap, env, sync::Arc};

use async_openai::config::{Config, OpenAIConfig};
use serenity::{
    Client,
    all::{
        Context, CreateEmbed, CreateEmbedFooter, CreateInteractionResponseFollowup, EditMessage,
        EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait,
};
use tokio::sync::RwLock;
use tracing::{Level, error, info, warn};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    agent::{
        Agent,
        approval::manager::ApprovalManager,
        channel::AgentChannel,
        config::AgentConfig,
        event::{AgentEvent, EventContent},
        tools::{
            basic::finish::FinishTool,
            discord::{
                DiscordContext,
                channel::{create_text::CreateTextChannelTool, send_message::SendMessageTool},
            },
        },
    },
    constant::SYSTEM_PROMPT,
};

struct Handler<C: Config> {
    pub agent_config: Arc<AgentConfig>,
    pub base_client: Arc<async_openai::Client<C>>,
    pub approval_manager: Arc<ApprovalManager>,
    pub agents: RwLock<HashMap<u64, Arc<AgentChannel>>>,
}

#[async_trait]
impl<C: Config + 'static> EventHandler for Handler<C> {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("started as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.id == ctx.cache.current_user().id {
            return;
        }

        let channel_id = message.channel_id.get();

        let agents = self.agents.read().await;
        let agent = match agents.get(&channel_id) {
            Some(a) => a.clone(),
            None => {
                drop(agents);

                info!("creating agent for channel {}", channel_id);

                let agent = Agent::new(
                    channel_id,
                    self.agent_config.clone(),
                    self.base_client.clone(),
                    self.approval_manager.clone(),
                    ctx.http.clone(),
                    ctx.cache.clone(),
                )
                .with_system_prompt(SYSTEM_PROMPT);

                let new_agent = Arc::new(AgentChannel::new(agent));

                self.agents
                    .write()
                    .await
                    .insert(channel_id, new_agent.clone());

                new_agent
            }
        };

        if let Err(err) = agent.tx.try_send(
            AgentEvent::new(EventContent::Message {
                guild_id: message.guild_id.map(|g| g.get().to_string()),
                channel_id: channel_id.to_string(),
                author: message.author.name,
                content: message.content,
            })
            .with_timestamp(message.timestamp.timestamp()),
        ) {
            error!(channel_id, "unable to send event to agent: {:?}", err)
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Component(mut component) = interaction {
            let custom_id = component.data.custom_id.clone();
            let channel_id = component.channel_id.get();

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

            let basic_approval = match self.approval_manager.get_basic_approval(&approval_id).await
            {
                Some(a) => a,
                None => {
                    error!("unable to find approval with id: {}", approval_id);

                    return;
                }
            };

            if !component
                .member
                .as_ref()
                .map(|m| m.permissions)
                .flatten()
                .map(|p| p.contains(basic_approval.needs_permissions))
                .unwrap_or(false)
            {
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

            let channel_agent = match self.agents.read().await.get(&channel_id).cloned() {
                Some(a) => a,
                None => {
                    error!("unable to find agent for channel {}", channel_id);

                    return;
                }
            };

            // we only take it here because if all the other fail, the approval should still persist
            let mut approval = match self.approval_manager.take(&approval_id).await {
                Some(a) => a,
                None => {
                    error!("unable to find basic approval with id: {}", approval_id);

                    return;
                }
            };

            if is_approved {
                let data = match approval.approval_callback.take() {
                    Some(callback) => {
                        callback(DiscordContext {
                            approval_manager: self.approval_manager.clone(),
                            cache: ctx.cache.clone(),
                            http: ctx.http.clone(),
                            operating_channel: channel_id,
                        })
                        .await
                    }
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
                    EditMessage::new().components(vec![]).embed(
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

    let agent_config = AgentConfig::default()
        .with_model(model)
        .with_basic_tool(FinishTool)
        .with_discord_tool(CreateTextChannelTool)
        .with_discord_tool(SendMessageTool);

    let mut client = Client::builder(
        bot_token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler {
        agent_config: Arc::new(agent_config),
        base_client: Arc::new(async_openai::Client::with_config(OpenAIConfig::new())),
        approval_manager: Arc::new(ApprovalManager::default()),
        agents: RwLock::const_new(HashMap::new()),
    })
    .await
    .expect("error while creating client");

    if let Err(err) = client.start().await {
        error!("client errored: {:?}", err);
    }
}
