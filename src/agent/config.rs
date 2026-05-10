use std::time::Duration;

use crate::agent::tools::{basic::BasicTool, container::ToolContainer, discord::DiscordTool};

pub struct AgentConfig {
    pub model: String,
    pub tools: ToolContainer,
    pub collect_duration: Duration,
}

impl AgentConfig {
    #[inline]
    pub fn with_model<T: AsRef<str>>(mut self, model: T) -> Self {
        self.model = model.as_ref().to_string();

        self
    }

    #[inline]
    pub fn with_discord_tool<T: DiscordTool + 'static>(mut self, tool: T) -> Self {
        self.tools = self.tools.with_discord_tool(tool);

        self
    }

    #[inline]
    pub fn with_basic_tool<T: BasicTool + 'static>(mut self, tool: T) -> Self {
        self.tools = self.tools.with_basic_tool(tool);

        self
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            collect_duration: Duration::from_millis(1000),
            model: String::default(),
            tools: ToolContainer::default(),
        }
    }
}
