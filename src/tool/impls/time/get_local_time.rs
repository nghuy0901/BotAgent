use std::sync::Arc;

use chrono::Local;
use serde_json::{Value, json};

use crate::{
    agent::context::DedicatedContext,
    tool::{Status, Tool, ToolResult, arguments::EmptyArguments},
};

pub struct GetLocalTime;

impl Tool for GetLocalTime {
    type Args = EmptyArguments;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "get_local_time"
    }

    fn description(&self) -> &'static str {
        "Retrieve the local time as a UNIX timestamp and ISO 8601 date string."
    }

    async fn execute(
        &self,
        _args: Self::Args,
        _ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let datetime = Local::now();

        Ok(Status::success(json!({
            "iso": datetime.to_rfc3339(),
            "unix": datetime.timestamp(),
        })))
    }
}
