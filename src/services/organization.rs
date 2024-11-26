use crate::{
    api::types::{
        organization::{CreateOrganizationInput, UpdateOrganizationInput},
        pagination::PaginationParams,
    },
    db::{
        models::Organization,
        repositories::{traits::OrganizationRepository, OrganizationRepositoryImpl},
    },
    errors::{ApiError, ErrorCode, ErrorContext, Result},
};
use diesel::PgConnection;
use uuid::Uuid;
use tracing::{info, debug, error, instrument};

pub struct OrganizationService<R = OrganizationRepositoryImpl> {
    repository: R,
}

// Constructor and default implementations
impl<R: OrganizationRepository> OrganizationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    #[instrument(skip_all)]
    fn validate_name(name: &str) -> Result<()> {
        if name.trim().is_empty() {
            debug!(
                error_code = %ErrorCode::ValidationError,
                field = "name",
                "Organization name validation failed: empty name"
            );
            return Err(ApiError::new(
                ErrorCode::ValidationError,
                "Organization name cannot be empty",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "name",
                    "code": "REQUIRED"
                }))
            ));
        }
        Ok(())
    }

    fn handle_db_error(e: impl std::error::Error, context: &str) -> ApiError {
        error!(error = %e, context = context, "Database error occurred");
        ApiError::new(
            ErrorCode::DatabaseError,
            context,
            ErrorContext::new().with_details(serde_json::json!({ 
                "details": e.to_string() 
            }))
        )
    }

    #[instrument(skip(self, conn), fields(name = %name))]
    fn check_name_conflict(
        &self, 
        conn: &mut PgConnection, 
        name: &str, 
        exclude_id: Option<Uuid>
    ) -> Result<()> {
        match self.repository.find_by_name(conn, name) {
            Ok(Some(existing)) => {
                if exclude_id.map_or(true, |id| existing.id != id) {
                    debug!(
                        error_code = %ErrorCode::Conflict,
                        existing_id = %existing.id,
                        "Name conflict detected"
                    );
                    return Err(ApiError::new(
                        ErrorCode::Conflict,
                        format!("Organization with name '{}' already exists", name),
                        ErrorContext::new().with_details(serde_json::json!({
                            "field": "name",
                            "code": "UNIQUE",
                            "value": name
                        }))
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
    pub async fn create(&self, conn: &mut PgConnection, input: CreateOrganizationInput) -> Result<Organization> {
        debug!("Initiating organization creation");
        
        Self::validate_name(&input.name)?;
        self.check_name_conflict(conn, &input.name, None)?;

        let organization: Organization = input.into();
        let result = self.repository.create(conn, &organization);

        match &result {
            Ok(org) => info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization successfully created"
            ),
            Err(e) => match e.code {
                ErrorCode::ValidationError => debug!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Validation error during creation"
                ),
                _ => error!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Failed to create organization"
                ),
            }
        }

        result
    }

    #[instrument(skip(self, conn), fields(organization_id = %id))]
    pub async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        debug!("Initiating organization lookup");
        
        let result = self.repository.find_by_id(conn, id);

        match &result {
            Ok(org) => debug!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization found"
            ),
            Err(e) => match e.code {
                ErrorCode::NotFound => debug!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Organization not found"
                ),
                _ => error!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Failed to find organization"
                ),
            }
        }

        result
    }

    #[instrument(skip(self, conn), fields(organization_id = %id, organization_name = %input.name))]
    pub async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrganizationInput) -> Result<Organization> {
        debug!("Initiating organization update");
        
        Self::validate_name(&input.name)?;
        self.check_name_conflict(conn, &input.name, Some(id))?;

        let mut organization = self.find_by_id(conn, id).await?;
        organization.name = input.name;
        organization.updated_at = chrono::Utc::now();

        let result = self.repository.update(conn, id, &organization);

        match &result {
            Ok(org) => info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization successfully updated"
            ),
            Err(e) => match e.code {
                ErrorCode::NotFound => debug!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Organization not found during update"
                ),
                _ => error!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Failed to update organization"
                ),
            }
        }

        result
    }

    #[instrument(skip(self, conn))]
    pub async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<Organization>> {
        debug!(
            page = pagination.page,
            per_page = pagination.per_page,
            "Initiating organization list"
        );
        
        let result = self.repository.list(conn, pagination);

        match &result {
            Ok(orgs) => info!(
                count = orgs.len(),
                page = pagination.page,
                per_page = pagination.per_page,
                "Organizations successfully retrieved"
            ),
            Err(e) => error!(
                error_code = %e.code,
                error_message = %e.message,
                "Failed to list organizations"
            ),
        }

        result
    }

    #[instrument(skip(self, conn), fields(organization_id = %id))]
    pub async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        debug!("Initiating organization deletion");
        
        let result = self.repository.soft_delete(conn, id);

        match &result {
            Ok(org) => info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization successfully deleted"
            ),
            Err(e) => match e.code {
                ErrorCode::NotFound => debug!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Organization not found during deletion"
                ),
                _ => error!(
                    error_code = %e.code,
                    error_message = %e.message,
                    "Failed to delete organization"
                ),
            }
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
