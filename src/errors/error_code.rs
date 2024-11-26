use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    // Authentication/Authorization
    Unauthorized,
    Forbidden,
    
    // Resource errors
    NotFound,
    Conflict,
    
    // Validation
    ValidationError,
    UnprocessableEntity,
    
    // Infrastructure
    DatabaseError,
    ConnectionPoolError,
    ConfigurationError,
    IoError,
    
    // Rate limiting
    RateLimitExceeded,
    
    // External services
    BadGateway,
    ServiceUnavailable,
    RequestTimeout,
    
    // Other
    InternalError,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::UnprocessableEntity => "UNPROCESSABLE_ENTITY",
            Self::DatabaseError => "DATABASE_ERROR",
            Self::ConnectionPoolError => "CONNECTION_POOL_ERROR",
            Self::ConfigurationError => "CONFIGURATION_ERROR",
            Self::IoError => "IO_ERROR",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::BadGateway => "BAD_GATEWAY",
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            Self::RequestTimeout => "REQUEST_TIMEOUT",
            Self::InternalError => "INTERNAL_ERROR",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
