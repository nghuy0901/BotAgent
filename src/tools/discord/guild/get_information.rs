use std::sync::Arc;

use serde_json::{Value, json};

use crate::{
    Result,
    agent::context::DedicatedContext,
    tools::{Tool, parameters::EmptyParameters},
};

pub struct GetGuildInformationTool;

impl Tool for GetGuildInformationTool {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "guild.get_information"
    }

    fn description(&self) -> &'static str {
        "Gets detailed information about the Discord guild you're in."
    }

    async fn execute(
        &self,
        _params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        if let Some(guild) = ctx.get_guild() {
            Ok(json!({
                "id": guild.id.to_string(),
                "name": guild.name,
                "owner": guild.owner_id.to_user_cached(ctx.cache()).map(|c| json!({
                    "user_id": guild.owner_id.to_string(),
                    "user_name": c.name,
                    "display_name": c.display_name(),
                })).unwrap_or(json!({
                    "user_id": guild.owner_id.to_string()
                })),
                "member_count": guild.member_count
            }))
        } else {
            Ok(json!({
                "error": "guild not found or not in a guild"
            }))
        }
    }
}
