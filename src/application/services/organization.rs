use crate::{
    common::error::{ApiError, ErrorCode, Result, ErrorContext},
    common::pagination::PaginationParams,
    domain::{
        models::Organization,
        repositories::OrganizationRepository,
        services::OrganizationService,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct OrganizationServiceImpl<R> {
    repository: Arc<R>,
}

impl<R> OrganizationServiceImpl<R>
where
    R: OrganizationRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> OrganizationService for OrganizationServiceImpl<R>
where
    R: OrganizationRepository + Send + Sync,
{
    async fn create_organization(&self, name: String) -> Result<Organization> {
        // Check if organization with same name exists
        if let Some(_) = self.repository.as_ref().find_by_name(&name).await? {
            return Err(ApiError::new(
                ErrorCode::Conflict,
                "Organization with this name already exists",
                ErrorContext::new()
            ));
        }

        let organization = Organization::new(name);
        let created = self.repository.as_ref().create(&organization).await?;
        
        info!(
            organization_id = %created.id,
            organization_name = %created.name,
            "Created new organization"
        );
        
        Ok(created)
    }

    async fn update_organization(&self, id: Uuid, name: String) -> Result<Organization> {
        // Check if organization exists
        let mut organization = self.repository.find_by_id(id).await?;
        
        // Check if new name is available (if name is being changed)
        if organization.name != name {
            if let Some(_) = self.repository.as_ref().find_by_name(&name).await? {
                return Err(ApiError::new(
                    ErrorCode::Conflict,
                    "Organization with this name already exists",
                    ErrorContext::new()
                ));
            }
        }

        organization.name = name;
        let updated = self.repository.update(id, &organization).await?;
        
        info!(
            organization_id = %updated.id,
            organization_name = %updated.name,
            "Updated organization"
        );
        
        Ok(updated)
    }

    async fn delete_organization(&self, id: Uuid) -> Result<Organization> {
        // First get the organization to return its data
        let organization = self.repository.as_ref().find_by_id(id).await?;
        
        // Then soft delete it
        self.repository.as_ref().soft_delete(id).await?;
        
        info!(
            organization_id = %organization.id,
            organization_name = %organization.name,
            "Deleted organization"
        );
        
        Ok(organization)
    }

    async fn get_organization(&self, id: Uuid) -> Result<Organization> {
        self.repository.find_by_id(id).await
    }

    async fn list_organizations(&self, pagination: PaginationParams) -> Result<(Vec<Organization>, i64)> {
        self.repository.as_ref().list(&pagination).await
    }

    async fn is_name_available(&self, name: &str) -> Result<bool> {
        let existing = self.repository.as_ref().find_by_name(name).await?;
        Ok(existing.is_none())
    }
} 