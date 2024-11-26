use crate::{
    api::types::pagination::PaginationParams,
    db::{
        models::Organization,
        repositories::traits::{OrganizationRepository, Repository},
        schema::organizations::dsl::*,
    },
    errors::{ApiError, ErrorCode, ErrorContext, Result},
};
use chrono::Utc;
use diesel::prelude::*;
use tracing::{error, debug};
use uuid::Uuid;

pub struct OrganizationRepositoryImpl;

impl Repository<Organization> for OrganizationRepositoryImpl {
    fn find_by_id(&self, conn: &mut PgConnection, search_id: Uuid) -> Result<Organization> {
        debug!(organization_id = %search_id, "Executing find by ID query");
        
        organizations
            .find(search_id)
            .filter(deleted_at.is_null())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::new(
                    ErrorCode::NotFound,
                    format!("Organization with id {} not found", search_id),
                    ErrorContext::default(),
                ),
                _ => {
                    error!(error = %e, "Database error during find by ID");
                    ApiError::new(
                        ErrorCode::DatabaseError,
                        "Database error occurred",
                        ErrorContext::new().with_details(serde_json::json!({
                            "error": e.to_string()
                        }))
                    )
                }
            })
    }

    fn create(&self, conn: &mut PgConnection, org: &Organization) -> Result<Organization> {
        debug!("Executing create organization query");
        
        diesel::insert_into(organizations)
            .values(org)
            .get_result(conn)
            .map_err(|e| {
                error!(error = %e, "Database error during create");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to create organization",
                    ErrorContext::new().with_details(serde_json::json!({
                        "error": e.to_string()
                    }))
                )
            })
    }

    fn update(&self, conn: &mut PgConnection, search_id: Uuid, org: &Organization) -> Result<Organization> {
        debug!(organization_id = %search_id, "Executing update query");
        
        diesel::update(organizations.find(search_id))
            .set(org)
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::new(
                    ErrorCode::NotFound,
                    format!("Organization with id {} not found", search_id),
                    ErrorContext::default(),
                ),
                _ => {
                    error!(error = %e, "Database error during update");
                    ApiError::new(
                        ErrorCode::DatabaseError,
                        "Failed to update organization",
                        ErrorContext::new().with_details(serde_json::json!({
                            "error": e.to_string()
                        }))
                    )
                }
            })
    }

    fn soft_delete(&self, conn: &mut PgConnection, search_id: Uuid) -> Result<Organization> {
        debug!(organization_id = %search_id, "Executing soft delete query");
        
        diesel::update(organizations.find(search_id))
            .set(deleted_at.eq(Some(Utc::now())))
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::new(
                    ErrorCode::NotFound,
                    format!("Organization with id {} not found", search_id),
                    ErrorContext::default(),
                ),
                _ => {
                    error!(error = %e, "Database error during soft delete");
                    ApiError::new(
                        ErrorCode::DatabaseError,
                        "Failed to delete organization",
                        ErrorContext::new().with_details(serde_json::json!({
                            "error": e.to_string()
                        }))
                    )
                }
            })
    }

    fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<Organization>> {
        debug!(
            page = pagination.page,
            per_page = pagination.per_page,
            "Executing list query"
        );
        
        let offset = (pagination.page - 1) * pagination.per_page;
        organizations
            .filter(deleted_at.is_null())
            .offset(offset)
            .limit(pagination.per_page)
            .load(conn)
            .map_err(|e| {
                error!(error = %e, "Database error during list");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to list organizations",
                    ErrorContext::new().with_details(serde_json::json!({
                        "error": e.to_string()
                    }))
                )
            })
    }
}

impl OrganizationRepository for OrganizationRepositoryImpl {
    fn find_by_name(&self, conn: &mut PgConnection, search_name: &str) -> Result<Option<Organization>> {
        debug!(name = %search_name, "Executing find by name query");
        
        organizations
            .filter(name.eq(search_name))
            .filter(deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(|e| {
                error!(error = %e, "Database error during find by name");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to find organization by name",
                    ErrorContext::new().with_details(serde_json::json!({
                        "error": e.to_string()
                    }))
                )
            })
    }
}
