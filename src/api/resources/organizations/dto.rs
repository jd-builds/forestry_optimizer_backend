use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate as ValidatorValidate;
use utoipa::ToSchema;

use crate::{
    database::{
        models::Organization,
        repositories::organization::OrganizationRepository,
    },
    error::{ApiError, Result, ErrorContext},
};

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

#[async_trait::async_trait]
pub trait Validate {
    async fn validate<R: OrganizationRepository + Send + Sync>(
        &self,
        conn: &mut PgConnection,
        repo: &R,
        org_id: Option<Uuid>,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl Validate for CreateOrganizationInput {
    async fn validate<R: OrganizationRepository + Send + Sync>(
        &self,
        conn: &mut PgConnection,
        repo: &R,
        _org_id: Option<Uuid>,
    ) -> Result<()> {
        // Validate struct using validator
        if let Err(e) = ValidatorValidate::validate(self) {
            return Err(ApiError::validation_with_context(
                "Invalid input",
                ErrorContext::new().with_details(serde_json::json!(e))
            ));
        }

        // Check if organization with same name exists
        if let Ok(Some(_)) = repo.find_by_name(conn, &self.name).await {
            return Err(ApiError::validation_with_context(
                "Organization with name already exists",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "name",
                    "code": "DUPLICATE",
                    "value": self.name
                }))
            ));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Validate for UpdateOrganizationInput {
    async fn validate<R: OrganizationRepository + Send + Sync>(
        &self,
        conn: &mut PgConnection,
        repo: &R,
        org_id: Option<Uuid>,
    ) -> Result<()> {
        // Validate struct using validator
        if let Err(e) = ValidatorValidate::validate(self) {
            return Err(ApiError::validation_with_context(
                "Invalid input",
                ErrorContext::new().with_details(serde_json::json!(e))
            ));
        }

        // If name is being updated, check if it conflicts with existing org
        if let Some(name) = &self.name {
            if let Ok(Some(existing)) = repo.find_by_name(conn, name).await {
                if Some(existing.id) != org_id {
                    return Err(ApiError::validation_with_context(
                        "Organization with name already exists",
                        ErrorContext::new().with_details(serde_json::json!({
                            "field": "name",
                            "code": "DUPLICATE",
                            "value": name
                        }))
                    ));
                }
            }
        }

        Ok(())
    }
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