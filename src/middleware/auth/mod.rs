//! Authentication and authorization middleware
//! 
//! This module provides middleware components for:
//! - JWT-based authentication
//! - Role-based authorization
//! - User claims extraction

mod auth_middleware;
mod role_middleware;

pub use auth_middleware::{Auth, AuthenticatedUser};
pub use role_middleware::{RequireAuth, RequireRole}; 