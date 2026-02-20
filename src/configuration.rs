use std::env;

use config::{Config, Environment, File};

#[derive(serde::Deserialize)]

pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    // detect environment (default = local)
    let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());

    let config_directory = "configuration";

    let settings = Config::builder()
        // load base config
        .add_source(File::with_name(&format!("{}/base", config_directory)))
        // load environment-specific config
        .add_source(File::with_name(&format!(
            "{}/{}",
            config_directory, environment
        )))
        // optionally allow env variables override
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()?;

    settings.try_deserialize()
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
