use std::env;

use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Ready},
    async_trait,
};
use tracing::{Level, error, info};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("started as {}", ready.user.name);
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
    .event_handler(Handler)
    .await
    .expect("error while creating client");

    if let Err(err) = client.start().await {
        error!("client errored: {:?}", err);
    }
}
