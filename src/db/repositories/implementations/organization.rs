use crate::{
    api::types::pagination::PaginationParams,
    db::{
        models::Organization,
        repositories::traits::{OrganizationRepository, Repository},
        schema::organizations::dsl::*,
    },
    errors::{ApiError, Result},
};
use chrono::Utc;
use diesel::prelude::*;
use tracing::error;
use uuid::Uuid;
use async_trait::async_trait;

pub struct OrganizationRepositoryImpl;

#[async_trait]
impl Repository<Organization> for OrganizationRepositoryImpl {
    async fn find_by_id(&self, conn: &mut PgConnection, search_id: Uuid) -> Result<Organization> {
        organizations
            .find(search_id)
            .filter(deleted_at.is_null())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::not_found(format!("Organization with id {} not found", search_id)),
                _ => {
                    error!("Database error occurred");
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
            .map_err(|e| {
                error!("Database error occurred");
                ApiError::database_error("Failed to create organization", Some(serde_json::json!({
                    "error": e.to_string()
                })))
            })
    }

    async fn update(&self, conn: &mut PgConnection, search_id: Uuid, org: &Organization) -> Result<Organization> {
        diesel::update(organizations.find(search_id))
            .set(org)
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::not_found(format!("Organization with id {} not found", search_id)),
                _ => {
                    error!("Database error occurred");
                    ApiError::database_error("Failed to update organization", Some(serde_json::json!({
                        "error": e.to_string()
                    })))
                }
            })
    }

    async fn soft_delete(&self, conn: &mut PgConnection, search_id: Uuid) -> Result<Organization> {
        diesel::update(organizations.find(search_id))
            .set(deleted_at.eq(Some(Utc::now())))
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::not_found(format!("Organization with id {} not found", search_id)),
                _ => {
                    error!("Database error occurred");
                    ApiError::database_error("Failed to delete organization", Some(serde_json::json!({
                        "error": e.to_string()
                    })))
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
                error!("Database error occurred");
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
                error!(error = %e, "Database error");
                ApiError::database_error("Failed to find organization by name", Some(serde_json::json!({
                    "error": e.to_string()
                })))
            })
    }
}
