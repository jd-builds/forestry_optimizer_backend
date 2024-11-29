use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tracing::{info, error};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    domain::services::AuthService,
    common::error::Result,
    domain::models::user::Role,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub org_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    auth_service: web::Data<dyn AuthService>,
    request: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    info!("Handling register request for email: {}", request.email);
    
    let token = auth_service.register_user(
        request.first_name.clone(),
        request.last_name.clone(),
        request.email.clone(),
        request.phone_number.clone(),
        request.password.clone(),
        request.org_id,
        Role::Manager,
    ).await?;

    Ok(HttpResponse::Created().json(token))
}

pub async fn login(
    auth_service: web::Data<dyn AuthService>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    info!("Handling login request for email: {}", request.email);
    
    let token = auth_service.login(
        request.email.clone(),
        request.password.clone(),
    ).await?;

    Ok(HttpResponse::Ok().json(token))
} 