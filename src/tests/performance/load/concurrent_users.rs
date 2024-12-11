use std::time::Instant;
use fake::{Faker, Fake};
use uuid::Uuid;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, connection::SimpleConnection};
use crate::{
    tests::common::helpers::TestDb,
    db::{
        schema::organizations,
        models::organization::Organization,
    },
    error::{DatabaseError, ApiError},
};
use super::super::{CONCURRENT_USERS, PERFORMANCE_THRESHOLD_MS};

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
    let org_id = org.id;
    TestDb::run_test(|conn| {
        Box::pin(async move {
            diesel::insert_into(organizations::table)
                .values(&org)
                .execute(conn)
                .map_err(|e| ApiError::from(DatabaseError::from(e)))
        })
    }).await.expect("Failed to insert test data");

    let start = Instant::now();
    let mut handles = vec![];

    // Spawn concurrent read operations
    for _ in 0..CONCURRENT_USERS {
        let org_id = org_id;
        let handle = tokio::spawn(async move {
            TestDb::run_test(|conn| {
                Box::pin(async move {
                    organizations::table
                        .filter(organizations::id.eq(org_id))
                        .first::<Organization>(conn)
                        .map_err(|e| ApiError::from(DatabaseError::from(e)))
                })
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