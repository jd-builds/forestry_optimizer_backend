use crate::db::DbPool;
use crate::error::AppError;
use crate::models::Organization;
use crate::repositories::organization_repository;
use actix_web::{web, HttpResponse};
use log::{debug, error, info};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateOrganizationInput {
    pub name: String,
}

#[derive(Deserialize)]
pub struct ListOrganizationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn get_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "Attempting to retrieve organization with id: {}",
        organization_id
    );

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

    let org_id = *organization_id;

    match organization_repository::get_organization_by_id(&mut conn, org_id) {
        Ok(organization) => {
            info!("Retrieved organization: {}", organization.id);
            Ok(HttpResponse::Ok().json(organization))
        }
        Err(AppError::NotFound(_)) => {
            info!("Organization not found: {}", org_id);
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            error!("Failed to get organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn create_organization(
    pool: web::Data<DbPool>,
    new_organization: web::Json<CreateOrganizationInput>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "Attempting to create new organization: {}",
        new_organization.name
    );

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

    match organization_repository::create_organization(&mut conn, &new_organization.into_inner()) {
        Ok(organization) => {
            info!("Created new organization: {}", organization.id);
            Ok(HttpResponse::Created().json(organization))
        }
        Err(e) => {
            error!("Failed to create organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn update_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
    updated_organization: web::Json<Organization>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "Attempting to update organization with id: {}",
        organization_id
    );

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

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
        Err(AppError::NotFound(_)) => {
            info!("Organization not found: {}", org_id);
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            error!("Failed to update organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn delete_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "Attempting to delete organization with id: {}",
        organization_id
    );

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

    let org_id = *organization_id;

    match organization_repository::delete_organization(&mut conn, org_id) {
        Ok(_) => {
            info!("Deleted organization: {}", org_id);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(AppError::NotFound(_)) => {
            info!("Organization not found: {}", org_id);
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            error!("Failed to delete organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn list_organizations(
    pool: web::Data<DbPool>,
    query: web::Query<ListOrganizationsQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

    match organization_repository::list_organizations(&mut conn, limit, offset) {
        Ok(organizations) => Ok(HttpResponse::Ok().json(organizations)),
        Err(e) => {
            error!("Failed to list organizations: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
