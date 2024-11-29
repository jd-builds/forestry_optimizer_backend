use std::{sync::Arc, time::Duration};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt, EnvFilter};

use crate::{
    domain::services::{AuthService, OrganizationService},
    application::services::{auth::AuthServiceImpl, organization::OrganizationServiceImpl},
    infrastructure::{
        security::{JwtManager, PasswordHasher},
        database::{
            repositories::{auth::AuthRepositoryImpl, organization::OrganizationRepositoryImpl},
            DbConfig, create_connection_pool,
        },
    },
};

use super::Environment;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;
const DEFAULT_DB_TIMEOUT_SECS: u64 = 30;
const DEFAULT_TOKEN_EXPIRATION_MINS: u64 = 60;
const DEFAULT_RATE_LIMIT_REQUESTS: u32 = 100;
const DEFAULT_RATE_LIMIT_WINDOW_SECS: u32 = 60;
const DEFAULT_CORS_MAX_AGE_SECS: u64 = 3600;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,
    
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(rename = "database_url")]
    pub database_url: String,
    
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_db_max_connections")]
    pub database_max_connections: u32,
    
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_db_timeout")]
    pub database_connection_timeout: u64,
    
    #[serde(rename = "jwt_secret")]
    pub jwt_secret: String,
    
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_token_expiration")]
    pub token_expiration: u64,
    
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
    #[serde(default)]
    pub telemetry: TelemetryConfig,
    #[serde(default)]
    pub cors: CorsConfig,
    #[serde(default)]
    pub environment: Environment,

    #[serde(skip)]
    _services: Option<Services>,
}

#[serde_as]
#[derive(Debug, Deserialize, Default)]
pub struct RateLimitConfig {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_rate_limit_requests")]
    pub max_requests: u32,
    
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "default_rate_limit_window")]
    pub window_size: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct TelemetryConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub sentry_dsn: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct CorsConfig {
    #[serde(default = "default_cors_origins")]
    pub allowed_origins: String,
    #[serde(default = "default_cors_methods")]
    pub allowed_methods: Vec<String>,
    #[serde(default = "default_cors_headers")]
    pub allowed_headers: Vec<String>,
    #[serde(default)]
    pub exposed_headers: Vec<String>,
    #[serde(default = "default_cors_max_age")]
    pub max_age: u64,
}

struct Services {
    _sentry_guard: Option<sentry::ClientInitGuard>,
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    auth_service: Arc<dyn AuthService>,
    org_service: Arc<dyn OrganizationService>,
}

impl std::fmt::Debug for Services {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Services")
            .field("pool", &"<Pool>")
            .field("auth_service", &"<AuthService>")
            .field("org_service", &"<OrganizationService>")
            .finish()
    }
}

impl AppConfig {
    pub fn load() -> std::io::Result<Self> {
        dotenv::dotenv().ok();
        
        debug!("Loading configuration from environment");
        let mut config: Self = envy::from_env().map_err(|e| {
            error!("Configuration error: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        
        debug!("Loaded config: {:?}", config);
        config.init_services()?;
        config.init_tracing();
        
        Ok(config)
    }

    pub fn pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        self._services.as_ref()
            .expect("Services not initialized")
            .pool.as_ref()
    }

    pub fn auth_service(&self) -> Arc<dyn AuthService> {
        self._services.as_ref()
            .expect("Services not initialized")
            .auth_service.clone()
    }

    pub fn org_service(&self) -> Arc<dyn OrganizationService> {
        self._services.as_ref()
            .expect("Services not initialized")
            .org_service.clone()
    }

    fn init_services(&mut self) -> std::io::Result<()> {
        let db_config = DbConfig {
            max_size: self.database_max_connections,
            connection_timeout: Duration::from_secs(self.database_connection_timeout),
            ..Default::default()
        };
        
        let pool = Arc::new(create_connection_pool(&self.database_url, db_config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?);
            
        let jwt_manager = Arc::new(JwtManager::new(self.jwt_secret.as_bytes()));
        let password_hasher = Arc::new(PasswordHasher::new());
        
        let auth_repository = Arc::new(AuthRepositoryImpl::new(pool.clone()));
        let auth_service = Arc::new(AuthServiceImpl::new(
            auth_repository,
            jwt_manager,
            password_hasher,
            pool.clone(),
        ));

        let org_repository = Arc::new(OrganizationRepositoryImpl::new(pool.clone()));
        let org_service = Arc::new(OrganizationServiceImpl::new(org_repository));

        self._services = Some(Services {
            _sentry_guard: self.init_sentry(),
            pool,
            auth_service,
            org_service,
        });
        
        Ok(())
    }

    fn init_sentry(&self) -> Option<sentry::ClientInitGuard> {
        self.telemetry.sentry_dsn.as_ref().map(|dsn| {
            let guard = sentry::init((
                dsn.clone(),
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    environment: Some(self.environment.to_string().into()),
                    ..Default::default()
                },
            ));
            info!("Sentry initialized successfully");
            guard
        })
    }

    fn init_tracing(&self) {
        let env_filter = if self.environment.is_development() {
            "debug"
        } else {
            "info"
        };

        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::new(env_filter))
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .with_ansi(self.environment.is_development())
            .compact()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            warn!("Failed to set tracing (might be already initialized): {}", e);
        }
    }
}

// Default implementations
fn default_host() -> String { DEFAULT_HOST.into() }
fn default_port() -> u16 { DEFAULT_PORT }
fn default_db_max_connections() -> u32 { DEFAULT_DB_MAX_CONNECTIONS }
fn default_db_timeout() -> u64 { DEFAULT_DB_TIMEOUT_SECS }
fn default_token_expiration() -> u64 { DEFAULT_TOKEN_EXPIRATION_MINS }
fn default_rate_limit_requests() -> u32 { DEFAULT_RATE_LIMIT_REQUESTS }
fn default_rate_limit_window() -> u32 { DEFAULT_RATE_LIMIT_WINDOW_SECS }
fn default_log_level() -> String { "info".into() }
fn default_cors_origins() -> String { "*".into() }
fn default_cors_max_age() -> u64 { DEFAULT_CORS_MAX_AGE_SECS }

fn default_cors_methods() -> Vec<String> {
    vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]
        .into_iter()
        .map(String::from)
        .collect()
}

fn default_cors_headers() -> Vec<String> {
    vec!["Content-Type", "Authorization", "Accept"]
        .into_iter()
        .map(String::from)
        .collect()
}