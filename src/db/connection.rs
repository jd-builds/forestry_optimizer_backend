use crate::errors::{ApiError, ErrorCode, ErrorContext, Result};
use actix_web::web;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use tracing::{error, debug};
use std::time::Duration;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub max_size: u32,
    pub min_idle: Option<u32>,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub connection_timeout: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: Some(5),
            max_lifetime: Some(Duration::from_secs(30 * 60)),
            idle_timeout: Some(Duration::from_secs(10 * 60)),
            connection_timeout: Duration::from_secs(30),
        }
    }
}

pub fn create_connection_pool(database_url: &str, config: DbConfig) -> Result<DbPool> {
    debug!("Creating database connection pool");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .max_size(config.max_size)
        .min_idle(config.min_idle)
        .max_lifetime(config.max_lifetime)
        .idle_timeout(config.idle_timeout)
        .connection_timeout(config.connection_timeout)
        .build(manager)
        .map_err(|e| {
            error!(error = %e, "Failed to create database connection pool");
            ApiError::new(
                ErrorCode::ConnectionPoolError,
                "Failed to create database connection pool",
                ErrorContext::new().with_details(serde_json::json!({
                    "error": e.to_string()
                }))
            )
        })
}

pub fn get_connection(
    pool: &web::Data<DbPool>,
) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
    pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        ApiError::new(
            ErrorCode::DatabaseError,
            "Failed to get database connection from pool",
            ErrorContext::new().with_details(serde_json::json!({
                "error": e.to_string()
            }))
        )
    })
}
