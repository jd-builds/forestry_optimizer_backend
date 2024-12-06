//! API module containing all HTTP-related functionality
//! 
//! This module is organized into the following submodules:
//! - `dto`: Data Transfer Objects (DTOs) for request and response payloads
//! - `handlers`: Request handlers for different resources
//! - `middleware`: Middleware for request processing
//! - `routes`: Route definitions and configuration

pub mod dto;
pub mod handlers;
pub mod middleware;
pub mod routes;

// Re-export commonly used models
pub use dto::organization::{CreateOrganizationInput, UpdateOrganizationInput};

// Re-export route configuration
pub use routes::configure_routes;
