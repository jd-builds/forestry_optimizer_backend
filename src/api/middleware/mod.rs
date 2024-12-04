//! Middleware components for request processing
//! 
//! This module provides middleware components that handle
//! cross-cutting concerns in the request processing pipeline.

pub mod auth;
pub mod rate_limit;
pub mod request_id;
pub mod security;
pub mod validation;

// Re-export commonly used middleware
pub use auth::{Auth, AuthenticatedUser, RequireAuth, RequireRole};
pub use rate_limit::RateLimit;
pub use request_id::RequestId;
pub use security::SecurityHeaders;
pub use validation::RequestValidate;