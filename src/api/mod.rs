//! API module containing all HTTP-related functionality
//! 
//! This module is organized into the following submodules:
//! - `handlers`: Request handlers for different resources
//! - `routes`: Route definitions and configuration
//! - `models`: Common types used across the API layer

pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod models;

// Re-export commonly used models
pub use models::{
    organization::{CreateOrganizationInput, OrganizationDto, UpdateOrganizationInput},
    pagination::{PaginatedResponse, PaginationParams},
    responses::{ApiResponse, ApiResponseBuilder, ErrorResponse},
};

// Re-export route configuration
pub use routes::configure_routes;
