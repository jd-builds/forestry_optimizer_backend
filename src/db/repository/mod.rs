use crate::{
    api::types::pagination::PaginationParams,
    error::Result,
};
use async_trait::async_trait;
use diesel::PgConnection;
use uuid::Uuid;

pub mod organization;
pub mod auth;

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

pub use organization::{OrganizationRepository, OrganizationRepositoryImpl};
pub use auth::{
    UserRepository,
    UserRepositoryImpl,
    RefreshTokenRepository,
    RefreshTokenRepositoryImpl,
    PasswordResetTokenRepository,
    EmailVerificationTokenRepository,
};
