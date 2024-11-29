use crate::domain::models::organization::Organization;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrganizationInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateOrganizationInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationListResponse {
    pub organizations: Vec<OrganizationDto>,
    pub total: i64,
}

impl From<Organization> for OrganizationDto {
    fn from(org: Organization) -> Self {
        Self {
            id: org.id,
            name: org.name,
            created_at: org.created_at,
            updated_at: org.updated_at,
        }
    }
}
