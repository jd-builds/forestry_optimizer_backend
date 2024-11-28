//! Authentication handlers implementation
//! 
//! This module provides handlers for authentication-related endpoints including
//! login, registration, token refresh, and password reset.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::{
    db::DbPool,
    services::auth::AuthService,
    api::types::responses::ApiResponseBuilder,
    errors::Result,
};

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Registration request payload
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub org_id: Uuid,
}

/// Token refresh request payload
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Authentication response payload
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

/// User response payload
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: String,
    pub org_id: Uuid,
}

/// Login handler
pub async fn login(
    pool: web::Data<DbPool>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let (access_token, refresh_token, user) = AuthService::login(
        &pool,
        &req.email,
        &req.password,
    ).await?;

    let response = AuthResponse {
        access_token,
        refresh_token: refresh_token.token,
        user: UserResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            phone_number: user.phone_number,
            role: format!("{:?}", user.role),
            org_id: user.org_id,
        },
    };

    Ok(HttpResponse::Ok().json(
        ApiResponseBuilder::success()
            .with_message("Login successful")
            .with_data(response)
            .build()
    ))
}

/// Registration handler
pub async fn register(
    pool: web::Data<DbPool>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let user = AuthService::register(
        &pool,
        &req.first_name,
        &req.last_name,
        &req.email,
        &req.phone_number,
        &req.password,
        req.org_id,
    ).await?;

    let response = UserResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        phone_number: user.phone_number,
        role: format!("{:?}", user.role),
        org_id: user.org_id,
    };

    Ok(HttpResponse::Created().json(
        ApiResponseBuilder::success()
            .with_message("Registration successful")
            .with_data(response)
            .build()
    ))
}

/// Token refresh handler
pub async fn refresh(
    pool: web::Data<DbPool>,
    req: web::Json<RefreshRequest>,
) -> Result<HttpResponse> {
    let (access_token, refresh_token) = AuthService::refresh_token(
        &pool,
        &req.refresh_token,
    ).await?;

    Ok(HttpResponse::Ok().json(
        ApiResponseBuilder::success()
            .with_message("Token refreshed")
            .with_data(json!({
                "access_token": access_token,
                "refresh_token": refresh_token.token,
            }))
            .build()
    ))
} 