use crate::common::error::Result;
use crate::common::pagination::PaginationParams;
use crate::domain::models::organization::Organization;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait OrganizationService: Send + Sync {
    async fn create_organization(&self, name: String) -> Result<Organization>;
    async fn get_organization(&self, id: Uuid) -> Result<Organization>;
    async fn update_organization(&self, id: Uuid, name: String) -> Result<Organization>;
    async fn delete_organization(&self, id: Uuid) -> Result<Organization>;
    async fn list_organizations(&self, pagination: PaginationParams) -> Result<(Vec<Organization>, i64)>;
    async fn is_name_available(&self, name: &str) -> Result<bool>;
} 