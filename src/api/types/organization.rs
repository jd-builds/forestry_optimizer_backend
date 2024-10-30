use crate::db::models::Organization;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateOrganizationInput {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateOrganizationInput {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ListOrganizationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub organization: Organization,
}

impl From<CreateOrganizationInput> for Organization {
    fn from(input: CreateOrganizationInput) -> Self {
        Organization {
            id: Uuid::new_v4(),
            name: input.name,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}

impl From<UpdateOrganizationInput> for Organization {
    fn from(input: UpdateOrganizationInput) -> Self {
        Self {
            name: input.name,
            updated_at: chrono::Utc::now(),
            ..Default::default()
        }
    }
}
