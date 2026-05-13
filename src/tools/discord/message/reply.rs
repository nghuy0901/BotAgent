use std::{str::FromStr, sync::Arc};

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{CreateMessage, MessageId, MessageReference};
use tracing::error;

use crate::{Result, agent::context::DedicatedContext, tools::Tool};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The message ID to reply to.")]
    pub message_id: String,

    #[schemars(description = "The content of your message.")]
    pub content: String,
}

pub struct ReplyToMessageTool;

impl Tool for ReplyToMessageTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "message.reply"
    }

    fn description(&self) -> &'static str {
        "Reply directly to a specific message, creating a visible quote. \
        Use when the context of which message you're responding to would otherwise \
        be ambiguous. Prefer channel.send_message for general responses."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let Ok(message_id) = MessageId::from_str(&params.message_id) else {
            return Ok(json!({
                "error": "unable to parse message_id"
            }));
        };

        Ok(
            match ctx
                .channel_id
                .send_message(
                    &ctx.http(),
                    CreateMessage::new()
                        .content(params.content)
                        .reference_message(MessageReference::from((ctx.channel_id, message_id))),
                )
                .await
            {
                Ok(reply) => json!({
                    "reply_message_id": reply.id,
                }),
                Err(err) => {
                    error!(
                        "error while replying to message {} in channel {}: {:?}",
                        message_id, ctx.channel_id, err
                    );

                    json!({
                        "error": format!("unable to reply to message: {err}")
                    })
                }
            },
        )
    }
}
