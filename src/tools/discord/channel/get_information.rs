use std::{str::FromStr, sync::Arc};

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::{Channel, ChannelId, ChannelType};
use tracing::error;

use crate::{Result, agent::context::DedicatedContext, tools::Tool};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
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
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "channel.get_information"
    }

    fn description(&self) -> &'static str {
        "Use this tool to retrieve detailed information about a specific channel."
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

        Ok(match &channel {
            Channel::Guild(guild_channel) => {
                let mut obj = json!({
                    "id": params.channel_id,
                    "environment": "guild",
                    "guild_id": guild_channel.guild_id.to_string(),
                    "name": guild_channel.name,
                    "kind": kind_to_value(guild_channel.kind),
                });

                if let Some(topic) = &guild_channel.topic {
                    obj["description"] = json!(topic);
                }

                if guild_channel.kind == ChannelType::Voice
                    || guild_channel.kind == ChannelType::Stage
                {
                    // doing this so the vc user count fetching doesnt fail
                    if ctx.cache().guild(guild_channel.guild_id).is_none() {
                        let _ = ctx.http().get_guild(guild_channel.guild_id).await;
                    }

                    if params.fetch_vc_users {
                        let vc_users = if let Some(guild) = guild_channel.guild(ctx.cache()) {
                            guild
                                .voice_states
                                .values()
                                .filter(|vs| vs.channel_id == Some(guild_channel.id))
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        };

                        obj["vc_user_count"] = json!(vc_users.len());

                        let mut users = Vec::new();

                        for vs in vc_users {
                            match guild_channel.guild_id.member(ctx.http(), vs.user_id).await {
                                Ok(member) => {
                                    users.push(json!({
                                        "id": member.user.id.to_string(),
                                        "user_name": member.user.name,
                                        "display_name": member.display_name()
                                    }));
                                }
                                Err(err) => {
                                    error!("failed to fetch member: {:?}", err);
                                }
                            }
                        }

                        obj["vc_users"] = json!(users);
                    } else {
                        obj["vc_users"] = json!("not queried");
                        obj["vc_user_count"] = json!("not queried");
                    }

                    obj["vc_user_limit"] = json!(
                        guild_channel
                            .user_limit
                            .map(|l| l.to_string())
                            .unwrap_or("-1".to_string())
                    );

                    obj["vc_user_count"] = json!(if params.fetch_vc_users
                        && let Some(guild) = guild_channel.guild(ctx.cache())
                    {
                        guild
                            .voice_states
                            .values()
                            .filter(|vs| vs.channel_id == Some(guild_channel.id))
                            .count()
                            .to_string()
                    } else {
                        "not queried".to_string()
                    });
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
                                error!(
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
                            error!("unable to fetch category with id {}: {:?}", parent_id, err);

                            json!({
                                "id": parent_id.to_string(),
                                "note": "unable to fetch other details"
                            })
                        }
                    };
                }

                obj
            }
            Channel::Private(private) => {
                let recipient = &private.recipient;

                json!({
                    "id": params.channel_id,
                    "environment": "direct_message",
                    "kind": kind_to_value(private.kind),
                    "recipient": {
                        "id": recipient.id.to_string(),
                        "user_name": recipient.name,
                        "display_name": recipient.display_name()
                    }
                })
            }
            _ => {
                return Ok(json!({
                    "error": "unknown channel type"
                }));
            }
        })
    }
}

fn kind_to_value(kind: ChannelType) -> Value {
    json!(match kind {
        ChannelType::Text => "text",
        ChannelType::Category => "category",
        ChannelType::Voice => "voice_chat",
        ChannelType::Stage => "stage",
        _ => "unknown",
    })
}
