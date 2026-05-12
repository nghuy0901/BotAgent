/* use std::str::FromStr;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{Channel, ChannelId, CreateMessage, Permissions};

use crate::agent::{
    Result,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tools::discord::{DiscordContext, DiscordTool},
};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The channel id of the channel you want to send the message in.")]
    pub channel_id: String,
    #[schemars(description = "The content of the message.")]
    pub content: String,
}

pub struct SendMessageTool;

impl DiscordTool for SendMessageTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "channel.send_message"
    }

    fn description(&self) -> &'static str {
        "Sends a Discord message in the specified channel with the given content."
    }

    async fn execute(&self, params: Self::Params, ctx: DiscordContext) -> Result<Self::Returns> {
        let channel = match ctx
            .http
            .get_channel(ChannelId::from_str(&params.channel_id)?)
            .await
        {
            Ok(channel) => channel,
            Err(_) => {
                return Ok(json!({
                    "message_sent": false,
                    "reason": "unknown channel id"
                }));
            }
        };

        let builder = CreateMessage::new().content(params.content.clone());

        if channel.id().get() != ctx.operating_channel {
            // we should attach it as a file if it reaches more than x lines to not clutter the channel
            let approval = ApprovalBuilder::new(
                "post a message in a different channel",
                NeededPermission::InChannel(channel.id(), Permissions::SEND_MESSAGES),
            )
            .param_field("Content", params.content)
            .on_approval(async move |ctx| {
                let (channel_kind, message) = match channel {
                    Channel::Guild(channel) => {
                        ("guild", channel.send_message(&ctx.http, builder).await?)
                    }
                    Channel::Private(channel) => (
                        "direct_messages",
                        channel.send_message(&ctx.http, builder).await?,
                    ),
                    _ => {
                        return Ok(Some(json!({
                            "message_sent": false,
                            "reason": "unknown error"
                        })));
                    }
                };

                Ok(Some(json!({
                    "message_sent": true,
                    "channel_kind": channel_kind,
                    "sent_message_id": message.id.to_string()
                })))
            })
            .build();

            let channel = ctx
                .get_operating_channel()
                .await
                .and_then(|c| c.guild().ok_or("not in a guild".into()))?;

            let approval_message = approval.to_message();
            let approval_id = approval.id.clone();

            ctx.approval_manager.register(approval);
            channel.send_message(&ctx.http, approval_message).await?;

            Ok(json!({
                "awaiting_approval": true,
                "approval_id": approval_id,
                "note": "cross channel posting requires approval"
            }))
        } else {
            let (channel_kind, message) = match channel {
                Channel::Guild(channel) => {
                    ("guild", channel.send_message(&ctx.http, builder).await?)
                }
                Channel::Private(channel) => (
                    "direct_messages",
                    channel.send_message(&ctx.http, builder).await?,
                ),
                _ => {
                    return Ok(json!({
                        "message_sent": false,
                        "reason": "unknown error"
                    }));
                }
            };

            Ok(json!({
                "message_sent": true,
                "channel_kind": channel_kind,
                "sent_message_id": message.id.to_string()
            }))
        }
    }
}
*/

use std::{str::FromStr, sync::Arc};

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{Channel, ChannelId, CreateMessage, Permissions};

use crate::{
    Result,
    agent::context::DedicatedContext,
    approval::{NeededPermission, builder::ApprovalBuilder},
    tools::Tool,
};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The channel id of the channel you want to send the message in.")]
    pub channel_id: String,
    #[schemars(description = "The content of the message.")]
    pub content: String,
}

pub struct SendMessageTool;

impl Tool for SendMessageTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "channel.send_message"
    }

    fn description(&self) -> &'static str {
        "Sends a Discord message in the specified channel with the given content."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let channel = match ctx
            .http()
            .get_channel(ChannelId::from_str(&params.channel_id)?)
            .await
        {
            Ok(channel) => channel,
            Err(_) => {
                return Ok(json!({
                    "message_sent": false,
                    "reason": "unknown channel id"
                }));
            }
        };

        let builder = CreateMessage::new().content(params.content.clone());

        if channel.id() != ctx.channel_id {
            // we should attach it as a file if it reaches more than x lines to not clutter the channel
            let approval = ApprovalBuilder::new(
                "post a message in a different channel",
                NeededPermission::InChannel(channel.id(), Permissions::SEND_MESSAGES),
            )
            .param_field("Content", params.content)
            .on_approval(async move |ctx| {
                let (channel_kind, message) = match channel {
                    Channel::Guild(channel) => {
                        ("guild", channel.send_message(ctx.http(), builder).await?)
                    }
                    Channel::Private(channel) => (
                        "direct_messages",
                        channel.send_message(ctx.http(), builder).await?,
                    ),
                    _ => {
                        return Ok(Some(json!({
                            "message_sent": false,
                            "reason": "unknown error"
                        })));
                    }
                };

                Ok(Some(json!({
                    "message_sent": true,
                    "channel_kind": channel_kind,
                    "sent_message_id": message.id.to_string()
                })))
            })
            .build();

            let channel = ctx
                .get_operating_channel()
                .await
                .and_then(|c| c.guild().ok_or("not in a guild".into()))?;

            let approval_message = approval.to_message();
            let approval_id = approval.id.clone();

            ctx.agent_context.approval_manager.register(approval);
            channel.send_message(ctx.http(), approval_message).await?;

            Ok(json!({
                "awaiting_approval": true,
                "approval_id": approval_id,
                "note": "cross channel posting requires approval"
            }))
        } else {
            let (channel_kind, message) = match channel {
                Channel::Guild(channel) => {
                    ("guild", channel.send_message(ctx.http(), builder).await?)
                }
                Channel::Private(channel) => (
                    "direct_messages",
                    channel.send_message(ctx.http(), builder).await?,
                ),
                _ => {
                    return Ok(json!({
                        "message_sent": false,
                        "reason": "unknown error"
                    }));
                }
            };

            Ok(json!({
                "message_sent": true,
                "channel_kind": channel_kind,
                "sent_message_id": message.id.to_string()
            }))
        }
    }
}
