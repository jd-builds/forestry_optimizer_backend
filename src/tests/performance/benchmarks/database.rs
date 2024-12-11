use std::time::Instant;
use fake::{Faker, Fake};
use uuid::Uuid;
use diesel::{RunQueryDsl, connection::SimpleConnection};
use crate::{
    tests::common::helpers::TestDb,
    db::{
        schema::organizations,
        models::organization::Organization,
    },
    error::{DatabaseError, ApiError},
};
use super::super::{BATCH_SIZE, PERFORMANCE_THRESHOLD_MS};

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
        Box::pin(async move {
            diesel::insert_into(organizations::table)
                .values(&organizations)
                .execute(conn)
                .map_err(|e| {
                    println!("Database error: {:?}", e);
                    ApiError::from(DatabaseError::from(e))
                })
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