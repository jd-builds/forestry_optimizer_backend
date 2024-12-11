//! Common test utilities and helpers
//! This module provides shared functionality for all test types

use actix_web::{
    App,
    dev::{ServiceResponse, ServiceFactory, ServiceRequest},
    body::MessageBody,
};
use diesel::{PgConnection, Connection};
use fake::{Fake, Faker};
use once_cell::sync::Lazy;
use serde::Serialize;
use uuid::Uuid;

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

/// Authentication test helpers
pub struct TestAuth;

impl TestAuth {
    /// Creates a test JWT token
    pub fn create_test_token(_user_id: Uuid, _role: &str) -> String {
        // TODO: Implement JWT token creation for tests
        "test_token".to_string()
    }
}

/// Test data generators
pub struct TestData;

impl TestData {
    /// Generates fake user data
    pub fn fake_user() -> serde_json::Value {
        let email: String = Faker.fake();
        serde_json::json!({
            "email": email,
            "name": Faker.fake::<String>(),
            "password": "test_password"
        })
    }

    /// Generates fake organization data
    pub fn fake_organization() -> serde_json::Value {
        serde_json::json!({
            "name": Faker.fake::<String>(),
            "description": Faker.fake::<String>()
        })
    }
}

/// Test assertions
pub mod assertions {
    use super::*;
    use actix_web::http::StatusCode;
    use pretty_assertions::assert_eq;

    pub fn assert_success<T: Serialize>(response: &ServiceResponse<impl MessageBody>, _expected_data: &T) {
        assert_eq!(response.status(), StatusCode::OK);
        // TODO: Add more specific assertions
    }

    pub fn assert_error(response: &ServiceResponse<impl MessageBody>, expected_status: StatusCode) {
        assert_eq!(response.status(), expected_status);
        // TODO: Add more specific error assertions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_loading() {
        assert!(!TEST_CONFIG.database_url.is_empty());
    }

    #[tokio::test]
    async fn test_fake_data_generation() {
        let user = TestData::fake_user();
        assert!(user.get("email").is_some());
        assert!(user.get("name").is_some());
    }
} 