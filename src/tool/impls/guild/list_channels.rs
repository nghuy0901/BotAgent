use std::{collections::HashMap, sync::Arc};

use serde_json::{Value, json};
use serenity::all::{ChannelId, ChannelType, GuildChannel};

use crate::{
    agent::context::DedicatedContext,
    tool::{Status, Tool, ToolError, ToolResult, arguments::EmptyArguments},
    util::channel_kind_to_value,
};

pub struct ListGuildChannelsTool;

impl Tool for ListGuildChannelsTool {
    type Args = EmptyArguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "list_channels"
    }

    fn description(&self) -> &'static str {
        "Retrieve all channels in the guild."
    }

    async fn execute(
        &self,
        _args: Self::Args,
        ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let Some(guild_id) = ctx.guild_id else {
            return Err(ToolError::custom("you are not operating in a guild"));
        };

        let channels = guild_id
            .channels(ctx.http())
            .await?
            .into_values()
            .collect::<Vec<_>>();

        let mut categories: HashMap<ChannelId, Vec<Value>> = HashMap::new();

        for channel in &channels {
            if let Some(parent_id) = channel.parent_id {
                categories
                    .entry(parent_id)
                    .and_modify(|e| e.push(channel_to_value(channel)))
                    .or_insert(vec![channel_to_value(channel)]);
            }
        }

        let mut list = vec![];

        for channel in channels {
            // we've already transformed them
            if channel.parent_id.is_some() {
                continue;
            }

            let mut value = channel_to_value(&channel);

            if channel.kind == ChannelType::Category {
                value["channels"] = json!(categories.remove(&channel.id).unwrap_or_default());
            }

            list.push(value);
        }

        Ok(Status::success(json!(list)))
    }
}

fn channel_to_value(channel: &GuildChannel) -> Value {
    let mut value = json!({
        "id": channel.id,
        "kind": channel_kind_to_value(channel.kind),
        "name": channel.name
    });

    if let Some(topic) = &channel.topic {
        value["description"] = json!(topic);
    }

    value
}
