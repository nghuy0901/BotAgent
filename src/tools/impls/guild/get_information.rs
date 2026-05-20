use std::sync::Arc;

use serde_json::{Value, json};

use crate::{
    agent::context::DedicatedContext,
    tools::{Status, Tool, ToolError, ToolResult, parameters::EmptyParameters},
};

pub struct GetGuildInformationTool;

impl Tool for GetGuildInformationTool {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "get_guild_information"
    }

    fn description(&self) -> &'static str {
        "Retrieve detailed information about the Discord guild you're operating in. \
        Includes guild details, owner information and approximate member count."
    }

    async fn execute(
        &self,
        _params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        if let Some(guild) = ctx.fetch_guild().await? {
            Ok(Status::success(json!({
                "id": guild.id.to_string(),
                "name": guild.name,
                "owner": match guild.owner_id.to_user(ctx.http()).await {
                    Ok(user) => json!({
                        "user_id": guild.owner_id.to_string(),
                        "user_name": user.name,
                        "display_name": user.display_name(),
                    }),
                    Err(_) => json!({ "user_id": guild.owner_id.to_string() }),
                },
                "approx_member_count": guild.approximate_member_count
            })))
        } else {
            Err(ToolError::custom("guild not found or not in a guild"))
        }
    }
}
