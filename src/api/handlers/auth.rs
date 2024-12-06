//! Authentication handlers implementation
//! 
//! This module provides handlers for authentication-related endpoints including
//! login, registration, token refresh, and password reset.

use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::{
    api::dto::{auth::{AuthResponse, LoginRequest, RefreshRequest, RegisterRequest, UserResponse}, ApiResponseBuilder, ErrorResponse}, config::Config, database::DbPool, domain::auth::AuthService, error::Result
};
use tracing::info;

/// Login handler
/// 
/// Authenticates a user and returns tokens
#[utoipa::path(
    post,
    path = "/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "auth"
)]
pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let service_response = AuthService::login(
        &pool,
        &req.email,
        &req.password,
        &config,
    ).await?;

    let (access_token, refresh_token, user) = service_response.data;

    info!(
        user_id = %user.id,
        "User logged in successfully"
    );

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
/// 
/// Registers a new user
#[utoipa::path(
    post,
    path = "/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = UserResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "auth"
)]
pub async fn register(
    pool: web::Data<DbPool>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let service_response = AuthService::register(
        &pool,
        &req.first_name,
        &req.last_name,
        &req.email,
        &req.phone_number,
        &req.password,
        req.org_id,
    ).await?;

    let user = service_response.data;

    info!(
        user_id = %user.id,
        "New user registered"
    );

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
/// 
/// Refreshes an access token using a refresh token
#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = AuthResponse),
        (status = 401, description = "Invalid refresh token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "auth"
)]
pub async fn refresh(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    req: web::Json<RefreshRequest>,
) -> Result<HttpResponse> {
    let service_response = AuthService::refresh_token(
        &pool,
        &req.refresh_token,
        &config,
    ).await?;

    let (access_token, refresh_token) = service_response.data;

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