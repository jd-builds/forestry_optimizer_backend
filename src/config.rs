use crate::error::{AppError, AppResult};
use serde::Deserialize;
use std::env;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_environment")]
    pub environment: Environment,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub sentry_dsn: Option<String>,
    pub database_url: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_log_level() -> String {
    "debug".to_string()
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

impl Config {
    pub fn load() -> AppResult<Self> {
        let config: Config = envy::from_env()
            .map_err(|error| AppError::Configuration(format!("Configuration error: {}", error)))?;

        // Adjust log level based on environment if not explicitly set
        let config = if env::var("LOG_LEVEL").is_err() {
            Config {
                log_level: match config.environment {
                    Environment::Production => "info".to_string(),
                    Environment::Staging => "debug".to_string(),
                    Environment::Development => "debug".to_string(),
                },
                ..config
            }
        } else {
            config
        };

        Ok(config)
    }
}
