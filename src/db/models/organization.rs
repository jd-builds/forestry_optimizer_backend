//! Organization model and implementation
//! 
//! This module defines the Organization model and implements the necessary traits
//! for database operations. It provides a robust foundation for managing organization
//! data with proper error handling and validation.

use super::{BaseModel, Timestamps};
use crate::{db::schema::organizations, error::{Result, ApiError, ErrorCode}};
use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};
use tracing::{error, warn};
use utoipa::ToSchema;
use uuid::Uuid;

/// Represents an organization in the system
/// 
/// This model serves as the core entity for managing organizations. It includes
/// all necessary fields for tracking organization data and implements soft deletion
/// for data retention.
/// 
/// # Fields
/// 
/// * `id` - Unique identifier for the organization
/// * `name` - Organization's display name
/// * `created_at` - Timestamp of when the organization was created
/// * `updated_at` - Timestamp of the last update
/// * `deleted_at` - Optional timestamp for soft deletion
#[derive(
    Debug,
    Default,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Timestamps for Organization {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}

impl BaseModel for Organization {
    type Table = organizations::table;

    fn id(&self) -> Uuid {
        self.id
    }

    fn table() -> Self::Table {
        organizations::table
    }

    /// Finds an organization by its unique identifier
    /// 
    /// This method performs a database query to find an organization by ID,
    /// excluding soft-deleted records. It includes proper error handling and
    /// logging for both not found and database error cases.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `id` - Organization's unique identifier
    /// 
    /// # Returns
    /// 
    /// Returns the organization if found, otherwise returns an appropriate error
    fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Self> {
        use diesel::QueryDsl;
        
        organizations::table
            .find(id)
            .filter(organizations::deleted_at.is_null())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    warn!(
                        error_code = %ErrorCode::NotFound,
                        organization_id = %id,
                        "Organization not found"
                    );
                    ApiError::not_found(format!("Organization with id {} not found", id))
                }
                _ => {
                    error!(
                        error_code = %ErrorCode::DatabaseError,
                        organization_id = %id,
                        error = %e,
                        "Database error occurred while finding organization"
                    );
                    ApiError::database_error("Failed to find organization", Some(serde_json::json!({
                        "error": e.to_string()
                    })))
                }
            })
    }

    fn set_deleted_at(&mut self, timestamp: Option<DateTime<Utc>>) {
        self.deleted_at = timestamp;
    }

    /// Returns the base query for organization operations
    /// 
    /// This method provides the default query filter that excludes
    /// soft-deleted organizations from all queries.
    fn base_query() -> Box<dyn BoxableExpression<Self::Table, Pg, SqlType = diesel::sql_types::Bool>> {
        Box::new(organizations::deleted_at.is_null())
    }
}
