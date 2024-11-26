use super::{database::Database, defaults::*, environment::Environment};
use ::sentry::ClientInitGuard as SentryGuard;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use tracing::{error, warn};
use serde::Deserialize;
use crate::errors::{ApiError, ErrorCode, ErrorContext, Result};

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_environment")]
    pub environment: Environment,
    pub sentry_dsn: Option<String>,
    pub database_url: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(skip)]
    _services: Option<Services>,
}

struct Services {
    _sentry_guard: Option<SentryGuard>,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Config {
    pub fn load() -> std::io::Result<Self> {
        // Load .env file and check if it was found
        if dotenv().is_err() {
            // Log warning but don't fail - env vars might be set directly
            warn!("No .env file found - using environment variables");
        }
        let mut config = Self::load_from_env()?;

        // Initialize services
        let services = Services {
            _sentry_guard: super::sentry::init(&config.sentry_dsn, &config.environment),
            pool: Database::create_pool(&config.database_url)?,
        };

        config._services = Some(services);
        Ok(config)
    }

    fn load_from_env() -> std::io::Result<Self> {
        Self::from_env().map_err(|e| {
            error!("Configuration error: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })
    }

    fn from_env() -> Result<Self> {
        envy::from_env()
            .map_err(|error| ApiError::new(
                ErrorCode::ConfigurationError,
                format!("Configuration error: {}", error),
                ErrorContext::new()
            ))
    }

    pub fn pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        &self
            ._services
            .as_ref()
            .expect("Services not initialized")
            .pool
    }
}
