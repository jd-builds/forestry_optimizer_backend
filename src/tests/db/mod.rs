//! Database tests
//! This module contains tests for database operations and models

use crate::{
    db::{
        models::{auth::{User, Role}, organization::Organization},
        repositories::{
            auth::UserRepositoryImpl,
            organization::OrganizationRepositoryImpl,
            Repository,
        },
    },
    error::{Result, common::DatabaseError},
    tests::common::TestDb,
    api::utils::PaginationParams,
};
use chrono::Utc;
use uuid::Uuid;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_crud() -> Result<()> {
        // Run test in a transaction
        let test_result = TestDb::run_test(|conn| -> Result<()> {
            // First create an organization since users need an org_id
            let org = Organization {
                id: Uuid::new_v4(),
                name: "Test Org".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            
            let org_repo = OrganizationRepositoryImpl;
            let created_org = futures::executor::block_on(org_repo.create(conn, &org))?;

            // Create a test user
            let user = User {
                id: Uuid::new_v4(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                email: "test@example.com".to_string(),
                phone_number: "1234567890".to_string(),
                password: "hashed_password".to_string(), // In real app, this would be hashed
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
            let created_user = futures::executor::block_on(repo.create(conn, &user))?;
            assert_eq!(created_user.email, user.email);
            assert_eq!(created_user.first_name, user.first_name);
            assert_eq!(created_user.role, Role::Operator);

            // Test Read
            let found_user = futures::executor::block_on(repo.find_by_id(conn, created_user.id))?;
            assert_eq!(found_user.id, created_user.id);
            assert_eq!(found_user.email, created_user.email);

            // Test Update
            let mut updated_user = created_user.clone();
            updated_user.first_name = "Updated".to_string();
            updated_user.role = Role::Manager;
            
            let updated_user = futures::executor::block_on(repo.update(conn, updated_user.id, &updated_user))?;
            assert_eq!(updated_user.first_name, "Updated");
            assert_eq!(updated_user.role, Role::Manager);

            // Test List with pagination
            let pagination = PaginationParams::default();
            let users = futures::executor::block_on(repo.list(conn, &pagination))?;
            assert!(!users.is_empty());
            assert!(users.iter().any(|u| u.id == created_user.id));

            // Test Soft Delete
            let deleted_user = futures::executor::block_on(repo.soft_delete(conn, created_user.id))?;
            assert!(deleted_user.deleted_at.is_some());

            // Verify deleted user doesn't appear in list
            let users_after_delete = futures::executor::block_on(repo.list(conn, &pagination))?;
            assert!(!users_after_delete.iter().any(|u| u.id == created_user.id));

            // Explicitly roll back the transaction to clean up
            Err(DatabaseError::TransactionFailed("Test completed successfully".to_string()).into())
        })
        .await;

        // Handle the expected rollback
        match test_result {
            Ok(_) => panic!("Expected transaction to be rolled back"),
            Err(e) => {
                if let Some(db_error) = e.source().and_then(|e| e.downcast_ref::<DatabaseError>()) {
                    match db_error {
                        DatabaseError::TransactionFailed(_) => Ok(()),
                        _ => Err(e),
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test]
    async fn test_pagination() -> Result<()> {
        let test_result = TestDb::run_test(|conn| -> Result<()> {
            // Create an organization for the test users
            let org = Organization {
                id: Uuid::new_v4(),
                name: "Test Org".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let org_repo = OrganizationRepositoryImpl;
            let created_org = futures::executor::block_on(org_repo.create(conn, &org))?;

            // Create multiple test users
            let repo = UserRepositoryImpl;
            let mut created_users = Vec::new();

            for i in 0..15 {
                let user = User {
                    id: Uuid::new_v4(),
                    first_name: format!("Test{}", i),
                    last_name: "User".to_string(),
                    email: format!("test{}@example.com", i),
                    phone_number: format!("123456789{}", i),
                    password: "hashed_password".to_string(),
                    is_supervisor: false,
                    org_id: created_org.id,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    role: Role::Operator,
                    email_verified: false,
                };
                let created_user = futures::executor::block_on(repo.create(conn, &user))?;
                created_users.push(created_user);
            }

            // Test first page (default pagination)
            let first_page = futures::executor::block_on(repo.list(conn, &PaginationParams::default()))?;
            assert_eq!(first_page.len(), 10); // Default page size should be 10

            // Test second page
            let second_page = futures::executor::block_on(repo.list(conn, &PaginationParams {
                page: 2,
                per_page: 10,
            }))?;
            assert_eq!(second_page.len(), 5); // Should have remaining 5 items

            // Test custom page size
            let small_page = futures::executor::block_on(repo.list(conn, &PaginationParams {
                page: 1,
                per_page: 5,
            }))?;
            assert_eq!(small_page.len(), 5);

            // Test empty page
            let empty_page = futures::executor::block_on(repo.list(conn, &PaginationParams {
                page: 4,
                per_page: 10,
            }))?;
            assert!(empty_page.is_empty());

            // Verify no duplicate users between pages
            let page1 = futures::executor::block_on(repo.list(conn, &PaginationParams {
                page: 1,
                per_page: 7,
            }))?;
            let page2 = futures::executor::block_on(repo.list(conn, &PaginationParams {
                page: 2,
                per_page: 7,
            }))?;

            let page1_ids: std::collections::HashSet<_> = page1.iter().map(|u| u.id).collect();
            let page2_ids: std::collections::HashSet<_> = page2.iter().map(|u| u.id).collect();
            assert!(page1_ids.is_disjoint(&page2_ids));

            // Clean up by rolling back
            Err(DatabaseError::TransactionFailed("Test completed successfully".to_string()).into())
        })
        .await;

        // Handle the expected rollback
        match test_result {
            Ok(_) => panic!("Expected transaction to be rolled back"),
            Err(e) => {
                if let Some(db_error) = e.source().and_then(|e| e.downcast_ref::<DatabaseError>()) {
                    match db_error {
                        DatabaseError::TransactionFailed(_) => Ok(()),
                        _ => Err(e),
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test]
    async fn test_organization_crud() -> Result<()> {
        let test_result = TestDb::run_test(|conn| -> Result<()> {
            let repo = OrganizationRepositoryImpl;

            // Test Create
            let org = Organization {
                id: Uuid::new_v4(),
                name: "Test Organization".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_org = futures::executor::block_on(repo.create(conn, &org))?;
            assert_eq!(created_org.name, org.name);
            assert_eq!(created_org.id, org.id);

            // Test Read
            let found_org = futures::executor::block_on(repo.find_by_id(conn, created_org.id))?;
            assert_eq!(found_org.id, created_org.id);
            assert_eq!(found_org.name, created_org.name);

            // Test Update
            let mut updated_org = created_org.clone();
            updated_org.name = "Updated Organization".to_string();
            let updated_org = futures::executor::block_on(repo.update(conn, updated_org.id, &updated_org))?;
            assert_eq!(updated_org.name, "Updated Organization");

            // Test List
            let orgs = futures::executor::block_on(repo.list(conn, &PaginationParams::new(1, 100)))?;
            assert!(!orgs.is_empty());
            assert!(orgs.iter().any(|o| o.id == created_org.id));

            // Test Soft Delete
            let deleted_org = futures::executor::block_on(repo.soft_delete(conn, created_org.id))?;
            assert!(deleted_org.deleted_at.is_some());

            // Verify deleted org doesn't appear in list
            let orgs_after_delete = futures::executor::block_on(repo.list(conn, &PaginationParams::default()))?;
            assert!(!orgs_after_delete.iter().any(|o| o.id == created_org.id));

            // Verify that users can't be created with deleted org_id
            let user_repo = UserRepositoryImpl;
            let user = User {
                id: Uuid::new_v4(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                email: "test@example.com".to_string(),
                phone_number: "1234567890".to_string(),
                password: "hashed_password".to_string(),
                is_supervisor: false,
                org_id: deleted_org.id,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
                role: Role::Operator,
                email_verified: false,
            };
            let user_result = futures::executor::block_on(user_repo.create(conn, &user));
            assert!(user_result.is_err(), "Should not be able to create user with deleted org_id");

            // Clean up by rolling back
            Err(DatabaseError::TransactionFailed("Test completed successfully".to_string()).into())
        })
        .await;

        // Handle the expected rollback
        match test_result {
            Ok(_) => panic!("Expected transaction to be rolled back"),
            Err(e) => {
                if let Some(db_error) = e.source().and_then(|e| e.downcast_ref::<DatabaseError>()) {
                    match db_error {
                        DatabaseError::TransactionFailed(_) => Ok(()),
                        _ => Err(e),
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test]
    async fn test_soft_delete() -> Result<()> {
        let test_result = TestDb::run_test(|conn| -> Result<()> {
            // Create test organization
            let org_repo = OrganizationRepositoryImpl;
            let org = Organization {
                id: Uuid::new_v4(),
                name: "Test Organization".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_org = futures::executor::block_on(org_repo.create(conn, &org))?;

            // Create test users in the organization
            let user_repo = UserRepositoryImpl;
            let mut users = Vec::new();
            for i in 0..3 {
                let user = User {
                    id: Uuid::new_v4(),
                    first_name: format!("Test{}", i),
                    last_name: "User".to_string(),
                    email: format!("test{}@example.com", i),
                    phone_number: format!("123456789{}", i),
                    password: "hashed_password".to_string(),
                    is_supervisor: false,
                    org_id: created_org.id,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                    role: Role::Operator,
                    email_verified: false,
                };
                let created_user = futures::executor::block_on(user_repo.create(conn, &user))?;
                users.push(created_user);
            }

            // Test soft delete of organization
            let deleted_org = futures::executor::block_on(org_repo.soft_delete(conn, created_org.id))?;
            assert!(deleted_org.deleted_at.is_some());

            // Verify organization doesn't appear in list
            let orgs = futures::executor::block_on(org_repo.list(conn, &PaginationParams::default()))?;
            assert!(!orgs.iter().any(|o| o.id == created_org.id));

            // Verify that users can still be found by ID
            for user in &users {
                let found_user = futures::executor::block_on(user_repo.find_by_id(conn, user.id))?;
                assert_eq!(found_user.id, user.id);
            }

            // Test soft delete of a user
            let deleted_user = futures::executor::block_on(user_repo.soft_delete(conn, users[0].id))?;
            assert!(deleted_user.deleted_at.is_some());

            // Verify deleted user doesn't appear in list but others do
            let remaining_users = futures::executor::block_on(user_repo.list(conn, &PaginationParams::default()))?;
            assert!(!remaining_users.iter().any(|u| u.id == users[0].id));
            assert!(remaining_users.iter().any(|u| u.id == users[1].id));
            assert!(remaining_users.iter().any(|u| u.id == users[2].id));

            // Clean up by rolling back
            Err(DatabaseError::TransactionFailed("Test completed successfully".to_string()).into())
        })
        .await;

        // Handle the expected rollback
        match test_result {
            Ok(_) => panic!("Expected transaction to be rolled back"),
            Err(e) => {
                if let Some(db_error) = e.source().and_then(|e| e.downcast_ref::<DatabaseError>()) {
                    match db_error {
                        DatabaseError::TransactionFailed(_) => Ok(()),
                        _ => Err(e),
                    }
                } else {
                    Err(e)
                }
            }
        }
    }
}
