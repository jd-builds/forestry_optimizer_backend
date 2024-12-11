//! Authentication and authorization middleware
//! 
//! This module provides middleware components for:
//! - JWT-based authentication
//! - Role-based authorization
//! - User claims extraction

mod auth;
mod role;

pub use auth::{Auth, AuthenticatedUser};
pub use role::{RequireAuth, RequireRole}; 