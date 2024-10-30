use crate::db::models::Organization;
use chrono::{DateTime, Utc};
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
pub struct OrganizationDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub organization: OrganizationDto,
}

#[derive(Serialize, ToSchema)]
pub struct OrganizationListResponse {
    pub organizations: Vec<OrganizationDto>,
    pub total: i64,
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

impl From<(Uuid, UpdateOrganizationInput)> for Organization {
    fn from((id, input): (Uuid, UpdateOrganizationInput)) -> Self {
        Organization {
            id,
            name: input.name,
            updated_at: Utc::now(),
            created_at: Utc::now(),
            deleted_at: None,
        }
    }
}
