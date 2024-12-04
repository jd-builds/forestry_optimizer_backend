//! Organization repository implementation
//! 
//! This module provides the concrete implementation of the organization repository
//! traits. It handles all database operations for organizations with proper error
//! handling and logging.

use crate::{
    api::types::pagination::PaginationParams,
    db::{
        models::Organization,
        repositories::traits::{OrganizationRepository, Repository},
        schema::organizations::dsl::*,
    },
    errors::{ApiError, ErrorCode, ErrorContext, Result},
};
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use tracing::{error, warn};
use uuid::Uuid;

/// Concrete implementation of the organization repository
pub struct OrganizationRepositoryImpl;

#[async_trait]
impl Repository<Organization> for OrganizationRepositoryImpl {
    /// Finds an organization by its unique identifier
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `search_id` - Organization ID to find
    /// 
    /// # Returns
    /// 
    /// Returns the organization if found, otherwise returns a NotFound error
    async fn find_by_id(&self, conn: &mut PgConnection, search_id: Uuid) -> Result<Organization> {
        organizations
            .find(search_id)
            .filter(deleted_at.is_null())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    warn!(
                        error_code = %ErrorCode::NotFound,
                        organization_id = %search_id,
                        "Organization not found"
                    );
                    ApiError::not_found(format!("Organization with id {} not found", search_id))
                }
                _ => {
                    error!(
                        error_code = %ErrorCode::DatabaseError,
                        organization_id = %search_id,
                        error = %e,
                        "Database error occurred while finding organization"
                    );
                    ApiError::database_error("Failed to find organization", Some(serde_json::json!({
                        "error": e.to_string()
                    })))
                }
            })
    }

    /// Creates a new organization
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `org` - Organization to create
    async fn create(&self, conn: &mut PgConnection, org: &Organization) -> Result<Organization> {
        diesel::insert_into(organizations)
            .values(org)
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                    ApiError::validation_with_context(
                        "Organization name already exists",
                        ErrorContext::new().with_details(serde_json::json!({
                            "field": "name",
                            "code": "DUPLICATE",
                            "value": org.name
                        }))
                    )
                }
                _ => {
                    error!(
                        error_code = %ErrorCode::DatabaseError,
                        error = %e,
                        "Failed to create organization"
                    );
                    ApiError::database_error("Failed to create organization", None)
                }
            })
    }

    /// Updates an existing organization
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `search_id` - Organization ID to update
    /// * `org` - Updated organization data
    async fn update(&self, conn: &mut PgConnection, search_id: Uuid, org: &Organization) -> Result<Organization> {
        diesel::update(organizations.find(search_id))
            .set(org)
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    ApiError::not_found(format!("Organization with id {} not found", search_id))
                }
                diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                    ApiError::validation_with_context(
                        "Organization name already exists",
                        ErrorContext::new().with_details(serde_json::json!({
                            "field": "name",
                            "code": "DUPLICATE",
                            "value": org.name
                        }))
                    )
                }
                _ => {
                    error!(
                        error_code = %ErrorCode::DatabaseError,
                        error = %e,
                        "Failed to update organization"
                    );
                    ApiError::database_error("Failed to update organization", None)
                }
            })
    }

    /// Soft deletes an organization
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `search_id` - Organization ID to delete
    async fn soft_delete(&self, conn: &mut PgConnection, search_id: Uuid) -> Result<Organization> {
        diesel::update(organizations.find(search_id))
            .set(deleted_at.eq(Some(Utc::now())))
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    ApiError::not_found(format!("Organization with id {} not found", search_id))
                }
                _ => {
                    error!(
                        error_code = %ErrorCode::DatabaseError,
                        error = %e,
                        "Failed to delete organization"
                    );
                    ApiError::database_error("Failed to delete organization", None)
                }
            })
    }

    /// Lists organizations with pagination
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `pagination` - Pagination parameters
    async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<Organization>> {
        let offset = (pagination.page - 1) * pagination.per_page;
        
        organizations
            .filter(deleted_at.is_null())
            .offset(offset)
            .limit(pagination.per_page)
            .load(conn)
            .map_err(|e| {
                error!(
                    error_code = %ErrorCode::DatabaseError,
                    error = %e,
                    "Database error occurred while listing organizations"
                );
                ApiError::database_error("Failed to list organizations", Some(serde_json::json!({
                    "error": e.to_string()
                })))
            })
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    /// Finds an organization by name
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `search_name` - Organization name to find
    async fn find_by_name(&self, conn: &mut PgConnection, search_name: &str) -> Result<Option<Organization>> {
        organizations
            .filter(name.eq(search_name))
            .filter(deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(|e| {
                error!(
                    error_code = %ErrorCode::DatabaseError,
                    error = %e,
                    "Failed to find organization by name"
                );
                ApiError::database_error("Failed to find organization by name", None)
            })
    }
}
