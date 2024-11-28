//! Authentication routes implementation
//! 
//! This module defines the routes for authentication-related endpoints.

use actix_web::{web, Scope};
use crate::api::handlers::auth;

/// Configures and returns the authentication routes
/// 
/// Sets up the following endpoints:
/// - POST /auth/login - User login
/// - POST /auth/register - User registration
/// - POST /auth/refresh - Token refresh
/// 
/// # Returns
/// 
/// Returns a configured Scope containing all authentication routes
pub fn routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(auth::login))
        .route("/register", web::post().to(auth::register))
        .route("/refresh", web::post().to(auth::refresh))
} 