use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{ChannelId, CreateChannel, Permissions};

use crate::{
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tool::{Status, Tool, ToolError, ToolResult},
};

#[derive(Deserialize, JsonSchema)]
pub struct Arguments {
    #[schemars(
        description = "The name of the channel. Spaces will get replaced with dashes. The name will be transformed to lowercase. Any unicode emojis or symbols are allowed."
    )]
    pub name: String,

    #[schemars(description = "An optional ID of a category to create the channel in.")]
    pub category_id: Option<String>,
}

pub struct CreateTextChannelTool;

impl Tool for CreateTextChannelTool {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "create_text_channel"
    }

    fn description(&self) -> &'static str {
        "Create a text channel in the guild."
    }

    async fn execute(
        &self,
        args: Self::Args,
        ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let name = args.name.to_lowercase().replace(" ", "-");

        let Some(guild_id) = ctx.guild_id else {
            return Err(ToolError::custom("you are not operating inside a guild"));
        };

        let mut approval = ApprovalBuilder::new(
            "create a text channel",
            NeededPermission::Basic(Permissions::MANAGE_CHANNELS),
        )
        .extra_data(json!({
            "channel_name": name
        }))
        .inline_arg("Channel Name", format!("`#{}`", name));

        let mut builder = CreateChannel::new(&name);

        if let Some(category_id) = args.category_id {
            let Ok(category_id) = category_id.parse::<ChannelId>() else {
                return Err(ToolError::validation(
                    "category_id",
                    "unable to parse as ChannelId",
                ));
            };

            builder = builder.category(category_id);

            let category_name = category_id
                .name(ctx.http())
                .await
                .unwrap_or(category_id.to_string());

            approval = approval
                .inline_arg("Category", format!("`{}`", category_name))
                .extra_data(json!({
                    "channel_name": name,
                    "category_id": category_id
                }));
        };

        let approval = approval
            .on_approval(async move |ctx| {
                let channel = match guild_id.create_channel(ctx.http(), builder).await {
                    Ok(channel) => channel,
                    Err(err) => {
                        return Ok(Some(json!({
                            r"error": format!("failed to create channel: {err}")
                        })));
                    }
                };

                Ok(Some(json!({
                    r"created_channel_id": channel.id.to_string()
                })))
            })
            .build();

        let approval_id = ctx.approval_manager.register(ctx.clone(), approval).await?;

        Ok(Status::pending_approval(approval_id, None))
    }
}
