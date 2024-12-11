use chrono::Utc;
use uuid::Uuid;
use crate::{
    db::{
        models::{auth::{Role, User}, Organization},
        repositories::{auth::UserRepositoryImpl, OrganizationRepositoryImpl, Repository},
    },
    error::Result,
    tests::{common::helpers::TestDb, setup},
    api::utils::PaginationParams,
};
use crate::db::repositories::auth::UserRepository;

#[tokio::test]
async fn test_user_crud() -> Result<()> {
    setup();
    TestDb::run_test(|conn| {
        Box::pin(async move {
            let repo = UserRepositoryImpl;

            // Create an organization first
            let org = Organization {
                id: Uuid::new_v4(),
                name: format!("Test Org {}", Uuid::new_v4()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let org_repo = OrganizationRepositoryImpl;
            let created_org = org_repo.create(conn, &org).await?;

            // Test Create
            let uuid = Uuid::new_v4();
            let password = User::hash_password("test_password")?;
            let user = User {
                id: Uuid::new_v4(),
                email: format!("test{}@example.com", uuid),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                phone_number: format!("1234567890{}", uuid.simple()),
                password,
                role: Role::Operator,
                org_id: created_org.id,
                email_verified: true,
                is_supervisor: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_user = repo.create(conn, &user).await?;
            assert_eq!(created_user.email, user.email);

            // Test Read
            let found_user = repo.find_by_id(conn, created_user.id).await?;
            assert_eq!(found_user.id, created_user.id);
            assert_eq!(found_user.email, created_user.email);

            // Test Update
            let mut updated_user = created_user.clone();
            updated_user.first_name = "Updated".to_string();
            let updated_user = repo.update(conn, updated_user.id, &updated_user).await?;
            assert_eq!(updated_user.first_name, "Updated");

            // Test List
            let users = repo.list(conn, &PaginationParams::new(1, 100)).await?;
            assert!(!users.is_empty());
            assert!(users.iter().any(|u| u.id == created_user.id));

            // Test Soft Delete
            let deleted_user = repo.soft_delete(conn, created_user.id).await?;
            assert!(deleted_user.deleted_at.is_some());

            // Verify deleted user doesn't appear in list
            let users_after_delete = repo.list(conn, &PaginationParams::new(1, 100)).await?;
            assert!(!users_after_delete.iter().any(|u| u.id == created_user.id));

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn test_user_queries() -> Result<()> {
    TestDb::run_test(|conn| {
        Box::pin(async move {
            let repo = UserRepositoryImpl;

            // Create an organization first
            let org = Organization {
                id: Uuid::new_v4(),
                name: format!("Test Org {}", Uuid::new_v4()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let org_repo = OrganizationRepositoryImpl;
            let created_org = org_repo.create(conn, &org).await?;

            // Test find by email
            let email = format!("test{}@example.com", Uuid::new_v4());
            let user = User {
                id: Uuid::new_v4(),
                email: email.clone(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                phone_number: "1234567890".to_string(),
                password: User::hash_password("test_password")?,
                role: Role::Operator,
                org_id: created_org.id,
                email_verified: true,
                is_supervisor: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_user = repo.create(conn, &user).await?;
            
            let found_user = repo.find_by_email(conn, &email).await?;
            assert_eq!(found_user.unwrap().id, created_user.id);

            // Test find by role
            let users_by_role = repo.find_by_role(conn, Role::Operator).await?;
            assert!(!users_by_role.is_empty());
            assert!(users_by_role.iter().any(|u| u.id == created_user.id));

            Ok(())
        })
    }).await
} 