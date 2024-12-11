//! Performance and benchmark tests
//! Tests for measuring performance characteristics

use crate::{
    tests::common::TestDb,
    db::schema::organizations,
    db::models::organization::Organization,
};
use std::time::Instant;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::connection::SimpleConnection;

#[cfg(test)]
mod tests {
    use fake::{Faker, Fake};
    use uuid::Uuid;
    use crate::{error::DatabaseError, ApiError};
    use super::*;

    const BATCH_SIZE: usize = 100;
    const CONCURRENT_USERS: usize = 10;
    const PERFORMANCE_THRESHOLD_MS: u128 = 1000; // 1 second

    #[tokio::test]
    async fn test_bulk_insert_performance() {
        let start = Instant::now();
        
        // Run migrations first
        let mut conn = TestDb::conn();
        conn.batch_execute("CREATE TABLE IF NOT EXISTS organizations (
            id UUID PRIMARY KEY,
            name VARCHAR NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            deleted_at TIMESTAMP
        )").expect("Failed to create table");
        
        // Generate test data
        let organizations: Vec<Organization> = (0..BATCH_SIZE)
            .map(|_| Organization {
                id: Uuid::new_v4(),
                name: Faker.fake(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                deleted_at: None,
            })
            .collect();

        let result = TestDb::run_test(|conn| {
            diesel::insert_into(organizations::table)
                .values(&organizations)
                .execute(conn)
                .map_err(|e| {
                    println!("Database error: {:?}", e);
                    ApiError::from(DatabaseError::from(e))
                })
        }).await;

        if let Err(e) = &result {
            println!("Test error: {:?}", e);
        }
        assert!(result.is_ok());
        
        let total_duration = start.elapsed();
        assert!(
            total_duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Bulk insert should complete within {}ms (took {}ms)",
            PERFORMANCE_THRESHOLD_MS,
            total_duration.as_millis()
        );
    }

    #[tokio::test]
    async fn test_concurrent_reads() {
        let mut conn = TestDb::conn();
        conn.batch_execute("CREATE TABLE IF NOT EXISTS organizations (
            id UUID PRIMARY KEY,
            name VARCHAR NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            deleted_at TIMESTAMP
        )").expect("Failed to create table");

        let org = Organization {
            id: Uuid::new_v4(),
            name: Faker.fake(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        };

        // Insert test data
        TestDb::run_test(|conn| {
            diesel::insert_into(organizations::table)
                .values(&org)
                .execute(conn)
                .map_err(|e| ApiError::from(DatabaseError::from(e)))
        }).await.expect("Failed to insert test data");

        let start = Instant::now();
        let mut handles = vec![];

        // Spawn concurrent read operations
        for _ in 0..CONCURRENT_USERS {
            let org_id = org.id;
            let handle = tokio::spawn(async move {
                TestDb::run_test(|conn| {
                    organizations::table
                        .filter(organizations::id.eq(org_id))
                        .first::<Organization>(conn)
                        .map_err(|e| ApiError::from(DatabaseError::from(e)))
                }).await
            });
            handles.push(handle);
        }

        // Wait for all reads to complete
        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Concurrent reads should complete within {}ms (took {}ms)",
            PERFORMANCE_THRESHOLD_MS,
            duration.as_millis()
        );
    }
} 