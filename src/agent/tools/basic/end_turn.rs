use serde_json::{Value, json};

use crate::agent::{
    Result,
    tools::{EmptyParameters, basic::BasicTool},
};

pub struct EndTurnTool;

impl BasicTool for EndTurnTool {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "end_turn"
    }

    fn description(&self) -> &'static str {
        "Call this when you have completed all actions and have nothing more to do."
    }

    async fn execute(&self, _parameters: Self::Params) -> Result<Self::Returns> {
        Ok(json!({
            "ended": true
        }))
    }
}
