use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::repositories::organization_repository;
use crate::models::Organization;
use crate::db::DbPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOrganizationInput {
    pub name: String,
}

pub async fn get_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    
    match organization_repository::get_organization_by_id(&mut conn, organization_id.into_inner()) {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_organization(
    pool: web::Data<DbPool>,
    new_organization: web::Json<CreateOrganizationInput>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    
    match organization_repository::create_organization(&mut conn, &new_organization.into_inner()) {
        Ok(organization) => HttpResponse::Created().json(organization),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
    updated_organization: web::Json<Organization>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    
    match organization_repository::update_organization(&mut conn, organization_id.into_inner(), &updated_organization.into_inner()) {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_organization(
    pool: web::Data<DbPool>,
    organization_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    
    match organization_repository::delete_organization(&mut conn, organization_id.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
