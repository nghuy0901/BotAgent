pub mod channel;
pub mod message;
pub mod start_typing;

use std::{pin::Pin, sync::Arc};

use serde_json::Value;
use serenity::all::{Cache, Channel, ChannelId, Http};

use crate::agent::{Result, approval::manager::ApprovalManager, tools::Parameters};

// We ignore "unused" lints, because cache will be used in the future
#[allow(unused)]
pub struct DiscordContext {
    pub approval_manager: Arc<ApprovalManager>,
    pub operating_channel: u64,
    pub http: Arc<Http>,
    pub cache: Arc<Cache>,
}

impl DiscordContext {
    #[inline]
    pub async fn get_operating_channel(&self) -> Result<Channel> {
        Ok(self
            .http
            .get_channel(ChannelId::new(self.operating_channel))
            .await?)
    }
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
