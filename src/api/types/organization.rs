use crate::{db::models::Organization, errors::ApiError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub trait Validate {
    fn validate(&self) -> Result<(), ApiError>;
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct CreateOrganizationInput {
    pub name: String, 
}

fn validate_organization_name(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        return Err(ApiError::new(
            "VALIDATION_ERROR",
            "Organization name cannot be empty",
            Some(serde_json::json!({
                "field": "name",
                "code": "REQUIRED",
                "value": name
            })),
        ));
    }
    if name.len() > 255 {
        return Err(ApiError::new(
            "VALIDATION_ERROR",
            "Organization name cannot be longer than 255 characters",
            Some(serde_json::json!({
                "field": "name",
                "code": "MAX_LENGTH",
                "max_length": 255,
                "actual_length": name.len()
            })),
        ));
    }
    Ok(())
}

impl Validate for CreateOrganizationInput {
    fn validate(&self) -> Result<(), ApiError> {
        validate_organization_name(&self.name)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateOrganizationInput {
    pub name: String,
}

impl Validate for UpdateOrganizationInput {
    fn validate(&self) -> Result<(), ApiError> {
        validate_organization_name(&self.name)
    }
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
