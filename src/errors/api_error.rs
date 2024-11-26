use super::{ErrorCode, ErrorContext};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use std::error::Error as StdError;
use tracing::error;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(flatten)]
    pub context: ErrorContext,
}

/// Implementation of common error creation methods
#[allow(dead_code)]
impl ApiError {
    pub fn new(code: ErrorCode, message: impl Into<String>, context: ErrorContext) -> Self {
        Self {
            code,
            message: message.into(),
            context,
        }
    }

    /// Creates a validation error with optional details
    pub fn validation(message: impl Into<String>, details: Option<serde_json::Value>) -> Self {
        Self::new(
            ErrorCode::ValidationError,
            message,
            ErrorContext::default().with_details(details.unwrap_or_default())
        )
    }

    /// Creates a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(
            ErrorCode::NotFound,
            message,
            ErrorContext::default()
        )
    }

    /// Creates a database error with optional details
    pub fn database_error(message: impl Into<String>, details: Option<serde_json::Value>) -> Self {
        Self::new(
            ErrorCode::DatabaseError,
            message,
            ErrorContext::default().with_details(details.unwrap_or_default())
        )
    }

    /// Creates a configuration error
    pub fn configuration_error(message: impl Into<String>) -> Self {
        Self::new(
            ErrorCode::ConfigurationError,
            message,
            ErrorContext::default()
        )
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        if matches!(self.code, ErrorCode::InternalError | ErrorCode::DatabaseError) {
            error!(
                error_code = %self.code,
                error_message = %self.message,
                "Internal server error occurred"
            );
        }

        HttpResponse::build(self.status_code())
            .json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self.code {
            ErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => StatusCode::FORBIDDEN,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::Conflict => StatusCode::CONFLICT,
            ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            ErrorCode::BadGateway => StatusCode::BAD_GATEWAY,
            ErrorCode::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ErrorCode::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl StdError for ApiError {}
