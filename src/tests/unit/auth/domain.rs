use chrono::Utc;
use uuid::Uuid;
use diesel::prelude::*;
use tracing::info;
use crate::{
    db::{
        models::{auth::{Role, User}, Organization},
        repositories::{auth::UserRepositoryImpl, OrganizationRepositoryImpl, Repository},
        schema::{users, organizations},
    },
    error::{Result, ApiError},
    tests::{common::helpers::TestDb, setup},
    api::utils::PaginationParams,
};

#[tokio::test]
async fn test_user_crud() -> Result<()> {
    setup();
    TestDb::run_test(|conn| {
        Box::pin(async move {
            // First create an organization since users need an org_id
            let org = Organization {
                id: Uuid::new_v4(),
                name: format!("Test Org {}", Uuid::new_v4()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            
            let org_repo = OrganizationRepositoryImpl;
            let created_org = org_repo.create(conn, &org).await?;

            // Create a test user with unique email and phone
            let uuid = Uuid::new_v4();
            let password = User::hash_password("test_password")?;
            let user = User {
                id: Uuid::new_v4(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                email: format!("test{}@example.com", uuid),
                phone_number: format!("1234567890{}", uuid.simple()),
                password,
                is_supervisor: false,
                org_id: created_org.id,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
                role: Role::Operator,
                email_verified: false,
            };

            let repo = UserRepositoryImpl;

            // Test Create
            let created_user = repo.create(conn, &user).await?;
            assert_eq!(created_user.email, user.email);
            assert_eq!(created_user.first_name, user.first_name);
            assert_eq!(created_user.role, Role::Operator);

            // Test Read
            let found_user = repo.find_by_id(conn, created_user.id).await?;
            assert_eq!(found_user.id, created_user.id);
            assert_eq!(found_user.email, created_user.email);

            // Test Update
            let mut updated_user = created_user.clone();
            updated_user.first_name = "Updated".to_string();
            updated_user.role = Role::Manager;
            
            let updated_user = repo.update(conn, updated_user.id, &updated_user).await?;
            assert_eq!(updated_user.first_name, "Updated");
            assert_eq!(updated_user.role, Role::Manager);

            // Test List with pagination
            let pagination = PaginationParams::default();
            let users = repo.list(conn, &pagination).await?;
            assert!(!users.is_empty());
            assert!(users.iter().any(|u| u.id == created_user.id));

            // Test Soft Delete
            let deleted_user = repo.soft_delete(conn, created_user.id).await?;
            assert!(deleted_user.deleted_at.is_some());

            // Verify deleted user doesn't appear in list
            let users_after_delete = repo.list(conn, &pagination).await?;
            assert!(!users_after_delete.iter().any(|u| u.id == created_user.id));

            Ok(())
        })
    })
    .await
}

#[tokio::test]
async fn test_user_pagination() -> Result<()> {
    setup();
    TestDb::run_test(|conn| {
        Box::pin(async move {
            // Set transaction isolation level to serializable
            diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                .execute(conn)
                .map_err(|e| ApiError::database_error("Failed to set transaction isolation level", Some(serde_json::json!({
                    "error": e.to_string()
                }))))?;

            // Clean up any existing data first
            let users_deleted = diesel::delete(users::table)
                .execute(conn)
                .map_err(|e| ApiError::database_error("Failed to clean up users", Some(serde_json::json!({
                    "error": e.to_string()
                }))))?;
            info!("Deleted {} existing users", users_deleted);

            let orgs_deleted = diesel::delete(organizations::table)
                .execute(conn)
                .map_err(|e| ApiError::database_error("Failed to clean up organizations", Some(serde_json::json!({
                    "error": e.to_string()
                }))))?;
            info!("Deleted {} existing organizations", orgs_deleted);

            // Verify clean state
            let initial_user_count = users::table
                .count()
                .get_result::<i64>(conn)
                .map_err(|e| ApiError::database_error("Failed to count users", Some(serde_json::json!({
                    "error": e.to_string()
                }))))?;
            assert_eq!(initial_user_count, 0, "Should start with 0 users");

            // Create an organization for the test users
            let org = Organization {
                id: Uuid::new_v4(),
                name: format!("Test Org {}", Uuid::new_v4()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let org_repo = OrganizationRepositoryImpl;
            let created_org = org_repo.create(conn, &org).await?;

            // Create multiple test users
            let repo = UserRepositoryImpl;
            let mut created_users = Vec::new();

            // Create exactly 15 users with sequential creation times
            let base_time = Utc::now();
            for i in 0..15 {
                let uuid = Uuid::new_v4();
                // Create users with different timestamps, spaced 1 second apart
                let now = base_time + chrono::Duration::seconds(i);
                let password = User::hash_password("test_password")?;
                let user = User {
                    id: Uuid::new_v4(),
                    first_name: format!("Test{}", i),
                    last_name: "User".to_string(),
                    email: format!("test{}@example.com", uuid),
                    phone_number: format!("1234567890{}", uuid.simple()),
                    password,
                    is_supervisor: false,
                    org_id: created_org.id,
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                    role: Role::Operator,
                    email_verified: false,
                };
                let created_user = repo.create(conn, &user).await?;
                info!("Created user {} with email {} at {}", created_user.id, created_user.email, created_user.created_at);
                created_users.push(created_user);

                // Verify count after each creation
                let current_count = users::table
                    .filter(users::deleted_at.is_null())
                    .count()
                    .get_result::<i64>(conn)
                    .map_err(|e| ApiError::database_error("Failed to count users", Some(serde_json::json!({
                        "error": e.to_string()
                    }))))?;
                info!("Current user count after creating user {}: {}", i, current_count);
                assert_eq!(current_count, i as i64 + 1, "User count should match number of created users");
            }
            info!("Created {} test users", created_users.len());

            // Get all users and log their details
            let all_users = users::table
                .filter(users::deleted_at.is_null())
                .order_by((users::created_at.desc(), users::id.desc()))
                .load::<User>(conn)
                .map_err(|e| ApiError::database_error("Failed to load all users", Some(serde_json::json!({
                    "error": e.to_string()
                }))))?;
            
            info!("All users in database:");
            for user in &all_users {
                info!("User {} (email: {}) created at {}", user.id, user.email, user.created_at);
            }

            // Verify total count
            let total_count = users::table
                .filter(users::deleted_at.is_null())
                .count()
                .get_result::<i64>(conn)
                .map_err(|e| ApiError::database_error("Failed to count users", Some(serde_json::json!({
                    "error": e.to_string()
                }))))?;
            info!("Total users in database: {}", total_count);
            assert_eq!(total_count, 15, "Should have exactly 15 users");

            // Test first page with 10 items
            let first_page = repo.list(conn, &PaginationParams {
                page: 1,
                per_page: 10,
            }).await?;
            info!("First page has {} items", first_page.len());
            assert_eq!(first_page.len(), 10, "First page should have 10 items");

            // Test second page with remaining 5 items
            let second_page = repo.list(conn, &PaginationParams {
                page: 2,
                per_page: 10,
            }).await?;
            info!("Second page has {} items", second_page.len());
            assert_eq!(second_page.len(), 5, "Second page should have 5 items");

            // Print details about both pages
            info!("First page users:");
            for user in &first_page {
                info!("User {} (email: {}) created at {}", user.id, user.email, user.created_at);
            }
            info!("Second page users:");
            for user in &second_page {
                info!("User {} (email: {}) created at {}", user.id, user.email, user.created_at);
            }

            // Verify that all users in first page are newer than all users in second page
            for first_user in &first_page {
                for second_user in &second_page {
                    assert!(
                        first_user.created_at >= second_user.created_at,
                        "First page user {} (created at {}) should be newer than second page user {} (created at {})",
                        first_user.id,
                        first_user.created_at,
                        second_user.id,
                        second_user.created_at
                    );
                }
            }

            // Verify that we got all users without duplicates
            let mut all_page_ids: Vec<_> = first_page.iter().map(|u| u.id).collect();
            all_page_ids.extend(second_page.iter().map(|u| u.id));
            let unique_ids: std::collections::HashSet<_> = all_page_ids.iter().collect();
            assert_eq!(unique_ids.len(), 15, "Should have 15 unique users across both pages");

            Ok(())
        })
    })
    .await
} 