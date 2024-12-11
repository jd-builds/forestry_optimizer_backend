use diesel::PgConnection;
use uuid::Uuid;
use validator::Validate as ValidatorValidate;

use crate::{
    api::resources::organization::dto::{CreateOrganizationInput, UpdateOrganizationInput},
    db::repositories::organization::OrganizationRepository,
    error::{ApiError, Result, ErrorContext},
};

pub struct OrganizationValidator;

impl OrganizationValidator {
    /// Validates input for creating a new organization
    pub async fn validate_create<R: OrganizationRepository + Send + Sync>(
        conn: &mut PgConnection,
        repo: &R,
        input: &CreateOrganizationInput,
    ) -> Result<()> {
        // Validate struct using validator
        Self::validate_struct(input)?;
        Self::validate_unique_name(conn, repo, &input.name, None).await?;
        Ok(())
    }

    /// Validates input for updating an organization
    pub async fn validate_update<R: OrganizationRepository + Send + Sync>(
        conn: &mut PgConnection,
        repo: &R,
        input: &UpdateOrganizationInput,
        org_id: Uuid,
    ) -> Result<()> {
        // Validate struct using validator
        Self::validate_struct(input)?;
        
        if let Some(name) = &input.name {
            Self::validate_unique_name(conn, repo, name, Some(org_id)).await?;
        }
        
        Ok(())
    }

    /// Validates struct using the validator crate
    fn validate_struct<T: ValidatorValidate>(input: &T) -> Result<()> {
        if let Err(e) = ValidatorValidate::validate(input) {
            return Err(ApiError::validation_with_context(
                "Invalid input",
                ErrorContext::new().with_details(serde_json::json!(e))
            ));
        }
        Ok(())
    }

    /// Validates that the organization name is unique
    async fn validate_unique_name<R: OrganizationRepository + Send + Sync>(
        conn: &mut PgConnection,
        repo: &R,
        name: &str,
        exclude_org_id: Option<Uuid>,
    ) -> Result<()> {
        if let Ok(Some(existing)) = repo.find_by_name(conn, name).await {
            if Some(existing.id) != exclude_org_id {
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
        Ok(())
    }
} 