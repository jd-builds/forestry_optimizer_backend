use crate::{db::models::{base::Timestamps, Organization}, errors::{ApiError, ErrorContext}, db::repositories::traits::OrganizationRepository};
use chrono::{DateTime, Utc};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Validate {
    async fn validate<R: OrganizationRepository>(&self, conn: &mut PgConnection, repository: &R, current_org_id: Option<Uuid>) -> Result<(), ApiError>;
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct CreateOrganizationInput {
    pub name: String, 
}

/// Validates basic organization name rules (sync)
fn validate_organization_name_basic(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        return Err(ApiError::validation_with_context(
            "Organization name cannot be empty",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "name",
                "code": "REQUIRED",
                "value": name
            }))
        ));
    }
    if name.len() > 255 {
        return Err(ApiError::validation_with_context(
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

/// Validates organization name uniqueness (async)
pub async fn validate_organization_name_unique<R: OrganizationRepository>(
    name: &str,
    conn: &mut PgConnection,
    repository: &R,
    current_org_id: Option<Uuid>,
) -> Result<(), ApiError> {
    if let Ok(Some(existing)) = repository.find_by_name(conn, name).await {
        match current_org_id {
            Some(id) if existing.id == id => Ok(()), // Same org, allow the update
            _ => Err(ApiError::validation_with_context(
                "Organization name already exists",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "name",
                    "code": "DUPLICATE",
                    "value": name
                }))
            )),
        }
    } else {
        Ok(())
    }
}

#[async_trait::async_trait]
impl Validate for CreateOrganizationInput {
    async fn validate<R: OrganizationRepository>(&self, conn: &mut PgConnection, repository: &R, _current_org_id: Option<Uuid>) -> Result<(), ApiError> {
        validate_organization_name_basic(&self.name)?;
        validate_organization_name_unique(&self.name, conn, repository, None).await
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateOrganizationInput {
    pub name: String,
}

#[async_trait::async_trait]
impl Validate for UpdateOrganizationInput {
    async fn validate<R: OrganizationRepository>(&self, conn: &mut PgConnection, repository: &R, current_org_id: Option<Uuid>) -> Result<(), ApiError> {
        validate_organization_name_basic(&self.name)?;
        validate_organization_name_unique(&self.name, conn, repository, current_org_id).await
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
