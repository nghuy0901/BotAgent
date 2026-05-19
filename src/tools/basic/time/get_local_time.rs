use std::sync::Arc;

use chrono::Local;
use serde_json::{Value, json};

use crate::{
    agent::context::DedicatedContext,
    tools::{Status, Tool, ToolResult, parameters::EmptyParameters},
};

pub struct GetLocalTime;

impl Tool for GetLocalTime {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "get_local_time"
    }

    fn description(&self) -> &'static str {
        "Gets the local time as a UNIX timestamp and ISO 8601 date string."
    }

    async fn execute(
        &self,
        _params: Self::Params,
        _ctx: Arc<DedicatedContext>,
    ) -> ToolResult<Status<Self::Returns>> {
        let datetime = Local::now();

        Ok(Status::success(json!({
            "iso": datetime.to_rfc3339(),
            "unix": datetime.timestamp(),
        })))
    }
}
