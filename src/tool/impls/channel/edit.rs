use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use serenity::all::{ChannelId, EditChannel, Permissions};

use crate::{
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tool::{Status, Tool, ToolError, ToolResult},
};

#[derive(Deserialize, JsonSchema)]
#[schemars(
    description = "The properties to change. Optional values must only be set if that property should be changed."
)]
pub struct Arguments {
    #[schemars(description = "The ID of the channel.")]
    pub channel_id: String,

    #[schemars(
        description = "The new name. Must be lowercase and spaces get replaced with a - character."
    )]
    pub new_name: Option<String>,
}

pub struct EditChannelTool;

impl Tool for EditChannelTool {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "edit_channel"
    }

    fn description(&self) -> &'static str {
        "Edit the properties of a channel. This also works for categories."
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

        let mut changed_properties: Vec<(String, String)> = Vec::new();
        let mut edit = EditChannel::new();

        if let Some(new_name) = args.new_name {
            let new_name = new_name.to_lowercase().replace(" ", "-");

            changed_properties.push(("New Name".to_string(), format!("#{new_name}")));

            edit = edit.name(new_name);
        }

        if changed_properties.is_empty() {
            return Err(ToolError::custom("at least one property must be changed."));
        }

        let mut approval = ApprovalBuilder::new(
            "edit a channel",
            NeededPermission::Basic(Permissions::MANAGE_CHANNELS),
        )
        .inline_arg("Channel", format!("<#{channel_id}>"))
        .on_approval(async move |ctx| {
            channel_id.edit(ctx.http(), edit).await?;

            Ok(None)
        });

        for (name, value) in changed_properties {
            approval = approval.inline_arg(name, format!("`{value}`"));
        }

        let approval_id = ctx
            .approval_manager
            .register(ctx.clone(), approval.build())
            .await?;

        Ok(Status::pending_approval(approval_id, None))
    }
}
