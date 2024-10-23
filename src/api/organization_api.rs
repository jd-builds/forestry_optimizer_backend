use actix_web::{web, HttpResponse};
use uuid::Uuid;
use log::{info, error};
use crate::repositories::organization_repository;
use crate::models::Organization;
use crate::db::DbPool;
use crate::error::AppError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOrganizationInput {
    pub name: String,
}

pub async fn get_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;
    
    match organization_repository::get_organization_by_id(&mut conn, organization_id.into_inner()) {
        Ok(organization) => {
            info!("Retrieved organization: {}", organization.id);
            Ok(HttpResponse::Ok().json(organization))
        },
        Err(AppError::NotFoundError(_)) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => {
            error!("Failed to get organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        },
    }
}

pub async fn create_organization(
    pool: web::Data<DbPool>,
    new_organization: web::Json<CreateOrganizationInput>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;
    
    match organization_repository::create_organization(&mut conn, &new_organization.into_inner()) {
        Ok(organization) => {
            info!("Created new organization: {}", organization.id);
            Ok(HttpResponse::Created().json(organization))
        },
        Err(e) => {
            error!("Failed to create organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        },
    }
}

pub async fn update_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
    updated_organization: web::Json<Organization>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;
    
    match organization_repository::update_organization(&mut conn, organization_id.into_inner(), &updated_organization.into_inner()) {
        Ok(organization) => {
            info!("Updated organization: {}", organization.id);
            Ok(HttpResponse::Ok().json(organization))
        },
        Err(AppError::NotFoundError(_)) => {
            Ok(HttpResponse::NotFound().finish())
        },
        Err(e) => {
            error!("Failed to update organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        },
    }
}

pub async fn delete_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

    let org_id = *organization_id; // Dereference to get the Uuid value
    
    match organization_repository::delete_organization(&mut conn, org_id) {
        Ok(_) => {
            info!("Deleted organization: {}", org_id);
            Ok(HttpResponse::NoContent().finish())
        },
        Err(AppError::NotFoundError(_)) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => {
            error!("Failed to delete organization: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        },
    }
}
