use diesel::{PgConnection, Connection};
use once_cell::sync::Lazy;
use std::{thread, time::Duration};
use crate::error::{Result, ApiError, common::DatabaseError};
use diesel::prelude::*;
use crate::db::schema::{users, organizations};
use futures::executor;
use std::future::Future;
use std::pin::Pin;

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
    /// Creates a new database connection for testing with retries
    pub fn conn() -> PgConnection {
        let max_retries = 5;
        let mut retry_count = 0;
        let mut last_error = None;

        while retry_count < max_retries {
            match PgConnection::establish(&TEST_CONFIG.database_url) {
                Ok(conn) => return conn,
                Err(e) => {
                    last_error = Some(e);
                    retry_count += 1;
                    if retry_count < max_retries {
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }

        panic!("Failed to connect to test database after {} retries: {:?}", max_retries, last_error);
    }

    /// Wraps a test in a transaction that gets rolled back
    pub async fn run_test<F, T>(f: F) -> Result<T>
    where
        F: for<'c> FnOnce(&'c mut PgConnection) -> Pin<Box<dyn Future<Output = Result<T>> + Send + 'c>>,
    {
        let mut conn = Self::conn();
        conn.transaction(|conn| {
            let result = executor::block_on(f(conn));
            match result {
                Ok(value) => Ok(value),
                Err(_e) => Err(diesel::result::Error::RollbackTransaction),
            }
        })
        .map_err(DatabaseError::from)
        .map_err(ApiError::from)
    }
}

pub async fn cleanup_test_data(conn: &mut PgConnection) -> Result<()> {
    // Delete in correct order to respect foreign key constraints
    diesel::delete(users::table).execute(conn)
        .map_err(DatabaseError::from)
        .map_err(ApiError::from)?;
    diesel::delete(organizations::table).execute(conn)
        .map_err(DatabaseError::from)
        .map_err(ApiError::from)?;
    Ok(())
}

pub async fn count_users(conn: &mut PgConnection) -> Result<i64> {
    users::table
        .count()
        .get_result(conn)
        .map_err(DatabaseError::from)
        .map_err(ApiError::from)
}

pub async fn count_organizations(conn: &mut PgConnection) -> Result<i64> {
    organizations::table
        .count()
        .get_result(conn)
        .map_err(DatabaseError::from)
        .map_err(ApiError::from)
}

pub async fn verify_clean_state(conn: &mut PgConnection) -> Result<()> {
    let user_count = count_users(conn).await?;
    let org_count = count_organizations(conn).await?;
    
    assert_eq!(user_count, 0, "Database should have no users");
    assert_eq!(org_count, 0, "Database should have no organizations");
    
    Ok(())
} 