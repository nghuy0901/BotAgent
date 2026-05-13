use std::sync::Arc;

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
        "Send a message to a channel. Use this for most responses. \
         Sending to a different channel than the current one requires approval."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let Ok(channel_id) = params.channel_id.parse::<ChannelId>() else {
            return Ok(json!({
                "error": "unable to parse channel_id"
            }));
        };

        let Ok(channel) = channel_id.to_channel(ctx.http()).await else {
            return Ok(json!({
                "error": "unknown channel id"
            }));
        };

        let builder = CreateMessage::new().content(&params.content);

        if channel.id() != ctx.channel_id {
            // we should attach it as a file if it reaches more than x lines to not clutter the channel
            let approval = ApprovalBuilder::new(
                "post a message in a different channel",
                NeededPermission::InChannel(channel_id, Permissions::SEND_MESSAGES),
            )
            .param_field("Content", params.content)
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

            Ok(json!({
                "awaiting_approval": true,
                "approval_id": approval_id,
                "note": "cross channel posting requires approval"
            }))
        } else {
            send_to_channel(channel, builder, &ctx).await
        }
    }
}

async fn send_to_channel(
    channel: Channel,
    builder: CreateMessage,
    ctx: &Arc<DedicatedContext>,
) -> Result<Value> {
    let (channel_kind, message) = match channel {
        Channel::Guild(channel) => ("guild", channel.send_message(ctx.http(), builder).await?),
        Channel::Private(channel) => (
            "direct_messages",
            channel.send_message(ctx.http(), builder).await?,
        ),
        _ => {
            return Ok(json!({
                "error": "unsupported channel kind"
            }));
        }
    };

    Ok(json!({
        "channel_kind": channel_kind,
        "sent_message_id": message.id.to_string()
    }))
}
