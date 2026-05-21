use std::fmt::Display;

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

use crate::Result;

#[derive(Deserialize)]
pub struct Configuration {
    pub approval: ApprovalConfig,
    pub bot: BotConfig,
    pub discord: DiscordConfig,
    pub llm: LlmConfig,
    pub tools: ToolsConfig,
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
pub struct ApprovalConfig {
    pub timeout: u64,
    pub skip_completion: Vec<SkippedCompletionEvent>,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub debounce_ms: u64,
    pub max_turns: usize,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub token: String,
}

#[derive(Deserialize)]
pub struct LlmConfig {
    pub model: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub project_id: Option<String>,
    pub org_id: Option<String>,
}

#[derive(Deserialize)]
pub struct ToolsConfig {
    pub disable: Vec<String>,
}

#[derive(Deserialize)]
pub struct UsersConfig {
    pub blacklist: Option<Vec<u64>>,
    pub whitelist: Option<Vec<u64>>,
}

pub fn load() -> Result<Configuration> {
    Ok(Figment::new()
        .merge(Toml::string(include_str!("../sweep.default.toml")))
        .merge(Toml::file("sweep.toml"))
        .merge(Env::prefixed("SWEEP__").split("__"))
        .extract()?)
}
