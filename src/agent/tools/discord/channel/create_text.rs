use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{ChannelId, CreateChannel, Permissions};

use crate::agent::{approval::builder::ApprovalBuilder, tools::discord::DiscordTool};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(
        description = "The name of the channel. Lowercase only. Spaces get replaced with dashes."
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
        let Some(channel) = ctx
            .http
            .get_channel(ChannelId::new(ctx.operating_channel))
            .await?
            .guild()
        else {
            return Ok(json!({
                "success": false,
                "reason": "You are not operating inside of a guild."
            }));
        };

        let guild_id = channel.guild_id.clone();

        let approval = ApprovalBuilder::new("create a text channel", Permissions::MANAGE_CHANNELS)
            .param("Channel Name", &params.name)
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

        ctx.approval_manager.register(approval).await;
        channel.send_message(&ctx.http, approval_message).await?;

        Ok(json!({
            "awaiting_approval": true,
            "approval_id": approval_id,
        }))
    }
}
