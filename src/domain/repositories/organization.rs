//! Organization repository trait
//! 
//! This module defines the organization-specific repository operations
//! that extend the base repository functionality.

use crate::{
    common::error::Result,
    domain::models::organization::Organization,
};
use super::base::Repository;
use async_trait::async_trait;

/// Repository trait for Organization model
#[async_trait]
pub trait OrganizationRepository: Repository<Organization> {
    async fn find_by_name(&self, name: &str) -> Result<Option<Organization>>;
}
