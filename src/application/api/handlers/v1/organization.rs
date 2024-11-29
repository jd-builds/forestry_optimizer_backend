use actix_web::{web, HttpResponse};
use crate::{
    application::api::types::organization::{
        CreateOrganizationInput, OrganizationDto, UpdateOrganizationInput, OrganizationListResponse
    }, common::{error::Result, pagination::PaginationParams}, domain::services::OrganizationService
};
use uuid::Uuid;
use std::sync::Arc;

/// Create a new organization
#[utoipa::path(
    post,
    path = "/v1/organizations",
    request_body = CreateOrganizationInput,
    responses(
        (status = 201, description = "Organization created successfully", body = OrganizationDto),
        (status = 409, description = "Organization already exists"),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn create_organization(
    service: web::Data<Arc<dyn OrganizationService>>,
    input: web::Json<CreateOrganizationInput>,
) -> Result<HttpResponse> {
    let organization = service.create_organization(input.0.name).await?;
    Ok(HttpResponse::Created().json(OrganizationDto::from(organization)))
}

/// Get organization by ID
#[utoipa::path(
    get,
    path = "/v1/organizations/{id}",
    responses(
        (status = 200, description = "Organization found", body = OrganizationDto),
        (status = 404, description = "Organization not found")
    )
)]
pub async fn get_organization(
    service: web::Data<Arc<dyn OrganizationService>>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let organization = service.get_organization(*id).await?;
    Ok(HttpResponse::Ok().json(OrganizationDto::from(organization)))
}

/// List organizations with pagination
#[utoipa::path(
    get,
    path = "/v1/organizations",
    responses(
        (status = 200, description = "List of organizations", body = OrganizationListResponse)
    )
)]
pub async fn list_organizations(
    service: web::Data<Arc<dyn OrganizationService>>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse> {
    let service = service.into_inner();
    let (organizations, total) = service.list_organizations(pagination.into_inner()).await?;
    Ok(HttpResponse::Ok().json(OrganizationListResponse {
        organizations: organizations.iter().map(|org| OrganizationDto::from(org.clone())).collect(),
        total,
    }))
}

/// Update organization
#[utoipa::path(
    put,
    path = "/v1/organizations/{id}",
    request_body = UpdateOrganizationInput,
    responses(
        (status = 200, description = "Organization updated successfully", body = OrganizationDto),
        (status = 404, description = "Organization not found"),
        (status = 409, description = "Organization name already taken"),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn update_organization(
    service: web::Data<Arc<dyn OrganizationService>>,
    id: web::Path<Uuid>,
    input: web::Json<UpdateOrganizationInput>,
) -> Result<HttpResponse> {
    let organization = service.update_organization(*id, input.0.name).await?;
    Ok(HttpResponse::Ok().json(OrganizationDto::from(organization)))
}

/// Delete organization
#[utoipa::path(
    delete,
    path = "/v1/organizations/{id}",
    responses(
        (status = 204, description = "Organization deleted successfully"),
        (status = 404, description = "Organization not found")
    )
)]
pub async fn delete_organization(
    service: web::Data<Arc<dyn OrganizationService>>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    service.delete_organization(*id).await?;
    Ok(HttpResponse::NoContent().finish())
} 