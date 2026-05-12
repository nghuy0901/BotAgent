use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{CreateChannel, Permissions};

use crate::agent::{
    approval::{NeededPermission, builder::ApprovalBuilder},
    tools::discord::DiscordTool,
};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(
        description = "The name of the channel. Spaces will get replaced with dashes. The name will be transformed to lowercase. Any unicode emojis or symbols are allowed."
    )]
    pub name: String,
}

pub struct CreateTextChannelTool;

impl DiscordTool for CreateTextChannelTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "channel.create_text"
    }

    fn description(&self) -> &'static str {
        "Creates a Discord text channel"
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: crate::agent::tools::discord::DiscordContext,
    ) -> crate::agent::Result<Self::Returns> {
        let Some(channel) = ctx.get_operating_channel().await?.guild() else {
            return Ok(json!({
                "success": false,
                "reason": "You are not operating inside of a guild."
            }));
        };

        let guild_id = channel.guild_id;

        let approval = ApprovalBuilder::new(
            "create a text channel",
            NeededPermission::Basic(Permissions::MANAGE_CHANNELS),
        )
        .param_inline("Channel Name", &params.name)
        .on_approval(async move |ctx| {
            let guild = ctx.http.get_guild(guild_id).await?;
            let channel = match guild
                .create_channel(&ctx.http, CreateChannel::new(params.name))
                .await
            {
                Ok(channel) => channel,
                Err(_) => {
                    return Ok(Some(json!({
                        r"success": false,
                        r"reason": "unknown error"
                    })));
                }
            };

            Ok(Some(json!({
                r"success": true,
                r"created_channel_id": channel.id.to_string()
            })))
        })
        .build();

        let approval_message = approval.to_message();
        let approval_id = approval.id.clone();

        ctx.approval_manager.register(approval);
        channel.send_message(&ctx.http, approval_message).await?;

        Ok(json!({
            "awaiting_approval": true,
            "approval_id": approval_id,
        }))
    }
}
