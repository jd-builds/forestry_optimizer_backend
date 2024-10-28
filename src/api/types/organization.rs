use crate::db::models::Organization;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateOrganizationInput {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateOrganizationInput {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ListOrganizationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub data: Organization,
}

#[derive(Serialize, ToSchema)]
pub struct OrganizationListResponse {
    pub data: Vec<Organization>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
