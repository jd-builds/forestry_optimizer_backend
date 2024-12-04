//! Base model traits for database entities
//! 
//! This module provides the base traits that all database models must implement.
//! It defines common functionality for timestamps and basic model operations.

use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*, QueryDsl};
use uuid::Uuid;
use crate::error::Result;

/// Trait for handling timestamp fields common across all models
/// 
/// This trait provides a standard interface for accessing and managing
/// timestamp fields that are common to all database models.
#[allow(unused)]
pub trait Timestamps {
    /// Gets the creation timestamp
    fn created_at(&self) -> DateTime<Utc>;
    
    /// Gets the last update timestamp
    fn updated_at(&self) -> DateTime<Utc>;
    
    /// Gets the deletion timestamp, if any
    fn deleted_at(&self) -> Option<DateTime<Utc>>;
    
    /// Checks if the model is marked as deleted
    fn is_deleted(&self) -> bool {
        self.deleted_at().is_some()
    }
}

/// Base model trait providing common functionality for all database models
/// 
/// This trait defines the core functionality that all database models must implement.
/// It provides methods for querying, identifying, and managing model state.
#[allow(dead_code)]
pub trait BaseModel: Sized + Timestamps {
    /// The diesel table type for this model
    type Table: Table + QueryDsl;

    /// Gets the unique identifier of the model
    fn id(&self) -> Uuid;
    
    /// Gets the diesel table associated with this model
    fn table() -> Self::Table;
    
    /// Finds a model by its unique identifier
    /// 
    /// # Arguments
    /// 
    /// * `conn` - Database connection
    /// * `id` - Unique identifier to search for
    /// 
    /// # Returns
    /// 
    /// Returns the model if found, otherwise returns an error
    fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Self>
    where
        Self: Queryable<diesel::sql_types::Record<(
            diesel::sql_types::Uuid,
            diesel::sql_types::Text,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Nullable<diesel::sql_types::Timestamptz>,
        )>, Pg>;
    
    /// Marks the model as deleted
    fn soft_delete(&mut self) {
        self.set_deleted_at(Some(Utc::now()));
    }
    
    /// Sets the deletion timestamp
    fn set_deleted_at(&mut self, timestamp: Option<DateTime<Utc>>);

    /// Gets the base query for this model
    /// 
    /// This method returns a query that includes any default filters
    /// or conditions that should be applied to all queries for this model.
    /// By default, it returns a TRUE condition.
    fn base_query() -> Box<dyn BoxableExpression<Self::Table, Pg, SqlType = diesel::sql_types::Bool>> {
        Box::new(diesel::dsl::sql::<diesel::sql_types::Bool>("TRUE"))
    }
}
