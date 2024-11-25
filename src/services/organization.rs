use crate::{
    api::types::{
        organization::{CreateOrganizationInput, UpdateOrganizationInput},
        pagination::PaginationParams,
    },
    db::{
        models::Organization,
        repositories::{traits::OrganizationRepository, OrganizationRepositoryImpl},
    },
    errors::ApiError,
};
use diesel::PgConnection;
use uuid::Uuid;

pub struct OrganizationService<R = OrganizationRepositoryImpl> {
    repository: R,
}

// Constructor and default implementations
impl<R: OrganizationRepository> OrganizationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn validate_name(name: &str) -> Result<(), ApiError> {
        if name.trim().is_empty() {
            return Err(ApiError::new(
                "VALIDATION_ERROR",
                "Organization name cannot be empty",
                Some(serde_json::json!({
                    "field": "name",
                    "code": "REQUIRED"
                })),
            ));
        }
        Ok(())
    }

    fn handle_db_error(e: impl std::error::Error, context: &str) -> ApiError {
        ApiError::new(
            "INTERNAL_SERVER_ERROR",
            context,
            Some(serde_json::json!({ "details": e.to_string() })),
        )
    }

    fn check_name_conflict(&self, conn: &mut PgConnection, name: &str, exclude_id: Option<Uuid>) -> Result<(), ApiError> {
        match self.repository.find_by_name(conn, name) {
            Ok(Some(existing)) => {
                if exclude_id.map_or(true, |id| existing.id != id) {
                    return Err(ApiError::new(
                        "RESOURCE_CONFLICT",
                        &format!("Organization with name '{}' already exists", name),
                        Some(serde_json::json!({
                            "field": "name",
                            "code": "UNIQUE",
                            "value": name
                        })),
                    ));
                }
            }
            Ok(None) => (),
            Err(e) => {
                return Err(Self::handle_db_error(
                    e,
                    "Failed to check for existing organization",
                ));
            }
        }
        Ok(())
    }
}

// Main service implementation
impl<R: OrganizationRepository> OrganizationService<R> {
    pub async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization, ApiError> {
        self.repository
            .find_by_id(conn, id)
            .map_err(|e| Self::handle_db_error(e, "Failed to fetch organization"))
    }

    pub async fn list(
        &self,
        conn: &mut PgConnection,
        pagination: &PaginationParams,
    ) -> Result<Vec<Organization>, ApiError> {
        self.repository
            .list(conn, pagination)
            .map_err(|e| Self::handle_db_error(e, "Failed to list organizations"))
    }

    pub async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrganizationInput,
    ) -> Result<Organization, ApiError> {
        // Validate input
        Self::validate_name(&input.name)?;
        
        // Check for name conflicts
        self.check_name_conflict(conn, &input.name, None)?;

        // Create organization
        let organization: Organization = input.into();
        self.repository
            .create(conn, &organization)
            .map_err(|e| Self::handle_db_error(e, "Failed to create organization"))
    }

    pub async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrganizationInput,
    ) -> Result<Organization, ApiError> {
        // Validate input
        Self::validate_name(&input.name)?;
        
        // Check for name conflicts
        self.check_name_conflict(conn, &input.name, Some(id))?;

        // Find and update the organization
        let mut organization = self.find_by_id(conn, id).await?;
        
        organization.name = input.name;
        organization.updated_at = chrono::Utc::now();

        self.repository
            .update(conn, id, &organization)
            .map_err(|e| Self::handle_db_error(e, "Failed to update organization"))
    }

    pub async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization, ApiError> {
        self.repository
            .soft_delete(conn, id)
            .map_err(|e| Self::handle_db_error(e, "Failed to delete organization"))
    }
}

// Default implementation using OrganizationRepositoryImpl
impl Default for OrganizationService<OrganizationRepositoryImpl> {
    fn default() -> Self {
        Self::new(OrganizationRepositoryImpl)
    }
}
