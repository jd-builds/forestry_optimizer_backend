use super::defaults::{default_environment, default_host, default_log_level, default_port};
use super::environment::Environment;
use crate::errors::{AppError, AppResult};
use serde::Deserialize;
use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        assert_eq!(default_environment(), Environment::Development);
        assert_eq!(default_log_level(), "debug");
        assert_eq!(default_host(), "0.0.0.0");
        assert_eq!(default_port(), 8080);
    }

    // Add more tests for Config::load() if needed
}
