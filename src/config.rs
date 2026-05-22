use std::fmt::Display;

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

use crate::core::access_filter::AccessFilter;

#[derive(Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub approval: ApprovalConfig,
    #[serde(default)]
    pub bot: BotConfig,
    #[serde(default)]
    pub channel: ChannelConfig,

    pub discord: DiscordConfig,
    pub llm: LlmConfig,

    #[serde(default)]
    pub tools: ToolsConfig,
    #[serde(default)]
    pub users: UsersConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkippedCompletionEvent {
    Approved,
    Denied,
    Timeout,
}

impl Display for SkippedCompletionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Approved => "approved",
            Self::Denied => "denied",
            Self::Timeout => "timeout",
        };

        write!(f, "{}", name)
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ApprovalConfig {
    pub timeout: u64,
    pub skip_completion: Vec<SkippedCompletionEvent>,
}

impl Default for ApprovalConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            skip_completion: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct BotConfig {
    pub debounce_ms: u64,
    pub max_turns: usize,
    pub wake_on_mention: bool,
    pub wake_on_mention_notify: bool,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            debounce_ms: 1000,
            max_turns: 10,
            wake_on_mention: true,
            wake_on_mention_notify: true,
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ChannelConfig {
    #[serde(flatten)]
    pub access_filter: AccessFilter<u64>,
    #[serde(rename = "override")]
    pub overrides: Vec<ChannelOverride>,
}

#[derive(Deserialize)]
pub struct ChannelOverride {
    pub id: u64,
    pub enable: bool,
    #[serde(default)]
    pub disable_all_tools: bool,
    #[serde(default)]
    pub disable_tools: Vec<String>,
    #[serde(default)]
    pub enable_tools: Vec<String>,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub token: String,
}

#[derive(Deserialize)]
pub struct LlmConfig {
    pub model: String,
    pub endpoint: String,

    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub org_id: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct ToolsConfig {
    pub disable: Vec<String>,
}

#[derive(Deserialize, Default)]
pub struct UsersConfig {
    #[serde(flatten)]
    pub access_filter: AccessFilter<u64>,
}

// We ignore the warning, because we need the error variant
#[allow(clippy::result_large_err)]
pub fn load() -> figment::Result<Configuration> {
    Figment::new()
        .merge(Toml::file("sweep.toml"))
        .join(Env::prefixed("SWEEP__").split("__"))
        .extract()
}
