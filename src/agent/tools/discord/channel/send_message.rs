use std::str::FromStr;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{Channel, ChannelId, CreateMessage};

use crate::agent::tools::{
    Result,
    discord::{DiscordContext, DiscordTool},
};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The channel id of the channel you want to send the message in.")]
    pub channel_id: String,
    #[schemars(description = "The content of the message.")]
    pub content: String,
}

pub struct SendMessageTool;

impl DiscordTool for SendMessageTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "channel.send_message"
    }

    fn description(&self) -> &'static str {
        "Sends a Discord message in the specified channel with the given content."
    }

    async fn execute(&self, params: Self::Params, ctx: DiscordContext) -> Result<Self::Returns> {
        let channel = match ctx
            .http
            .get_channel(ChannelId::from_str(&params.channel_id)?)
            .await
        {
            Ok(channel) => channel,
            Err(_) => {
                return Ok(json!({
                    "message_sent": false,
                    "reason": "unknown channel id"
                }));
            }
        };

        let builder = CreateMessage::new().content(params.content);

        let (channel_kind, message) = match channel {
            Channel::Guild(channel) => ("guild", channel.send_message(&ctx.http, builder).await?),
            Channel::Private(channel) => (
                "direct_messages",
                channel.send_message(&ctx.http, builder).await?,
            ),
            _ => {
                return Ok(json!({
                    "message_sent": false,
                    "reason": "unknown error"
                }));
            }
        };

        Ok(json!({
            "message_sent": true,
            "channel_kind": channel_kind,
            "sent_message_id": message.id.to_string()
        }))
    }
}
