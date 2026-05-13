use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::ReactionType;

use crate::{Result, agent::context::DedicatedContext, tools::Tool};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The ID of the message.")]
    pub message_id: String,

    #[schemars(description = "The emoji to react with. This must be a unicode emoji.")]
    pub emoji: String,
}

pub struct ReactMessageTool;

impl Tool for ReactMessageTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "message.react"
    }

    fn description(&self) -> &'static str {
        "React to a Discord message with a unicode emoji. Use for acknowledgements or \
        lightweight responses where a full message would be excessive."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let message = ctx
            .http()
            .get_message(ctx.channel_id, params.message_id.parse()?)
            .await?;

        Ok(
            match message
                .react(ctx.http(), ReactionType::Unicode(params.emoji))
                .await
            {
                Ok(_) => json!({ "reacted": true }),
                Err(_) => json!({ "error": "unable to react" }),
            },
        )
    }
}
