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

pub fn load() -> Result<Configuration> {
    Ok(Figment::new()
        .merge(Toml::file("sweep.default.toml"))
        .merge(Toml::file("sweep.toml"))
        .merge(Env::prefixed("SWEEP__").split("__"))
        .extract()?)
}
