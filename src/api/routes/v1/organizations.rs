//! Organization resource routes
//! 
//! This module defines the routes for the organization resource.
//! It provides RESTful endpoints for CRUD operations on organizations.

use crate::api::handlers::organization::{delete, read, update};
use actix_web::web;
use crate::api::middleware::auth::RequireRole;
use crate::db::models::auth::Role;

/// Configures and returns the organization routes
/// 
/// Sets up the following endpoints with role-based authorization:
/// - GET /organizations/{id} - Get a specific organization (All roles)
/// - PUT /organizations/{id} - Update an organization (Admin, Manager)
/// - DELETE /organizations/{id} - Delete an organization (Admin only)
/// - GET /organizations - List all organizations (All roles)
/// 
/// Note: POST /organizations (create) is handled separately as a public route
/// 
/// # Returns
/// Returns a configured Scope containing all organization routes
#[allow(clippy::module_name_repetitions)]
pub fn routes() -> actix_web::Scope<impl actix_web::dev::ServiceFactory<
    actix_web::dev::ServiceRequest,
    Config = (),
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
    InitError = (),
>> {
    web::scope("")
        .service(
            web::resource("")
                .route(web::get().to(read::list_organizations))
        )
        .service(
            web::resource("/{id}")
                .route(web::get().to(read::get_organization))
                .route(web::put().to(update::update_organization).wrap(RequireRole(Role::Manager)))
                .route(web::delete().to(delete::delete_organization).wrap(RequireRole(Role::Admin)))
        )
}
