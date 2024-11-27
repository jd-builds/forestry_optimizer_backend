//! Organization resource routes
//! 
//! This module defines the routes for the organization resource.
//! It provides RESTful endpoints for CRUD operations on organizations.

use crate::api::handlers::organization::{create, delete, read, update};
use actix_web::{web, Scope};

/// Configures and returns the organization routes
/// 
/// Sets up the following endpoints:
/// - POST /organizations - Create a new organization
/// - GET /organizations/{id} - Get a specific organization
/// - PUT /organizations/{id} - Update an organization
/// - DELETE /organizations/{id} - Delete an organization
/// - GET /organizations - List all organizations
/// 
/// # Returns
/// 
/// Returns a configured Scope containing all organization routes
#[allow(clippy::module_name_repetitions)]
pub fn routes() -> Scope {
    web::scope("/organizations")
        // Create organization
        .route(
            "",
            web::post().to(create::create_organization)
        )
        // Get organization by ID
        .route(
            "/{id}",
            web::get().to(read::get_organization)
        )
        // Update organization
        .route(
            "/{id}",
            web::put().to(update::update_organization)
        )
        // Delete organization
        .route(
            "/{id}",
            web::delete().to(delete::delete_organization)
        )
        // List organizations
        .route(
            "",
            web::get().to(read::list_organizations)
        )
}
