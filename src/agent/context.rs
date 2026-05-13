use std::{
    ops::Deref,
    sync::{Arc, OnceLock},
};

use async_openai::config::OpenAIConfig;
use dashmap::DashMap;
use serenity::all::{Cache, ChannelId, GuildId, Http, PartialGuild};

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
    pub guild_id: Option<GuildId>,
    pub agent_context: Arc<AgentContext>,
}

impl Deref for DedicatedContext {
    type Target = AgentContext;

    fn deref(&self) -> &Self::Target {
        self.agent_context.as_ref()
    }
}

impl DedicatedContext {
    pub fn new<T: Into<ChannelId>>(agent_context: Arc<AgentContext>, channel_id: T) -> Self {
        Self {
            channel_id: channel_id.into(),
            guild_id: None,
            agent_context,
        }
    }

    pub async fn fetch_guild(&self) -> Result<Option<PartialGuild>> {
        let Some(guild_id) = self.guild_id else {
            return Ok(None);
        };

        Ok(Some(
            guild_id.to_partial_guild_with_counts(self.http()).await?,
        ))
    }
}
