use crate::db::{get_connection, DbPool};
use crate::error::AppError;
use crate::models::Organization;
use crate::repositories::organization_repository;
use actix_web::{web, HttpResponse};
use log::{debug, info};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateOrganizationInput {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ListOrganizationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/api/organizations/{id}",
    responses(
        (status = 200, description = "Organization found", body = Organization),
        (status = 404, description = "Organization not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Organization ID")
    )
)]
pub async fn get_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    debug!(
        "Attempting to retrieve organization with id: {}",
        organization_id
    );

    let mut conn = get_connection(&pool)?;
    let org_id = *organization_id;

    match organization_repository::get_organization_by_id(&mut conn, org_id) {
        Ok(organization) => {
            info!("Retrieved organization: {}", organization.id);
            Ok(HttpResponse::Ok().json(organization))
        }
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "/api/organizations",
    request_body = CreateOrganizationInput,
    responses(
        (status = 201, description = "Organization created", body = Organization),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_organization(
    pool: web::Data<DbPool>,
    new_organization: web::Json<CreateOrganizationInput>,
) -> Result<HttpResponse, AppError> {
    debug!(
        "Attempting to create new organization: {}",
        new_organization.name
    );

    let mut conn = get_connection(&pool)?;

    match organization_repository::create_organization(&mut conn, &new_organization.into_inner()) {
        Ok(organization) => {
            info!("Created new organization: {}", organization.id);
            Ok(HttpResponse::Created().json(organization))
        }
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    put,
    path = "/api/organizations/{id}",
    request_body = Organization,
    responses(
        (status = 200, description = "Organization updated", body = Organization),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Organization ID")
    )
)]
pub async fn update_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
    updated_organization: web::Json<Organization>,
) -> Result<HttpResponse, AppError> {
    debug!(
        "Attempting to update organization with id: {}",
        organization_id
    );

    let mut conn = get_connection(&pool)?;
    let org_id = *organization_id;

    match organization_repository::update_organization(
        &mut conn,
        org_id,
        &updated_organization.name,
    ) {
        Ok(organization) => {
            info!("Updated organization: {}", organization.id);
            Ok(HttpResponse::Ok().json(organization))
        }
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/organizations/{id}",
    responses(
        (status = 204, description = "Organization deleted"),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Organization ID")
    )
)]
pub async fn delete_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    debug!(
        "Attempting to delete organization with id: {}",
        organization_id
    );

    let mut conn = get_connection(&pool)?;
    let org_id = *organization_id;

    match organization_repository::delete_organization(&mut conn, org_id) {
        Ok(_) => {
            info!("Deleted organization: {}", org_id);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/organizations",
    responses(
        (status = 200, description = "List of organizations", body = [Organization]),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("limit" = Option<i64>, Query, description = "Limit the number of organizations"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination")
    )
)]
pub async fn list_organizations(
    pool: web::Data<DbPool>,
    query: web::Query<ListOrganizationsQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let mut conn = get_connection(&pool)?;

    match organization_repository::list_organizations(&mut conn, limit, offset) {
        Ok(organizations) => Ok(HttpResponse::Ok().json(organizations)),
        Err(e) => Err(e),
    }
}
