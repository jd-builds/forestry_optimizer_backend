use crate::{
    api::utils::PaginationParams,
    database::{
        models::Organization,
        repositories::Repository,
        schema::organizations::dsl::*,
    },
    error::{ApiError, ErrorCode, ErrorContext, Result},
};
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use tracing::{error, warn};
use uuid::Uuid;

/// Organization-specific repository operations
/// 
/// This trait extends the base Repository trait with operations specific
/// to the Organization model. It provides additional query methods and
/// business logic specific to organizations.
#[async_trait]
pub trait OrganizationRepository: Repository<Organization> {
    /// Finds an organization by its name
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `name` - Name of the organization to find
    /// 
    /// # Returns
    /// 
    /// Returns Some(Organization) if found, None if not found
    async fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> Result<Option<Organization>>;
}

/// Concrete implementation of the organization repository
pub struct OrganizationRepositoryImpl;

#[async_trait]
impl Repository<Organization> for OrganizationRepositoryImpl {
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