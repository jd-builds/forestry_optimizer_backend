//! Database connection management
//! 
//! This module provides functionality for managing database connections
//! and connection pools. It handles connection pool configuration,
//! creation, and error handling.

use crate::error::{ApiError, ErrorCode, ErrorContext, Result};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use tracing::{error, debug};
use std::time::Duration;

/// Type alias for the database connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Configuration for the database connection pool
/// 
/// This struct contains all the configuration options for
/// the database connection pool, including connection limits
/// and timeouts.
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Maximum number of connections in the pool
    pub max_size: u32,
    
    /// Minimum number of idle connections maintained in the pool
    pub min_idle: Option<u32>,
    
    /// Maximum lifetime of a connection in the pool
    pub max_lifetime: Option<Duration>,
    
    /// Maximum time a connection can remain idle before being closed
    pub idle_timeout: Option<Duration>,
    
    /// Maximum time to wait for a connection from the pool
    pub connection_timeout: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: Some(5),
            max_lifetime: Some(Duration::from_secs(30 * 60)), // 30 minutes
            idle_timeout: Some(Duration::from_secs(10 * 60)), // 10 minutes
            connection_timeout: Duration::from_secs(30),      // 30 seconds
        }
    }
}

/// Database connection management
pub struct Database;

impl Database {
    /// Creates a new database connection pool with default configuration
    /// 
    /// # Arguments
    /// 
    /// * `database_url` - The URL of the database to connect to
    /// 
    /// # Returns
    /// 
    /// Returns a configured connection pool or an error if the pool
    /// cannot be created
    pub fn create_pool(database_url: &str) -> std::io::Result<DbPool> {
        create_connection_pool(database_url, DbConfig::default()).map_err(|e| {
            error!("Failed to create database pool: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })
    }
}

/// Creates a new database connection pool with custom configuration
/// 
/// # Arguments
/// 
/// * `database_url` - The URL of the database to connect to
/// * `config` - Configuration options for the connection pool
/// 
/// # Returns
/// 
/// Returns a configured connection pool or an error if the pool
/// cannot be created
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

/// Gets a connection from the connection pool
/// 
/// # Arguments
/// 
/// * `pool` - The connection pool to get a connection from
/// 
/// # Returns
/// 
/// Returns a pooled connection or an error if no connection
/// could be acquired
pub fn get_connection(
    pool: &DbPool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DbConfig::default();
        assert_eq!(config.max_size, 10);
        assert_eq!(config.min_idle, Some(5));
        assert_eq!(config.max_lifetime, Some(Duration::from_secs(30 * 60)));
        assert_eq!(config.idle_timeout, Some(Duration::from_secs(10 * 60)));
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
    }

    // Add more tests as needed...
}
