use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate as ValidatorValidate;
use utoipa::ToSchema;

use crate::db::models::Organization;

/// Input for creating a new organization
#[derive(Debug, Deserialize, ValidatorValidate, ToSchema)]
pub struct CreateOrganizationInput {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

/// Input for updating an organization
#[derive(Debug, Deserialize, ValidatorValidate, ToSchema)]
pub struct UpdateOrganizationInput {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
}

/// Organization response
#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Query parameters for listing organizations
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListOrganizationsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl From<CreateOrganizationInput> for Organization {
    fn from(input: CreateOrganizationInput) -> Self {
        Self {
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
        Self {
            id,
            name: input.name.unwrap_or_default(),
            created_at: chrono::Utc::now(), // Note: This should ideally preserve the original created_at
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
} 