use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::ReactionType;

use crate::{
    agent::context::DedicatedContext,
    tool::{Status, Tool, ToolError, ToolResult},
};

#[derive(Deserialize, JsonSchema)]
pub struct Arguments {
    #[schemars(description = "The ID of the message.")]
    pub message_id: String,

    #[schemars(description = "The emoji to react with. This must be a unicode emoji.")]
    pub emoji: String,
}

pub struct ReactMessageTool;

impl Tool for ReactMessageTool {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "react_message"
    }

    fn description(&self) -> &'static str {
        "React to a Discord message with a unicode emoji. Use for acknowledgements or \
        lightweight responses where a full message would be excessive."
    }

    async fn execute(
        &self,
        args: Self::Args,
        ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let Ok(message_id) = args.message_id.parse() else {
            return Err(ToolError::validation(
                "message_id",
                "unable to parse as MessageId",
            ));
        };

        let message = ctx.http().get_message(ctx.channel_id, message_id).await?;

        message
            .react(ctx.http(), ReactionType::Unicode(args.emoji))
            .await?;

        Ok(Status::success(json!({ "reacted": true })))
    }
}
