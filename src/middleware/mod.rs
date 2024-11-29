//! Middleware components for request processing
//! 
//! This module provides a collection of middleware components that handle
//! cross-cutting concerns in the request processing pipeline. Each middleware
//! component is focused on a specific aspect of request handling:
//! 
//! - `rate_limit`: Rate limiting to prevent abuse
//! - `request_id`: Request tracking and correlation
//! - `security`: Security headers
//! - `validation`: Request payload validation
//! 
//! # Architecture
//! 
//! The middleware system is built on Actix Web's middleware framework and follows
//! these design principles:
//! 
//! 1. **Single Responsibility**: Each middleware handles one specific concern
//! 2. **Composability**: Middlewares can be combined in any order
//! 3. **Configuration**: Each middleware is configurable for different use cases
//! 4. **Performance**: Minimal overhead in the request processing pipeline
//! 
//! # Example
//! 
//! ```rust
//! use actix_web::App;
//! use crate::middleware::{RateLimit, RequestId, SecurityHeaders};
//! 
//! let app = App::new()
//!     .wrap(RateLimit::new(100, 10))  // 100 requests per 10 seconds
//!     .wrap(RequestId)               // Add request ID to all requests
//!     .wrap(SecurityHeaders)         // Add security headers
//! ```

pub mod auth;
mod rate_limit;
mod request_id;
mod security;
mod validation;

pub use rate_limit::RateLimit;
pub use request_id::RequestId;
pub use security::SecurityHeaders;