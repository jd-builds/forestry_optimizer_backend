use crate::errors::{AppError, AppResult};
use actix_web::web;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use tracing::error;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConfig {
    pub max_size: u32,
    pub min_idle: Option<u32>,
    pub max_lifetime: Option<std::time::Duration>,
    pub idle_timeout: Option<std::time::Duration>,
    pub connection_timeout: std::time::Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: Some(5),
            max_lifetime: Some(std::time::Duration::from_secs(30 * 60)),
            idle_timeout: Some(std::time::Duration::from_secs(10 * 60)),
            connection_timeout: std::time::Duration::from_secs(30),
        }
    }
}

pub fn create_connection_pool(database_url: &str, config: DbConfig) -> AppResult<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(config.max_size)
        .min_idle(config.min_idle)
        .max_lifetime(config.max_lifetime)
        .idle_timeout(config.idle_timeout)
        .connection_timeout(config.connection_timeout)
        .build(manager)
        .map_err(Into::into)
}

pub fn get_connection(
    pool: &web::Data<DbPool>,
) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, AppError> {
    pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        AppError::Database(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(format!("Connection pool error: {}", e)),
        ))
    })
}
