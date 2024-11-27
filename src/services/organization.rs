//! Organization service implementation
//! 
//! This module provides the business logic layer for organization operations.
//! It handles validation, error handling, and coordinates between the API layer
//! and data access layer.

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
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Service for managing organization operations
/// 
/// This service encapsulates all business logic for organization operations,
/// providing a clean interface between the API layer and data access layer.
/// 
/// # Type Parameters
/// 
/// * `R` - The repository implementation to use, defaults to OrganizationRepositoryImpl
pub struct OrganizationService<R = OrganizationRepositoryImpl> {
    repository: R,
}

impl<R: OrganizationRepository> OrganizationService<R> {
    /// Creates a new organization service with the given repository
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Validates an organization name
    /// 
    /// Ensures the name meets all business rules and constraints.
    /// Logs appropriate warnings for validation failures.
    fn validate_name(name: &str) -> Result<()> {
        validate_organization_name(name)
    }

    /// Checks for name conflicts with existing organizations
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `name` - Name to check
    /// * `exclude_id` - Optional ID to exclude from the check (for updates)
    async fn check_name_conflict(&self, conn: &mut PgConnection, name: &str, exclude_id: Option<Uuid>) -> Result<()> {
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

    /// Finds an organization by ID
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `id` - Organization ID to find
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
            Err(_) => {
                warn!(
                    error_code = %ErrorCode::NotFound,
                    organization_id = %id,
                    "Organization not found"
                );
            }
        }
        
        result
    }

    /// Creates a new organization
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `input` - Organization creation input
    pub async fn create(&self, conn: &mut PgConnection, input: CreateOrganizationInput) -> Result<Organization> {
        Self::validate_name(&input.name)?;
        self.check_name_conflict(conn, &input.name, None).await?;

        let organization: Organization = input.into();
        let result = self.repository.create(conn, &organization).await;
        
        if let Ok(org) = &result {
            info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization created successfully"
            );
        }
        result
    }

    /// Updates an existing organization
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `id` - Organization ID to update
    /// * `input` - Updated organization data
    pub async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrganizationInput) -> Result<Organization> {
        Self::validate_name(&input.name)?;
        self.check_name_conflict(conn, &input.name, Some(id)).await?;

        let mut organization = self.find_by_id(conn, id).await?;
        organization.name = input.name;
        organization.updated_at = chrono::Utc::now();

        let result = self.repository.update(conn, id, &organization).await;
        
        if let Ok(org) = &result {
            info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization updated successfully"
            );
        }
        result
    }

    /// Lists organizations with pagination
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `pagination` - Pagination parameters
    pub async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<Organization>> {
        let result = self.repository.list(conn, pagination).await;
        
        if let Ok(orgs) = &result {
            info!(
                count = orgs.len(),
                "Organizations retrieved successfully"
            );
        }
        result
    }

    /// Soft deletes an organization
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `id` - Organization ID to delete
    pub async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<Organization> {
        let result = self.repository.soft_delete(conn, id).await;
        
        if let Ok(org) = &result {
            info!(
                organization_id = %org.id,
                organization_name = %org.name,
                "Organization deleted successfully"
            );
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
