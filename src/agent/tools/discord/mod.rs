pub mod channel;

use std::{pin::Pin, sync::Arc};

use serde_json::Value;
use serenity::all::{Cache, Http};

use crate::agent::tools::{Parameters, Result};

// We ignore "unused" lints, because cache will be used in the future
#[allow(unused)]
pub struct DiscordContext {
    pub http: Arc<Http>,
    pub cache: Arc<Cache>,
}

pub trait DiscordTool: Send + Sync {
    type Params: Parameters;
    type Returns: ToString;

    fn tool_name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn execute(
        &self,
        params: Self::Params,
        ctx: DiscordContext,
    ) -> impl Future<Output = Result<Self::Returns>> + Send;
}

pub trait DiscordToolHolder: Send + Sync {
    fn execute(
        &self,
        params: Value,
        ctx: DiscordContext,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>>;
}

impl<T: DiscordTool> DiscordToolHolder for T {
    fn execute(
        &self,
        params: Value,
        ctx: DiscordContext,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + '_ + Send>> {
        Box::pin(async move {
            let param = serde_json::from_value(params)?;

            T::execute(self, param, ctx).await.map(|v| v.to_string())
        })
    }
}
