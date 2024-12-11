use chrono::Utc;
use uuid::Uuid;
use crate::{
    db::{
        models::organization::Organization,
        repositories::{
            organization::OrganizationRepositoryImpl,
            Repository,
        },
    },
    domain::organizations::OrganizationValidator,
    api::resources::organizations::dto::{CreateOrganizationInput, UpdateOrganizationInput},
    error::Result,
    tests::{common::helpers::TestDb, setup},
};

#[tokio::test(flavor = "multi_thread")]
async fn test_organization_validation() -> Result<()> {
    setup();
    TestDb::run_test(|conn| {
        Box::pin(async move {
            let repo = OrganizationRepositoryImpl;

            // Test name uniqueness
            let input = CreateOrganizationInput {
                name: format!("Test Org {}", Uuid::new_v4()),
            };
            
            // First creation should succeed
            OrganizationValidator::validate_create(conn, &repo, &input).await?;

            // Create the organization
            let org = Organization {
                id: Uuid::new_v4(),
                name: input.name.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_org = repo.create(conn, &org).await?;

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
            
            let result = OrganizationValidator::validate_update(conn, &repo, &update_input, created_org.id).await;
            assert!(result.is_ok(), "Should allow valid update");

            Ok(())
        })
    }).await
}

#[tokio::test(flavor = "multi_thread")]
async fn test_organization_business_rules() -> Result<()> {
    TestDb::run_test(|conn| {
        Box::pin(async move {
            let repo = OrganizationRepositoryImpl;

            // Test organization with active users
            let org = Organization {
                id: Uuid::new_v4(),
                name: format!("Test Org {}", Uuid::new_v4()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_org = repo.create(conn, &org).await?;

            // Test deletion validation
            let result = repo.soft_delete(conn, created_org.id).await;
            assert!(result.is_ok(), "Should allow deleting organization without users");

            Ok(())
        })
    }).await
} 