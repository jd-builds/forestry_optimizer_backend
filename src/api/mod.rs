//! API module containing all HTTP-related functionality
//! 
//! This module is organized into the following submodules:
//! - `dto`: Data Transfer Objects (DTOs) for request and response payloads
//! - `handlers`: Request handlers for different resources
//! - `middleware`: Middleware for request processing
//! - `routes`: Route definitions and configuration

pub mod resources;
pub mod middleware;
pub mod utils;

// Re-export commonly used models
pub use resources::organization::dto::{CreateOrganizationInput, UpdateOrganizationInput};

// Re-export route configuration
pub use resources::docs::routes::configure;
