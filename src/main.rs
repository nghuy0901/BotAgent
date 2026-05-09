pub(crate) mod agent;
pub(crate) mod constant;

use std::{collections::HashMap, env, sync::Arc};

use async_openai::config::{Config, OpenAIConfig};
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Message, Ready},
    async_trait,
};
use tokio::sync::RwLock;
use tracing::{Level, error, info};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::agent::{
    Agent,
    channel::AgentChannel,
    event::{AgentEvent, EventContent},
};

struct Handler<C: Config> {
    pub model: String,

    pub base_client: Arc<async_openai::Client<C>>,
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

                let new_agent = Arc::new(AgentChannel::new(Agent::new(
                    &self.model,
                    self.base_client.clone(),
                    ctx.http.clone(),
                    ctx.cache.clone(),
                )));

                self.agents
                    .write()
                    .await
                    .insert(channel_id, new_agent.clone());

                new_agent
            }
        };

        if let Err(err) = agent.tx.try_send(
            AgentEvent::new(EventContent::Message {
                author: message.author.name,
                content: message.content,
            })
            .with_timestamp(message.timestamp.timestamp()),
        ) {
            error!(channel_id, "unable to send event to agent: {:?}", err)
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

    let mut client = Client::builder(
        bot_token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler {
        // TODO: MAKE THIS A PARAMETER / VARIABLE
        model: "Qwen3.5-9B".to_string(),
        base_client: Arc::new(async_openai::Client::with_config(OpenAIConfig::new())),
        agents: RwLock::const_new(HashMap::new()),
    })
    .await
    .expect("error while creating client");

    if let Err(err) = client.start().await {
        error!("client errored: {:?}", err);
    }
}
