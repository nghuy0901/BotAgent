use std::sync::Arc;

use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Result, agent::context::DedicatedContext, tools::Tool};

#[derive(Deserialize, JsonSchema)]
pub struct Params {
    #[schemars(description = "The UNIX timestamp.")]
    pub timestamp: i64,
}

pub struct TimestampToLocal;

impl Tool for TimestampToLocal {
    type Params = Params;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "time.timestamp_to_local"
    }

    fn description(&self) -> &'static str {
        "Converts a UNIX timestamp to a local DateTime string."
    }

    async fn execute(
        &self,
        parameters: Self::Params,
        _ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        let datetime = match DateTime::from_timestamp_secs(parameters.timestamp) {
            Some(d) => d,
            None => {
                return Ok(json!({
                    "error": "unable to convert timestamp to DateTime"
                }));
            }
        }
        .with_timezone(&Local);

        Ok(json!({
            "iso": datetime.to_rfc3339()
        }))
    }
}
