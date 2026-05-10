use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::ReactionType;

use crate::agent::tools::discord::DiscordTool;

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The ID of the message.")]
    pub message_id: String,
    #[schemars(description = "The emoji to react with. This must be a unicode emoji.")]
    pub emoji: String,
}

pub struct ReactMessageTool;

impl DiscordTool for ReactMessageTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "message.react"
    }

    fn description(&self) -> &'static str {
        "Reacts to a Discord message with an emoji. Never over-use."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: crate::agent::tools::discord::DiscordContext,
    ) -> crate::agent::Result<Self::Returns> {
        let message = ctx
            .http
            .get_message(ctx.operating_channel.into(), params.message_id.parse()?)
            .await?;

        Ok(
            match message
                .react(&ctx.http, ReactionType::Unicode(params.emoji))
                .await
            {
                Ok(_) => json!({ "reacted": true }),
                Err(_) => json!({ "error": "unable to react" }),
            },
        )
    }
}
