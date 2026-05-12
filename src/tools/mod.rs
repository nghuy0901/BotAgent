use std::{pin::Pin, sync::Arc};

use serde_json::Value;

use crate::{Result, agent::context::DedicatedContext, tools::parameters::Parameters};

pub mod basic;
pub mod container;
pub mod discord;
pub mod domain;
pub mod parameters;

pub trait Tool: Send + Sync {
    type Params: Parameters;
    type Returns: ToString;

    fn tool_name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn execute(
        &self,
        params: Self::Params,
        ctx: Arc<DedicatedContext>,
    ) -> impl Future<Output = Result<Self::Returns>> + Send;
}

pub trait ToolHolder: Send + Sync {
    fn execute(
        &self,
        params: Value,
        ctx: Arc<DedicatedContext>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>>;
}

impl<T: Tool> ToolHolder for T {
    fn execute(
        &self,
        params: Value,
        ctx: Arc<DedicatedContext>,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>> {
        Box::pin(async move {
            let param = serde_json::from_value(params)?;

            T::execute(self, param, ctx).await.map(|v| v.to_string())
        })
    }
}
