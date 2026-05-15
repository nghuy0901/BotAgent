use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

use crate::Result;

#[derive(Deserialize)]
pub struct Configuration {
    pub discord: DiscordConfig,
    pub llm: LlmConfig,
    pub bot: BotConfig,
    pub tools: ToolsConfig,
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
pub struct BotConfig {
    pub debounce_ms: u64,
    pub typing_indicator: bool,
}

#[derive(Deserialize)]
pub struct ToolsConfig {
    pub disable: Vec<String>,
}

pub fn load() -> Result<Configuration> {
    Ok(Figment::new()
        .merge(Toml::string(include_str!("../sweep.default.toml")))
        .merge(Toml::file("sweep.toml"))
        .merge(Env::prefixed("SWEEP__").split("__"))
        .extract()?)
}
