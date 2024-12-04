//! Organization repository trait
//! 
//! This module defines the organization-specific repository operations
//! that extend the base repository functionality.

use crate::{
    db::models::Organization,
    error::Result,
};
use async_trait::async_trait;
use diesel::PgConnection;

use super::base::Repository;

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
