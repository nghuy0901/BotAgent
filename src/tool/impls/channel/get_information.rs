use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{Channel, ChannelId, ChannelType};

use crate::{
    agent::context::DedicatedContext,
    tool::{Status, Tool, ToolError, ToolResult},
    util::channel_kind_to_value,
};

#[derive(Deserialize, JsonSchema)]
pub struct Arguments {
    #[schemars(description = "The ID of the channel.")]
    pub channel_id: String,
    #[schemars(
        description = "Enabling this will fetch the vc user count and user information (no effect on text channels)."
    )]
    pub fetch_vc_users: bool,
}

// i feel like this thing is kinda heavy performance wise
pub struct GetChannelInformationTool;

impl Tool for GetChannelInformationTool {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "get_channel_information"
    }

    fn description(&self) -> &'static str {
        "Retrieve information about a channel by ID. \
         Use fetch_vc_users only for voice/stage channels when presence info is needed, \
         because it requires guild cache and triggers one HTTP call per user."
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

        let channel = channel_id.to_channel(ctx.http()).await?;

        match &channel {
            Channel::Guild(guild_channel) => {
                let mut obj = json!({
                    "id": args.channel_id,
                    "environment": "guild",
                    "guild_id": guild_channel.guild_id.to_string(),
                    "name": guild_channel.name,
                    "kind": channel_kind_to_value(guild_channel.kind),
                });

                if let Some(topic) = &guild_channel.topic {
                    obj["description"] = json!(topic);
                }

                if guild_channel.kind == ChannelType::Voice
                    || guild_channel.kind == ChannelType::Stage
                {
                    if args.fetch_vc_users {
                        let vc_user_ids = ctx.cache().guild(guild_channel.guild_id).map(|guild| {
                            guild
                                .voice_states
                                .values()
                                .filter(|vs| vs.channel_id == Some(guild_channel.id))
                                .map(|vs| vs.user_id)
                                .collect::<Vec<_>>()
                        });

                        match vc_user_ids {
                            Some(ids) => {
                                obj["vc_user_count"] = json!(ids.len());

                                let mut users = Vec::new();

                                for user_id in ids {
                                    match guild_channel.guild_id.member(ctx.http(), user_id).await {
                                        Ok(member) => users.push(json!({
                                            "id": member.user.id.to_string(),
                                            "user_name": member.user.name,
                                            "display_name": member.display_name()
                                        })),
                                        Err(err) => {
                                            tracing::error!("failed to fetch member: {:?}", err)
                                        }
                                    }
                                }

                                obj["vc_users"] = json!(users);
                            }
                            None => {
                                obj["vc_users"] = json!("unavailable");
                                obj["vc_user_count"] = json!("unavailable");
                            }
                        }
                    } else {
                        obj["vc_users"] = json!("not requested");
                        obj["vc_user_count"] = json!("not requested");
                    }

                    obj["vc_user_limit"] = json!(guild_channel.user_limit);
                }

                if let Some(parent_id) = guild_channel.parent_id {
                    obj["category"] = match parent_id.to_channel(ctx.http()).await {
                        Ok(category) => {
                            let category = category.category();

                            if let Some(category) = category {
                                json!({
                                    "id": parent_id.to_string(),
                                    "name": category.name,
                                })
                            } else {
                                tracing::error!(
                                    "unable to fetch category with id {}: got a category, but it isn't one?",
                                    parent_id
                                );

                                json!({
                                    "id": parent_id.to_string(),
                                    "note": "unable to fetch other details"
                                })
                            }
                        }
                        Err(err) => {
                            tracing::error!(
                                "unable to fetch category with id {}: {:?}",
                                parent_id,
                                err
                            );

                            json!({
                                "id": parent_id.to_string(),
                                "note": "unable to fetch other details"
                            })
                        }
                    };
                }

                Ok(Status::success(obj))
            }
            Channel::Private(private) => {
                let recipient = &private.recipient;

                Ok(Status::success(json!({
                    "id": args.channel_id,
                    "environment": "direct_message",
                    "kind": channel_kind_to_value(private.kind),
                    "recipient": {
                        "id": recipient.id.to_string(),
                        "user_name": recipient.name,
                        "display_name": recipient.display_name()
                    }
                })))
            }
            _ => Err(ToolError::custom("unknown channel type")),
        }
    }
}
