use super::{defaults::*, environment::Environment};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use tracing::{error, warn};
use serde::Deserialize;
use crate::db::Database;
use crate::error::{ApiError, ErrorCode, ErrorContext, Result};

#[derive(Debug, Clone, Deserialize)]
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
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
}

#[derive(Debug, Clone)]
struct Services {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Config {
    pub fn load() -> std::io::Result<Self> {
        if dotenv().is_err() {
            warn!("No .env file found - using environment variables");
        }
        let mut config = Self::load_from_env()?;

        let _guard = super::sentry::init(&config.sentry_dsn, &config.environment);
        
        let services = Services {
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
