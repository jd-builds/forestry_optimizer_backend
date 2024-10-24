use crate::error::{AppError, AppResult};
use serde::Deserialize;
use std::env;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub environment: Environment,
    pub log_level: String,
    pub sentry_dsn: Option<String>,
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, PartialEq)]
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

impl Config {
    pub fn load() -> AppResult<Self> {
        let environment = match env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            _ => Environment::Development,
        };

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| {
            match environment {
                Environment::Production => "info",
                Environment::Staging => "debug",
                Environment::Development => "debug",
            }
            .to_string()
        });

        let sentry_dsn = if environment != Environment::Development {
            Some(env::var("SENTRY_DSN").ok())
        } else {
            None
        }
        .flatten();

        Ok(Config {
            environment,
            log_level,
            sentry_dsn,
            database_url: env::var("DATABASE_URL")?,
            host: env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("API_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| AppError::Validation("API_PORT must be a number".to_string()))?,
        })
    }
}
