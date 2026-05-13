use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{CreateChannel, Permissions};

use crate::{
    Result,
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tools::Tool,
};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(
        description = "The name of the channel. Spaces will get replaced with dashes. The name will be transformed to lowercase. Any unicode emojis or symbols are allowed."
    )]
    pub name: String,
}

pub struct CreateTextChannelTool;

impl Tool for CreateTextChannelTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "channel.create_text"
    }

    fn description(&self) -> &'static str {
        "Create a text channel in the guild."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let name = params.name.to_lowercase().replace(" ", "-");

        let Some(guild_id) = ctx.guild_id else {
            return Ok(json!({
                "error": "you are not operating inside a guild"
            }));
        };

        let approval = ApprovalBuilder::new(
            "create a text channel",
            NeededPermission::Basic(Permissions::MANAGE_CHANNELS),
        )
        .param_inline("Channel Name", format!("#{}", name))
        .on_approval(async move |ctx| {
            let channel = match guild_id
                .create_channel(ctx.http(), CreateChannel::new(name))
                .await
            {
                Ok(channel) => channel,
                Err(err) => {
                    return Ok(Some(json!({
                        r"error": format!("failed to create channel: {err}")
                    })));
                }
            };

            Ok(Some(json!({
                r"success": true,
                r"created_channel_id": channel.id.to_string()
            })))
        })
        .build();

        let approval_id = ctx.approval_manager.register(ctx.clone(), approval).await?;

        Ok(json!({
            "awaiting_approval": true,
            "approval_id": approval_id,
        }))
    }
}
