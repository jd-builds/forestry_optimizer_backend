use crate::{db::models::{base::Timestamps, Organization}, errors::{ApiError, ErrorCode, ErrorContext}};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::warn;
use utoipa::ToSchema;
use uuid::Uuid;

pub trait Validate {
    fn validate(&self) -> Result<(), ApiError>;
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct CreateOrganizationInput {
    pub name: String, 
}

pub fn validate_organization_name(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        warn!(
            error_code = %ErrorCode::ValidationError,
            field = "name",
            "Organization name cannot be empty"
        );
        return Err(ApiError::new(
            ErrorCode::ValidationError,
            "Organization name cannot be empty",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "name",
                "code": "REQUIRED",
                "value": name
            }))
        ));
    }
    if name.len() > 255 {
        warn!(
            error_code = %ErrorCode::ValidationError,
            field = "name",
            "Organization name cannot be longer than 255 characters"
        );
        return Err(ApiError::new(
            ErrorCode::ValidationError,
            "Organization name cannot be longer than 255 characters",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "name",
                "code": "MAX_LENGTH",
                "max_length": 255,
                "actual_length": name.len()
            }))
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

#[derive(Debug, Deserialize, ToSchema)]
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
        let created_at = org.created_at();
        let updated_at = org.updated_at();
        
        Self {
            id: org.id,
            name: org.name,
            created_at,
            updated_at,
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
