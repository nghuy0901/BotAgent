pub mod finish;

use std::pin::Pin;

use serde_json::Value;

use crate::agent::{Result, tools::Parameters};

pub trait BasicTool: Send + Sync {
    type Params: Parameters;
    type Returns: ToString;

    fn tool_name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn execute(
        &self,
        parameters: Self::Params,
    ) -> impl Future<Output = Result<Self::Returns>> + Send + Sync;
}

pub trait BasicToolHolder: Send + Sync {
    fn execute(
        &self,
        parameters: Value,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send + Sync>>;
}

impl<T: BasicTool> BasicToolHolder for T {
    fn execute(
        &self,
        parameters: Value,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send + Sync>> {
        Box::pin(async move {
            let param = serde_json::from_value(parameters)?;

            T::execute(self, param).await.map(|v| v.to_string())
        })
    }
}
