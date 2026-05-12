use std::sync::{Arc, OnceLock};

use async_openai::config::OpenAIConfig;
use dashmap::DashMap;
use serenity::all::{Cache, Channel, ChannelId, Http};

use crate::{
    Result, agent::channel::AgentChannel, approval::manager::ApprovalManager,
    config::Configuration, tools::container::ToolContainer,
};

pub struct AgentContext {
    http_lock: OnceLock<Arc<Http>>,
    cache_lock: OnceLock<Arc<Cache>>,

    pub tool_container: ToolContainer,
    pub configuration: Configuration,
    pub base_client: async_openai::Client<OpenAIConfig>,
    pub approval_manager: ApprovalManager,
    pub agents: DashMap<ChannelId, Arc<AgentChannel>>,
}

impl AgentContext {
    pub fn new(configuration: Configuration, tool_container: ToolContainer) -> Self {
        Self {
            tool_container,
            http_lock: OnceLock::new(),
            cache_lock: OnceLock::new(),
            configuration,
            base_client: async_openai::Client::new(),
            approval_manager: ApprovalManager::default(),
            agents: DashMap::new(),
        }
    }

    pub fn setup(&self, cache: Arc<Cache>, http: Arc<Http>) {
        let _ = self.cache_lock.set(cache);
        let _ = self.http_lock.set(http);
    }

    /// Gets a reference to the Bot's [Cache]. Panics when cache is not initialized, which should never happen.
    pub fn cache(&self) -> &Arc<Cache> {
        self.cache_lock.get().expect("cache not initialized")
    }

    /// Gets a reference to the Bot's [Http]. Panics when http is not initialized, which should never happen.
    pub fn http(&self) -> &Arc<Http> {
        self.http_lock.get().expect("http not initialized")
    }
}

pub struct DedicatedContext {
    pub channel_id: ChannelId,
    pub agent_context: Arc<AgentContext>,
}

impl DedicatedContext {
    pub fn new<T: Into<ChannelId>>(agent_context: Arc<AgentContext>, channel_id: T) -> Self {
        Self {
            channel_id: channel_id.into(),
            agent_context,
        }
    }

    #[inline]
    pub async fn get_operating_channel(&self) -> Result<Channel> {
        Ok(self.channel_id.to_channel(self.http()).await?)
    }

    pub fn config(&self) -> &Configuration {
        &self.agent_context.configuration
    }

    pub fn tools(&self) -> &ToolContainer {
        &self.agent_context.tool_container
    }

    pub fn approval_manager(&self) -> &ApprovalManager {
        &self.agent_context.approval_manager
    }

    /// Gets a reference to the Bot's [Cache]. Panics when cache is not initialized, which should never happen.
    pub fn cache(&self) -> &Arc<Cache> {
        self.agent_context.cache()
    }

    /// Gets a reference to the Bot's [Http]. Panics when http is not initialized, which should never happen.
    pub fn http(&self) -> &Arc<Http> {
        self.agent_context.http()
    }
}
