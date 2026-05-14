use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

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
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub debounce_ms: u64,
}

pub fn load() -> Result<Configuration, figment::Error> {
    Figment::new()
        .merge(Toml::file("sweep.default.toml"))
        .merge(Toml::file("sweep.toml"))
        .merge(Env::prefixed("SWEEP_"))
        .extract()
}
