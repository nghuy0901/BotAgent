use std::{
    ops::Deref,
    sync::{Arc, OnceLock},
};

use dashmap::{DashMap, DashSet};
use openai_dive::v1::{api::Client, resources::chat::ChatCompletionTool};
use serenity::all::{Cache, ChannelId, GuildId, Http, PartialGuild, UserId};

use crate::{
    Result,
    agent::channel::AgentChannel,
    approval::manager::ApprovalManager,
    config::{ChannelOverride, Configuration},
    tool::container::ToolContainer,
};

pub struct AgentContext {
    http_lock: OnceLock<Arc<Http>>,
    cache_lock: OnceLock<Arc<Cache>>,

    pub endpoint: Client,
    pub configuration: Configuration,
    pub tool_container: ToolContainer,
    pub approval_manager: ApprovalManager,
    pub agents: DashMap<ChannelId, Arc<AgentChannel>>,
}

impl AgentContext {
    pub fn new(configuration: Configuration, tool_container: ToolContainer) -> Self {
        let mut client = Client::new(
            configuration
                .llm
                .api_key
                .as_ref()
                .cloned()
                .unwrap_or_default(),
        );

        client.set_base_url(&configuration.llm.endpoint);

        if let Some(project_id) = &configuration.llm.project_id {
            client.set_project(project_id);
        }

        if let Some(org_id) = &configuration.llm.org_id {
            client.set_organization(org_id);
        }

        Self {
            http_lock: OnceLock::new(),
            cache_lock: OnceLock::new(),
            endpoint: client,
            configuration,
            tool_container,
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
    pub tools: Vec<ChatCompletionTool>,

    /// A [DashSet] of users that interacted with Sweep in this context.
    pub participants: DashSet<UserId>,
}

impl Deref for DedicatedContext {
    type Target = AgentContext;

    fn deref(&self) -> &Self::Target {
        self.agent_context.as_ref()
    }
}

impl DedicatedContext {
    /// Creates a new context that is dedicated to specific channel in a guild.
    ///
    /// We already compute the tools here so we can cache them.
    pub fn new<T: Into<ChannelId>>(
        agent_context: Arc<AgentContext>,
        channel_id: T,
        channel_override: Option<&ChannelOverride>,
    ) -> Self {
        let mut query = agent_context.tool_container.query().exclude_if(
            !agent_context.configuration.bot.wake_on_mention,
            "end_conversation",
        );

        if let Some(o) = channel_override
            && o.disable_all_tools
        {
            query = query.exclude_all();
        } else if let Some(o) = channel_override
            && !o.enable_tools.is_empty()
        {
            query = query.exclude_all().include_list(&o.enable_tools);
        } else {
            query = query.exclude_list(agent_context.configuration.tools.disable.clone());

            if let Some(o) = channel_override
                && !o.disable_tools.is_empty()
            {
                query = query.exclude_list(o.disable_tools.clone());
            }
        }

        Self {
            tools: query.run(),
            channel_id: channel_id.into(),
            guild_id: None,
            agent_context,
            participants: DashSet::new(),
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
