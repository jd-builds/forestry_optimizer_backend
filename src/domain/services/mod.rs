//! Domain service traits
//! 
//! This module contains trait definitions for all domain services.
//! These services encapsulate core business logic and operations.

pub mod auth;
pub mod organization;

pub use auth::AuthService;
pub use organization::OrganizationService;