use diesel::{PgConnection, Connection};
use once_cell::sync::Lazy;
use crate::error::{Result, ApiError, common::DatabaseError};
use std::future::Future;
use std::pin::Pin;
use diesel::prelude::*;
use crate::db::schema::{users, organizations};

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
        F: for<'c> FnOnce(&'c mut PgConnection) -> Pin<Box<dyn Future<Output = Result<T>> + 'c>>,
    {
        let mut conn = Self::conn();
        conn.transaction(|conn| {
            match futures::executor::block_on(test(conn)) {
                Ok(result) => Ok(result),
                Err(e) => {
                    eprintln!("Test error: {:?}", e);
                    Err(diesel::result::Error::RollbackTransaction)
                }
            }
        }).map_err(DatabaseError::from).map_err(ApiError::from)
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