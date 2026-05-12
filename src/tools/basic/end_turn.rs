use std::sync::Arc;

use serde_json::{Value, json};

use crate::{
    Result,
    agent::context::DedicatedContext,
    tools::{Tool, parameters::EmptyParameters},
};

pub struct EndTurnTool;

impl Tool for EndTurnTool {
    type Params = EmptyParameters;
    type Returns = Value;

    fn tool_name(&self) -> &'static str {
        "end_turn"
    }

    fn description(&self) -> &'static str {
        "Call this when you have completed all actions and have nothing more to do."
    }

    async fn execute(
        &self,
        _params: Self::Params,
        _ctx: Arc<DedicatedContext>,
    ) -> Result<Self::Returns> {
        Ok(json!({
            "ended": true
        }))
    }
}
