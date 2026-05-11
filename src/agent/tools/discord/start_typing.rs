use serde_json::{Value, json};
use serenity::all::ChannelId;

use crate::agent::tools::{EmptyParameters, discord::DiscordTool};

/// This tool allows Sweep to send typing effects and makes it act more humane.
///
/// There are a few problems, like:
/// - Typing might not stay for the period of time it is needed
/// - The LLM might trigger typing and then not send a message
/// - The LLM might "forget" to trigger typing
///
/// I think this is a good approach for implementing something like this. We might introduce an option to toggle it when the TOML config is there.
pub struct StartTypingTool;

impl DiscordTool for StartTypingTool {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "start_typing"
    }

    fn description(&self) -> &'static str {
        "Always use this before sending a new message."
    }

    async fn execute(
        &self,
        _params: Self::Params,
        ctx: super::DiscordContext,
    ) -> crate::agent::Result<Self::Returns> {
        let _ = ctx
            .http
            .broadcast_typing(ChannelId::new(ctx.operating_channel))
            .await;

        Ok(json!({
            "typing": true
        }))
    }
}
