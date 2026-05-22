use std::sync::Arc;

use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    agent::context::DedicatedContext,
    tool::{Status, Tool, ToolError, ToolResult},
};

#[derive(Deserialize, JsonSchema)]
pub struct Arguments {
    #[schemars(description = "The UNIX timestamp.")]
    pub timestamp: i64,
}

pub struct TimestampToLocal;

impl Tool for TimestampToLocal {
    type Args = Arguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "timestamp_to_local"
    }

    fn description(&self) -> &'static str {
        "Convert a UNIX timestamp to a local DateTime string."
    }

    async fn execute(
        &self,
        args: Self::Args,
        _ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let datetime = match DateTime::from_timestamp_secs(args.timestamp) {
            Some(d) => d,
            None => {
                return Err(ToolError::validation("timestamp", "out of range"));
            }
        }
        .with_timezone(&Local);

        Ok(Status::success(json!({
            "iso": datetime.to_rfc3339()
        })))
    }
}
