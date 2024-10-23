use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use crate::error::AppResult;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool(database_url: &str) -> AppResult<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).map_err(Into::into)
}
