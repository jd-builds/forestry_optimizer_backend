//! Error code enumeration for the application
//! 
//! This module defines all possible error types that can occur in the application.
//! Error codes are organized by category and map directly to HTTP status codes
//! when returned in API responses.
//! 
//! The error codes follow these principles:
//! - Clear and descriptive names that indicate the error type
//! - Organized into logical categories (auth, validation, infrastructure, etc.)
//! - Map cleanly to standard HTTP status codes
//! - Support proper error handling and recovery

use serde::{Deserialize, Serialize};
use std::fmt;

/// Comprehensive set of error codes for the application
/// 
/// Each variant represents a specific type of error that can occur.
/// The variants are organized into categories and include documentation
/// about when they should be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    // Authentication/Authorization
    /// User is not authenticated
    Unauthorized,
    /// User lacks permission for the requested operation
    Forbidden,
    
    // Resource errors
    /// Requested resource does not exist
    NotFound,
    /// Resource state conflict (e.g., duplicate entry)
    Conflict,
    
    // Validation
    /// Input validation failed
    ValidationError,
    /// Request semantically invalid
    UnprocessableEntity,
    
    // Infrastructure
    /// Database operation failed
    DatabaseError,
    /// Failed to get connection from pool
    ConnectionPoolError,
    /// Application configuration error
    ConfigurationError,
    /// File system or network I/O error
    IoError,
    
    // Rate limiting
    /// Too many requests from client
    RateLimitExceeded,
    
    // External services
    /// Upstream service returned an error
    BadGateway,
    /// Service temporarily unavailable
    ServiceUnavailable,
    /// Request timed out
    RequestTimeout,
    
    // Other
    /// Unexpected internal error
    InternalError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert enum variant to string, replacing underscores with spaces
        let name = format!("{:?}", self)
            .chars()
            .map(|c| if c == '_' { ' ' } else { c })
            .collect::<String>();
        write!(f, "{}", name)
    }
}
