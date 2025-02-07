use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub auth: AuthSettings,
    pub slack: SlackSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisSettings {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub secret_key: String,
    pub token_expiration: u64,
}

#[derive(Debug, Deserialize)]
pub struct SlackSettings {
    pub webhook_url: String,
}

pub fn load_config() -> Result<Settings, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name("src/config/default.toml"))
        .build()?;

    config.try_deserialize()
}