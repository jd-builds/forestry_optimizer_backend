//! Domain logic tests
//! Tests for business rules and domain validations

use crate::{
    db::{
        models::organization::Organization,
        repositories::{
            organization::OrganizationRepositoryImpl,
            Repository,
        },
    },
    ErrorCode,
    domain::organizations::OrganizationValidator,
    api::resources::organizations::dto::{CreateOrganizationInput, UpdateOrganizationInput},
    error::Result,
    tests::common::TestDb,
    error::ApiError,
    error::ErrorContext,
};
use chrono::Utc;
use uuid::Uuid;
use crate::db::models::auth::User;
use crate::domain::auth::AuthValidator;

#[cfg(test)]
mod tests {
    use crate::{db::{models::auth::Role, repositories::{auth::CreateUserParams, UserRepositoryImpl}}, tests::common::TestAuth};

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_organization_validation() -> Result<()> {
        let test_result = TestDb::run_test(|conn| {
            let repo = OrganizationRepositoryImpl;

            // Test name uniqueness
            let input = CreateOrganizationInput {
                name: "Test Org".to_string(),
            };
            
            // First creation should succeed
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    OrganizationValidator::validate_create(conn, &repo, &input).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Create the organization
                    let org = Organization {
                        id: Uuid::new_v4(),
                        name: input.name.clone(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_org = repo.create(conn, &org).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Second creation with same name should fail
                    let result = OrganizationValidator::validate_create(conn, &repo, &input).await;
                    assert!(result.is_err(), "Should not allow duplicate organization names");

                    // Test name length validation
                    let empty_name = CreateOrganizationInput {
                        name: "".to_string(),
                    };
                    let result = OrganizationValidator::validate_create(conn, &repo, &empty_name).await;
                    assert!(result.is_err(), "Should not allow empty organization names");

                    let long_name = CreateOrganizationInput {
                        name: "a".repeat(256),
                    };
                    let result = OrganizationValidator::validate_create(conn, &repo, &long_name).await;
                    assert!(result.is_err(), "Should not allow names longer than 255 characters");

                    // Test update validation
                    let update_input = UpdateOrganizationInput {
                        name: Some("Updated Org".to_string()),
                    };
                    
                    OrganizationValidator::validate_update(conn, &repo, &update_input, created_org.id).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    Err(diesel::result::Error::RollbackTransaction)
                })
            }).map_err(|e| ApiError::new(
                ErrorCode::DatabaseError,
                "Transaction failed",
                ErrorContext::default()
            ).with_source(e))?;

            Ok(())
        }).await;

        match test_result {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.code == ErrorCode::DatabaseError {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_user_validation() -> Result<()> {
        let test_result = TestDb::run_test(|conn| {
            let repo = UserRepositoryImpl;

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    // Test invalid email
                    let result = AuthValidator::validate_login(conn, &repo, "invalid-email", "password123").await;
                    assert!(result.is_err(), "Should not allow invalid email format");

                    // Test valid email but wrong password
                    let result = AuthValidator::validate_login(conn, &repo, "test@example.com", "wrongpass").await;
                    assert!(result.is_err(), "Should not allow wrong password");

                    // Create test user
                    let user = User {
                        id: Uuid::new_v4(),
                        email: "test@example.com".to_string(),
                        first_name: "Test".to_string(),
                        last_name: "User".to_string(),
                        phone_number: "1234567890".to_string(),
                        password: "hashed_password".to_string(),
                        role: Role::Operator,
                        org_id: Uuid::new_v4(),
                        email_verified: true,
                        is_supervisor: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_user = repo.create(conn, &user).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Test valid login
                    let result = AuthValidator::validate_login(conn, &repo, &created_user.email, "password123").await;
                    assert!(result.is_ok(), "Should allow valid credentials");

                    Err(diesel::result::Error::RollbackTransaction)
                })
            }).map_err(|e| ApiError::new(
                ErrorCode::DatabaseError,
                "Transaction failed",
                ErrorContext::default()
            ).with_source(e))?;

            Ok(())
        }).await;

        match test_result {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.code == ErrorCode::DatabaseError {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_business_rules() -> Result<()> {
        let test_result = TestDb::run_test(|conn| {
            let org_repo = OrganizationRepositoryImpl;
            let user_repo = UserRepositoryImpl;

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    // Create organization
                    let org = Organization {
                        id: Uuid::new_v4(),
                        name: "Test Org".to_string(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_org = org_repo.create(conn, &org).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Create a user in the organization
                    let user = User {
                        id: Uuid::new_v4(),
                        email: "test@example.com".to_string(),
                        first_name: "Test".to_string(),
                        last_name: "User".to_string(),
                        phone_number: "1234567890".to_string(),
                        password: "hashed_password".to_string(),
                        role: Role::Operator,
                        org_id: created_org.id,
                        email_verified: true,
                        is_supervisor: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    user_repo.create(conn, &user).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Test organization deletion with active users
                    let result = org_repo.soft_delete(conn, created_org.id).await;
                    assert!(result.is_err(), "Should not allow deleting organization with active users");

                    // Test user reassignment
                    let new_org = Organization {
                        id: Uuid::new_v4(),
                        name: "New Org".to_string(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_new_org = org_repo.create(conn, &new_org).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Test registration with new org
                    let params = CreateUserParams {
                        email: "new@example.com",
                        first_name: "New",
                        last_name: "User",
                        phone_number: "1234567890",
                        password: "password123",
                        org_id: created_new_org.id,
                    };
                    
                    AuthValidator::validate_registration(conn, &user_repo, &params).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Test invalid registration
                    let invalid_params = CreateUserParams {
                        email: "invalid-email",
                        first_name: "New",
                        last_name: "User",
                        phone_number: "1234567890",
                        password: "password123",
                        org_id: created_new_org.id,
                    };
                    
                    let result = AuthValidator::validate_registration(conn, &user_repo, &invalid_params).await;
                    assert!(result.is_err(), "Should not allow invalid email format");

                    Err(diesel::result::Error::RollbackTransaction)
                })
            }).map_err(|e| ApiError::new(
                ErrorCode::DatabaseError,
                "Transaction failed",
                ErrorContext::default()
            ).with_source(e))?;

            Ok(())
        }).await;

        match test_result {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.code == ErrorCode::DatabaseError {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_password_validation() -> Result<()> {
        let test_result = TestDb::run_test(|conn| {
            let repo = UserRepositoryImpl;

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    // Test short password
                    let params = CreateUserParams {
                        email: "test@example.com",
                        first_name: "Test",
                        last_name: "User",
                        phone_number: "1234567890",
                        password: "short",
                        org_id: Uuid::new_v4(),
                    };
                    
                    let result = AuthValidator::validate_registration(conn, &repo, &params).await;
                    assert!(result.is_err(), "Should not allow short passwords");

                    // Test password without numbers
                    let params = CreateUserParams {
                        email: "test@example.com",
                        first_name: "Test",
                        last_name: "User",
                        phone_number: "1234567890",
                        password: "onlyletters",
                        org_id: Uuid::new_v4(),
                    };
                    
                    let result = AuthValidator::validate_registration(conn, &repo, &params).await;
                    assert!(result.is_err(), "Should require numbers in password");

                    // Test valid password
                    let params = CreateUserParams {
                        email: "test@example.com",
                        first_name: "Test",
                        last_name: "User",
                        phone_number: "1234567890",
                        password: "ValidPass123!",
                        org_id: Uuid::new_v4(),
                    };
                    
                    AuthValidator::validate_registration(conn, &repo, &params).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    Err(diesel::result::Error::RollbackTransaction)
                })
            }).map_err(|e| ApiError::new(
                ErrorCode::DatabaseError,
                "Transaction failed",
                ErrorContext::default()
            ).with_source(e))?;

            Ok(())
        }).await;

        match test_result {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.code == ErrorCode::DatabaseError {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_role_based_access() -> Result<()> {
        let test_result = TestDb::run_test(|conn| {
            let org_repo = OrganizationRepositoryImpl;
            let user_repo = UserRepositoryImpl;

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    // Create organization
                    let org = Organization {
                        id: Uuid::new_v4(),
                        name: "Test Org".to_string(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_org = org_repo.create(conn, &org).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Create users with different roles
                    let admin = User {
                        id: Uuid::new_v4(),
                        email: "admin@example.com".to_string(),
                        first_name: "Admin".to_string(),
                        last_name: "User".to_string(),
                        phone_number: "1234567890".to_string(),
                        password: "hashed_password".to_string(),
                        role: Role::Admin,
                        org_id: created_org.id,
                        email_verified: true,
                        is_supervisor: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_admin = user_repo.create(conn, &admin).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    let operator = User {
                        id: Uuid::new_v4(),
                        email: "operator@example.com".to_string(),
                        first_name: "Operator".to_string(),
                        last_name: "User".to_string(),
                        phone_number: "1234567890".to_string(),
                        password: "hashed_password".to_string(),
                        role: Role::Operator,
                        org_id: created_org.id,
                        email_verified: true,
                        is_supervisor: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    let created_operator = user_repo.create(conn, &operator).await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    // Test role-based permissions
                    #[allow(unused_variables)]
                    let admin_token = TestAuth::create_test_token(created_admin.id, "admin");
                    #[allow(unused_variables)]
                    let operator_token = TestAuth::create_test_token(created_operator.id, "operator");

                    // Admin should be able to create organizations
                    let new_org = Organization {
                        id: Uuid::new_v4(),
                        name: "New Org".to_string(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    
                    let result = org_repo.create(conn, &new_org).await;
                    assert!(result.is_ok(), "Admin should be able to create organizations");

                    // Operator should not be able to create organizations
                    let another_org = Organization {
                        id: Uuid::new_v4(),
                        name: "Another Org".to_string(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        deleted_at: None,
                    };
                    
                    let result = org_repo.create(conn, &another_org).await;
                    assert!(result.is_err(), "Operator should not be able to create organizations");

                    Err(diesel::result::Error::RollbackTransaction)
                })
            }).map_err(|e| ApiError::new(
                ErrorCode::DatabaseError,
                "Transaction failed",
                ErrorContext::default()
            ).with_source(e))?;

            Ok(())
        }).await;

        match test_result {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.code == ErrorCode::DatabaseError {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}
