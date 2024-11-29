//! Base repository trait defining common CRUD operations
//! 
//! This module provides the base repository trait that all concrete repositories
//! must implement. It defines the standard CRUD operations and common functionality
//! that should be available across all repositories.

use crate::common::{error::Result, pagination::PaginationParams};
use async_trait::async_trait;
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
pub trait Repository<T> {
    async fn create(&self, entity: &T) -> Result<T>;
    async fn find_by_id(&self, id: Uuid) -> Result<T>;
    async fn update(&self, id: Uuid, entity: &T) -> Result<T>;
    async fn soft_delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self, pagination: &PaginationParams) -> Result<(Vec<T>, i64)>;
}
