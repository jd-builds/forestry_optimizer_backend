use crate::error::{ApiError, ErrorCode, ErrorContext};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AuthError {
    InvalidCredentials(String),
    TokenExpired(String),
    TokenInvalid(String),
    InsufficientPermissions(String),
    SessionExpired(String),
    AccountLocked(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCredentials(msg) => write!(f, "Invalid credentials: {}", msg),
            Self::TokenExpired(msg) => write!(f, "Token expired: {}", msg),
            Self::TokenInvalid(msg) => write!(f, "Invalid token: {}", msg),
            Self::InsufficientPermissions(msg) => write!(f, "Insufficient permissions: {}", msg),
            Self::SessionExpired(msg) => write!(f, "Session expired: {}", msg),
            Self::AccountLocked(msg) => write!(f, "Account locked: {}", msg),
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        let (code, message) = match &error {
            AuthError::InvalidCredentials(_) | 
            AuthError::TokenExpired(_) |
            AuthError::TokenInvalid(_) |
            AuthError::SessionExpired(_) => 
                (ErrorCode::Unauthorized, error.to_string()),
            
            AuthError::InsufficientPermissions(_) => 
                (ErrorCode::Forbidden, error.to_string()),
            
            AuthError::AccountLocked(_) => 
                (ErrorCode::Forbidden, error.to_string()),
        };

        ApiError::new(
            code,
            message,
            ErrorContext::new().with_details(serde_json::json!({
                "error_type": format!("{:?}", error)
            }))
        )
    }
} 