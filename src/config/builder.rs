use super::{database::Database, defaults::*, environment::Environment, logger::Logger};
use crate::errors::{AppError, AppResult};
use ::sentry::ClientInitGuard as SentryGuard;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
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
    #[serde(skip)]
    _services: Option<Services>,
}

struct Services {
    _sentry_guard: Option<SentryGuard>,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Config {
    pub fn load() -> std::io::Result<Self> {
        let mut config = Self::load_from_env()?;

        // Initialize services
        let services = Services {
            _sentry_guard: super::sentry::init(&config.sentry_dsn, &config.environment),
            pool: Database::create_pool(&config.database_url)?,
        };

        // Initialize logger last so it can log service initialization
        Logger::init(&config.log_level);

        config._services = Some(services);
        Ok(config)
    }

    fn load_from_env() -> std::io::Result<Self> {
        Self::from_env().map_err(|e| {
            error!("Configuration error: {}", e);
            if let AppError::Configuration(_msg) = &e {
                error!("Please check your environment variables or .env file");
            }
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })
    }

    fn from_env() -> AppResult<Self> {
        envy::from_env()
            .map_err(|error| AppError::Configuration(format!("Configuration error: {}", error)))
    }

    pub fn pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        &self
            ._services
            .as_ref()
            .expect("Services not initialized")
            .pool
    }
}
