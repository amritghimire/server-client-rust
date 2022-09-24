use config::{Config, ConfigError, Environment, File};

use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}


#[derive(Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}


#[derive(Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub log_level: String,
    pub addr: String,
    pub port: u16,
    pub static_dir: String
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        Self::load(run_mode)
    }

    pub fn load(run_mode: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }

    pub fn test() -> Result<Self, ConfigError> {
        Self::load("test".to_string())
    }
}

