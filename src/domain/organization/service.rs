use crate::{
    api::utils::PaginationParams,
    api::resources::organization::dto::{CreateOrganizationInput, UpdateOrganizationInput},
    db::{
        models::Organization,
        repositories::organization::OrganizationRepository,
    },
    domain::organization::validation::OrganizationValidator,
    error::{ApiError, ErrorCode, Result},
};
use diesel::PgConnection;
use tracing::{error, info};
use uuid::Uuid;

/// Service for managing organizations
pub struct OrganizationService<R: OrganizationRepository + Send + Sync> {
    repository: R,
}

impl<R: OrganizationRepository + Send + Sync> OrganizationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }

    /// Creates a new organization
    pub async fn create(&self, conn: &mut PgConnection, input: CreateOrganizationInput) -> Result<Organization> {
        OrganizationValidator::validate_create(conn, &self.repository, &input).await?;
        
        let org: Organization = input.into();
        let result = self.repository.create(conn, &org).await;
        
        if let Ok(org) = &result {
            info!(
                organization_id = %org.id,
                "Created organization '{}'", org.name
            );
        }
        
        result
    }

    /// Updates an existing organization
    pub async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrganizationInput,
    ) -> Result<Organization> {
        OrganizationValidator::validate_update(conn, &self.repository, &input, id).await?;
        
        let org: Organization = (id, input).into();
        let result = self.repository.update(conn, id, &org).await;
        
        if let Ok(org) = &result {
            info!(
                organization_id = %org.id,
                "Updated organization '{}'", org.name
            );
        }
        
        result
    }

    /// Deletes an organization
    pub async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        let result = self.repository.soft_delete(conn, id).await;
        
        if let Ok(org) = &result {
            info!(
                organization_id = %org.id,
                "Deleted organization '{}'", org.name
            );
        }
        
        result
    }

    /// Gets an organization by ID
    pub async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        self.repository.find_by_id(conn, id).await
    }

    /// Lists organizations with pagination
    pub async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<Organization>> {
        self.repository.list(conn, pagination).await
    }

    /// Gets an organization by name
    pub async fn get_by_name(&self, conn: &mut PgConnection, name: &str) -> Result<Organization> {
        let result = self.repository.find_by_name(conn, name).await;
        
        match result {
            Ok(Some(org)) => Ok(org),
            Ok(None) => {
                error!(
                    error_code = %ErrorCode::NotFound,
                    organization_name = %name,
                    "Organization not found"
                );
                Err(ApiError::not_found(format!("Organization with name {} not found", name)))
            }
            Err(e) => Err(e),
        }
    }
}