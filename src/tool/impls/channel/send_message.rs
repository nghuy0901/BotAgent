use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{Channel, ChannelId, CreateMessage, Permissions};

use crate::{
    Result,
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tool::{Status, Tool, ToolError, ToolResult},
};

#[derive(Deserialize, JsonSchema)]
pub struct Arguments {
    #[schemars(description = "The channel id of the channel you want to send the message in.")]
    pub channel_id: String,
    #[schemars(
        description = "The content of the message. Do not include questions for the current user."
    )]
    pub content: String,
}

pub struct SendMessageTool;

impl Tool for SendMessageTool {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "send_message"
    }

    fn description(&self) -> &'static str {
        "Send a message to a different channel than the one you're chatting in. \
        Only use for cross-posting messages."
    }

    async fn execute(
        &self,
        args: Self::Args,
        ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let Ok(channel_id) = args.channel_id.parse::<ChannelId>() else {
            return Err(ToolError::validation(
                "channel_id",
                "unable to parse as ChannelId",
            ));
        };

        if channel_id == ctx.channel_id {
            return Err(ToolError::validation(
                "channel_id",
                "channel_id must be different than the channel you are in",
            ));
        }

        let channel = channel_id.to_channel(ctx.http()).await?;
        let builder = CreateMessage::new().content(&args.content);

        // we should attach it as a file if it reaches more than x lines to not clutter the channel
        let approval = ApprovalBuilder::new(
            "send a message in a different channel",
            NeededPermission::InChannel(channel_id, Permissions::SEND_MESSAGES),
        )
        .inline_arg("Channel", format!("<#{}>", channel_id))
        .field_arg("Content", args.content)
        .on_approval(async move |ctx| {
            send_to_channel(channel, builder, &ctx)
                .await
                .map(Option::Some)
        })
        .build();

        let approval_id = ctx
            .agent_context
            .approval_manager
            .register(ctx.clone(), approval)
            .await?;

        Ok(Status::pending_approval(approval_id, None))
    }
}

async fn send_to_channel(
    channel: Channel,
    builder: CreateMessage,
    ctx: &Arc<DedicatedContext>,
) -> Result<Value> {
    let message = match channel {
        Channel::Guild(channel) => channel.send_message(ctx.http(), builder).await?,
        _ => {
            return Ok(json!({
                "error": "unsupported channel kind"
            }));
        }
    };

    Ok(json!({
        "message_id": message.id.to_string()
    }))
}
