use crate::{
    api::types::{
        organization::{validate_organization_name, CreateOrganizationInput, UpdateOrganizationInput},
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
use tracing::{debug, info, warn};

pub struct OrganizationService<R = OrganizationRepositoryImpl> {
    repository: R,
}

// Constructor and default implementations
impl<R: OrganizationRepository> OrganizationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn validate_name(name: &str) -> Result<()> {
        validate_organization_name(name)
    }

    async fn check_name_conflict(
        &self, 
        conn: &mut PgConnection, 
        name: &str, 
        exclude_id: Option<Uuid>
    ) -> Result<()> {
        if let Some(existing) = self.repository.find_by_name(conn, name).await? {
            if exclude_id.map_or(true, |id| existing.id != id) {
                warn!(
                    error_code = %ErrorCode::Conflict,
                    field = "name",
                    "Organization name conflict detected"
                );
                return Err(ApiError::new(
                    ErrorCode::Conflict,
                    "Organization with this name already exists",
                    ErrorContext::new().with_details(serde_json::json!({
                        "field": "name",
                        "code": "DUPLICATE",
                        "value": name
                    }))
                ));
            }
        }
        debug!(
            error_code = %ErrorCode::ValidationError,
            field = "name",
            "Organization name validation passed"
        );
        Ok(())
    }
}

// Main service implementation
impl<R: OrganizationRepository> OrganizationService<R> {
    pub async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        let result = self.repository.find_by_id(conn, id).await;
        
        match &result {
            Ok(org) => {
                debug!(
                    organization_id = %org.id,
                    organization_name = %org.name,
                    "Organization found successfully"
                );
            }
            Err(_e) => {
                warn!(
                    error_code = %ErrorCode::NotFound,
                    organization_id = %id,
                    "Organization not found"
                );
            }
        }
        
        result
    }

    pub async fn create(&self, conn: &mut PgConnection, input: CreateOrganizationInput) -> Result<Organization> {
        Self::validate_name(&input.name)?;
        self.check_name_conflict(conn, &input.name, None).await?;

        let organization: Organization = input.into();
        let result = self.repository.create(conn, &organization).await;
        
        if let Ok(_org) = &result {
            info!("Organization created successfully");
        }
        result
    }

    pub async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrganizationInput) -> Result<Organization> {
        Self::validate_name(&input.name)?;
        self.check_name_conflict(conn, &input.name, Some(id)).await?;

        let mut organization = self.find_by_id(conn, id).await?;
        organization.name = input.name;
        organization.updated_at = chrono::Utc::now();

        let result = self.repository.update(conn, id, &organization).await;
        
        if let Ok(_org) = &result {
            info!("Organization updated successfully");
        }
        result
    }

    pub async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<Organization>> {
        let result = self.repository.list(conn, pagination).await;
        
        if let Ok(_orgs) = &result {
            info!("Organizations retrieved successfully");
        }
        result
    }

    pub async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        let result = self.repository.soft_delete(conn, id).await;
        
        if let Ok(_org) = &result {
            info!("Organization deleted successfully");
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
