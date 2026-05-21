use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use serenity::all::{ChannelId, Permissions};

use crate::{
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tools::{Status, Tool, ToolError, ToolResult},
};

#[derive(Deserialize, JsonSchema)]
pub struct Arguments {
    #[schemars(description = "The ID of the channel.")]
    pub channel_id: String,

    #[schemars(
        description = "Why you want to delete this channel (short). Always include the username of the user that request this action (if provided)."
    )]
    pub reason: String,
}

pub struct DeleteChannelTool;

impl Tool for DeleteChannelTool {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "delete_channel"
    }

    fn description(&self) -> &'static str {
        "Delete a channel. Also works with categories."
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

        let approval = ApprovalBuilder::new(
            "delete a channel",
            NeededPermission::Basic(Permissions::MANAGE_CHANNELS),
        )
        .inline_arg("Channel", format!("<#{}>", channel_id))
        .inline_arg("Reason", format!("`{}`", args.reason))
        .on_approval(async move |ctx| {
            ctx.http()
                .delete_channel(channel_id, Some(&args.reason))
                .await?;

            Ok(None)
        })
        .build();

        let approval_id = ctx.approval_manager.register(ctx.clone(), approval).await?;

        Ok(Status::pending_approval(approval_id, None))
    }
}
