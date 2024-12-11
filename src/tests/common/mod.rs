//! Common test utilities and helpers
//! This module provides shared functionality for all test types

use actix_web::{
    App,
    dev::{ServiceResponse, ServiceFactory, ServiceRequest},
    body::MessageBody,
};
use diesel::{PgConnection, Connection};
use once_cell::sync::Lazy;

use crate::{
    error::{Result, ApiError},
    error::common::DatabaseError,
};

/// Global test configuration
pub static TEST_CONFIG: Lazy<TestConfig> = Lazy::new(|| {
    dotenv::from_filename(".env.test").ok();
    TestConfig::new()
});

/// Test configuration structure
pub struct TestConfig {
    pub database_url: String,
}

impl TestConfig {
    pub fn new() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in .env.test"),
        }
    }
}

/// Database test utilities
pub struct TestDb;

impl TestDb {
    /// Creates a new database connection for testing
    pub fn conn() -> PgConnection {
        PgConnection::establish(&TEST_CONFIG.database_url)
            .expect("Failed to connect to test database")
    }

    /// Wraps a test in a transaction that gets rolled back
    pub async fn run_test<F, T>(test: F) -> Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T>,
    {
        let mut conn = Self::conn();
        conn.transaction(|conn| {
            match test(conn) {
                Ok(result) => Ok(result),
                Err(_) => Err(diesel::result::Error::RollbackTransaction),
            }
        }).map_err(DatabaseError::from).map_err(ApiError::from)
    }
}

/// Test app builder
pub async fn spawn_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(actix_web::middleware::Logger::default())
}

/// Test assertions
pub mod assertions;
pub mod fixtures;
pub mod helpers;

pub use assertions::*;
pub use fixtures::*;
pub use helpers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_loading() {
        assert!(!TEST_CONFIG.database_url.is_empty());
    }

    #[tokio::test]
    async fn test_fake_data_generation() {
        let user = fake_user();
        assert!(user.get("email").is_some());
        assert!(user.get("first_name").is_some());
        assert!(user.get("last_name").is_some());
        assert!(user.get("phone_number").is_some());
        assert!(user.get("password").is_some());
    }
} 