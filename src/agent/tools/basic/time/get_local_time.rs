use chrono::Local;
use serde_json::{Value, json};

use crate::agent::tools::{EmptyParameters, basic::BasicTool};

pub struct GetLocalTime;

impl BasicTool for GetLocalTime {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "time.timestamp_to_local"
    }

    fn description(&self) -> &'static str {
        "Converts a UNIX timestamp to a local DateTime string."
    }

    async fn execute(&self, _parameters: Self::Params) -> crate::agent::Result<Self::Returns> {
        let datetime = Local::now();

        Ok(json!({
            "iso": datetime.to_rfc3339(),
            "unix": datetime.timestamp(),
        }))
    }
}
