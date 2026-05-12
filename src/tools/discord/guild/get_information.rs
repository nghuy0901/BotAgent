use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use serenity::all::GuildId;

use crate::{Result, agent::context::DedicatedContext, tools::Tool};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The ID of the guild.")]
    pub guild_id: String,
}

pub struct GetGuildInformationTool;

impl Tool for GetGuildInformationTool {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "guild.get_information"
    }

    fn description(&self) -> &'static str {
        "Gets detailed information about a Discord guild."
    }

    async fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        if let Some(guild) = ctx.cache().guild(params.guild_id.parse::<GuildId>()?) {
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
                "error": "guild not found"
            }))
        }
    }
}
