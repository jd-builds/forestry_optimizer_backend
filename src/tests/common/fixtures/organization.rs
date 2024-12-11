use fake::{Fake, Faker};
use uuid::Uuid;
use diesel::PgConnection;
use chrono::Utc;
use crate::{
    db::{
        models::Organization,
        repositories::{OrganizationRepositoryImpl, Repository},
    },
    error::Result,
};
use super::constants::*;

pub fn fake_organization() -> serde_json::Value {
    serde_json::json!({
        "name": format!("{} {}", TEST_ORG_NAME_PREFIX, Faker.fake::<String>()),
    })
}

pub async fn create_test_organization(conn: &mut PgConnection) -> Result<Organization> {
    let repo = OrganizationRepositoryImpl;
    let org_data = fake_organization();
    
    let org = Organization {
        id: Uuid::new_v4(),
        name: org_data["name"].as_str().unwrap().to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };
    
    repo.create(conn, &org).await
}

pub async fn create_test_organizations(conn: &mut PgConnection, count: i32) -> Result<Vec<Organization>> {
    let mut orgs = Vec::new();
    for _ in 0..count {
        orgs.push(create_test_organization(conn).await?);
    }
    Ok(orgs)
} 