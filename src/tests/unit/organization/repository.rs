use chrono::Utc;
use uuid::Uuid;
use crate::{
    api::utils::PaginationParams, db::{
        models::organization::Organization,
        repositories::{
            organization::OrganizationRepositoryImpl, OrganizationRepository, Repository
        },
    }, error::Result, tests::{common::helpers::TestDb, setup}
};

#[tokio::test]
async fn test_organization_crud() -> Result<()> {
    setup();
    TestDb::run_test(|conn| {
        Box::pin(async move {
            let repo = OrganizationRepositoryImpl;

            // Test Create
            let org = Organization {
                id: Uuid::new_v4(),
                name: format!("Test Org {}", Uuid::new_v4()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_org = repo.create(conn, &org).await?;
            assert_eq!(created_org.name, org.name);
            assert_eq!(created_org.id, org.id);

            // Test Read
            let found_org = repo.find_by_id(conn, created_org.id).await?;
            assert_eq!(found_org.id, created_org.id);
            assert_eq!(found_org.name, created_org.name);

            // Test Update
            let mut updated_org = created_org.clone();
            let new_name = format!("Updated Org {}", Uuid::new_v4());
            updated_org.name = new_name.clone();
            let updated_org = repo.update(conn, updated_org.id, &updated_org).await?;
            assert_eq!(updated_org.name, new_name);

            // Test List
            let orgs = repo.list(conn, &PaginationParams::new(1, 100)).await?;
            assert!(!orgs.is_empty());
            assert!(orgs.iter().any(|o| o.id == created_org.id));

            // Test Soft Delete
            let deleted_org = repo.soft_delete(conn, created_org.id).await?;
            assert!(deleted_org.deleted_at.is_some());

            // Verify deleted org doesn't appear in list
            let orgs_after_delete = repo.list(conn, &PaginationParams::new(1, 100)).await?;
            assert!(!orgs_after_delete.iter().any(|o| o.id == created_org.id));

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn test_organization_queries() -> Result<()> {
    TestDb::run_test(|conn| {
        Box::pin(async move {
            let repo = OrganizationRepositoryImpl;

            // Test find by name
            let name = format!("Test Org {}", Uuid::new_v4());
            let org = Organization {
                id: Uuid::new_v4(),
                name: name.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };
            let created_org = repo.create(conn, &org).await?;
            
            let found_org = repo.find_by_name(conn, &name).await?;
            assert_eq!(found_org.unwrap().id, created_org.id);

            Ok(())
        })
    }).await
} 