use crate::error::{AppResult, AppError};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use actix_web::web;
use log::error;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool(database_url: &str) -> AppResult<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).map_err(Into::into)
}

pub fn get_connection(pool: &web::Data<DbPool>) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, AppError> {
    pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        AppError::Database(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(format!("Connection pool error: {}", e))
        ))
    })
}
