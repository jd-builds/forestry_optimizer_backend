use crate::db::{create_connection_pool, DbConfig};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::io;
use tracing::error;

pub struct Database;

impl Database {
    pub fn create_pool(database_url: &str) -> io::Result<Pool<ConnectionManager<PgConnection>>> {
        create_connection_pool(database_url, DbConfig::default()).map_err(|e| {
            error!("Failed to create database pool: {}", e);
            io::Error::new(io::ErrorKind::Other, e)
        })
    }
}
