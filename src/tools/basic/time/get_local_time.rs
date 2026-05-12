use std::sync::Arc;

use chrono::Local;
use serde_json::{Value, json};

use crate::{
    Result,
    agent::context::DedicatedContext,
    tools::{Tool, parameters::EmptyParameters},
};

pub struct GetLocalTime;

impl Tool for GetLocalTime {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "time.get_local"
    }

    fn description(&self) -> &'static str {
        "Gets the local time as a UNIX timestamp and ISO 8601 date string."
    }

    async fn execute(
        &self,
        _parameters: Self::Params,
        _ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let datetime = Local::now();

        Ok(json!({
            "iso": datetime.to_rfc3339(),
            "unix": datetime.timestamp(),
        }))
    }
}
