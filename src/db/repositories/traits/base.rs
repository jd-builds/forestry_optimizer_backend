//! Base repository trait defining common CRUD operations
//! 
//! This module provides the base repository trait that all concrete repositories
//! must implement. It defines the standard CRUD operations and common functionality
//! that should be available across all repositories.

use crate::{
    api::types::pagination::PaginationParams,
    errors::Result,
};
use async_trait::async_trait;
use diesel::PgConnection;
use uuid::Uuid;

/// Base repository trait for database operations
/// 
/// This trait defines the standard CRUD operations that all repositories
/// must implement. It provides a consistent interface for database operations
/// across different model types.
/// 
/// # Type Parameters
/// 
/// * `M` - The model type this repository handles
#[async_trait]
pub trait Repository<M>: Send + Sync + 'static {
    /// Finds a model by its unique identifier
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<M>;

    /// Creates a new model in the database
    async fn create(&self, conn: &mut PgConnection, model: &M) -> Result<M>;

    /// Updates an existing model in the database
    async fn update(&self, conn: &mut PgConnection, id: Uuid, model: &M) -> Result<M>;

    /// Soft deletes a model from the database
    async fn soft_delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<M>;

    /// Lists models with pagination
    async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<M>>;
}
