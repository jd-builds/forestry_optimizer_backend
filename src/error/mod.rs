//! Error handling infrastructure for the application
//! 
//! This module provides a robust error handling system that includes:
//! - Strongly typed error codes for different failure scenarios
//! - Rich error context with metadata and structured details
//! - HTTP integration for proper error responses
//! - Consistent error creation and handling patterns
//! - Specialized error types for common scenarios (database, validation, auth)
//! 
//! The error system is designed around these main components:
//! 1. `ApiError` - The core error type that implements proper HTTP responses
//! 2. `ErrorCode` - Enumeration of all possible error types
//! 3. `ErrorContext` - Additional error metadata and details
//! 4. Common error types - Specialized errors for specific domains
//! 
//! # Example
//! ```
//! use optimizer::error::{ApiError, ErrorCode, ErrorContext};
//! 
//! // Create a validation error with details
//! let error = ApiError::new(
//!     ErrorCode::ValidationError,
//!     "Invalid input",
//!     ErrorContext::new().with_details(serde_json::json!({
//!         "field": "email",
//!         "reason": "invalid format"
//!     }))
//! );
//! ```

mod api;
mod code;
mod context;
pub mod common;

pub use api::ApiError;
pub use code::ErrorCode;
pub use context::ErrorContext;
pub use common::{DatabaseError, ValidationError, AuthError};

/// Type alias for Results that use ApiError as the error type
pub type Result<T> = std::result::Result<T, ApiError>;
