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
use tracing::{info, debug, warn, error, instrument};

pub struct OrganizationService<R = OrganizationRepositoryImpl> {
    repository: R,
}

// Constructor and default implementations
impl<R: OrganizationRepository> OrganizationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn validate_name(name: &str) -> Result<(), ApiError> {
        debug!("Validating organization input");
        if name.trim().is_empty() {
            warn!(
                error_code = "VALIDATION_ERROR",
                error_message = "Organization name cannot be empty",
                "Validation error"
            );
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
        debug!("Checking for name conflicts");
        match self.repository.find_by_name(conn, name) {
            Ok(Some(existing)) => {
                if exclude_id.map_or(true, |id| existing.id != id) {
                    warn!(
                        error_code = "RESOURCE_CONFLICT",
                        error_message = &format!("Organization with name '{}' already exists", name),
                        "Resource conflict"
                    );
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
    #[instrument(skip(self, conn), fields(organization_name = %input.name))]
    pub async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrganizationInput,
    ) -> Result<Organization, ApiError> {
        Self::validate_name(&input.name)?;
        
        self.check_name_conflict(conn, &input.name, None)?;

        debug!("Creating new organization");
        let organization: Organization = input.into();
        let result = self.repository
            .create(conn, &organization)
            .map_err(|e| {
                error!(error = %e, "Failed to create organization");
                Self::handle_db_error(e, "Failed to create organization")
            });

        match &result {
            Ok(org) => info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Successfully created organization"
            ),
            Err(e) => warn!(
                error_code = %e.code,
                error_message = %e.message,
                "Failed to create organization"
            ),
        }

        result
    }

    #[instrument(skip(self, conn), fields(organization_id = %id, organization_name = %input.name))]
    pub async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrganizationInput,
    ) -> Result<Organization, ApiError> {
        Self::validate_name(&input.name)?;
        
        self.check_name_conflict(conn, &input.name, Some(id))?;

        let mut organization = self.find_by_id(conn, id).await?;
        
        let old_name = organization.name.clone();
        organization.name = input.name;
        organization.updated_at = chrono::Utc::now();

        debug!(
            organization_id = %id,
            old_name = %old_name,
            new_name = %organization.name,
            "Updating organization"
        );

        let result = self.repository
            .update(conn, id, &organization)
            .map_err(|e| {
                error!(error = %e, "Failed to update organization");
                Self::handle_db_error(e, "Failed to update organization")
            });

        match &result {
            Ok(org) => info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Successfully updated organization"
            ),
            Err(e) => warn!(
                error_code = %e.code,
                error_message = %e.message,
                "Failed to update organization"
            ),
        }

        result
    }

    #[instrument(skip(self, conn), fields(organization_id = %id))]
    pub async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization, ApiError> {
        debug!(organization_id = %id, "Deleting organization");
        
        let result = self.repository
            .soft_delete(conn, id)
            .map_err(|e| {
                error!(error = %e, "Failed to delete organization");
                Self::handle_db_error(e, "Failed to delete organization")
            });

        match &result {
            Ok(org) => info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Successfully deleted organization"
            ),
            Err(e) => warn!(
                error_code = %e.code,
                error_message = %e.message,
                "Failed to delete organization"
            ),
        }

        result
    }

    #[instrument(skip(self, conn), fields(organization_id = %id))]
    pub async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization, ApiError> {
        debug!(organization_id = %id, "Finding organization by ID");
        
        let result = self.repository
            .find_by_id(conn, id)
            .map_err(|e| {
                error!(error = %e, "Failed to fetch organization");
                Self::handle_db_error(e, "Failed to fetch organization")
            });

        match &result {
            Ok(org) => debug!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Found organization"
            ),
            Err(e) => warn!(
                error_code = %e.code,
                error_message = %e.message,
                "Organization not found"
            ),
        }

        result
    }

    #[instrument(skip(self, conn))]
    pub async fn list(
        &self,
        conn: &mut PgConnection,
        pagination: &PaginationParams,
    ) -> Result<Vec<Organization>, ApiError> {
        debug!(
            page = pagination.page,
            per_page = pagination.per_page,
            "Listing organizations"
        );
        
        let result = self.repository
            .list(conn, pagination)
            .map_err(|e| {
                error!(error = %e, "Failed to list organizations");
                Self::handle_db_error(e, "Failed to list organizations")
            });

        match &result {
            Ok(orgs) => info!(
                count = orgs.len(),
                page = pagination.page,
                per_page = pagination.per_page,
                "Successfully retrieved organizations"
            ),
            Err(e) => warn!(
                error_code = %e.code,
                error_message = %e.message,
                "Failed to list organizations"
            ),
        }

        result
    }
}

// Default implementation using OrganizationRepositoryImpl
impl Default for OrganizationService<OrganizationRepositoryImpl> {
    fn default() -> Self {
        Self::new(OrganizationRepositoryImpl)
    }
}
