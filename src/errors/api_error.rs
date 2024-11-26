use crate::config::{Config, Environment};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use tracing::error;
use sentry::capture_error;
use serde::Serialize;
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] diesel::result::Error),

    #[error("Connection pool error")]
    R2D2(#[from] r2d2::Error),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    NotFound(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[allow(dead_code)]
    #[error("Validation error: {0}")]
    Validation(String),

    #[allow(dead_code)]
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[allow(dead_code)]
    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[allow(dead_code)]
    #[error("Resource conflict: {0}")]
    Conflict(String),

    #[allow(dead_code)]
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[allow(dead_code)]
    #[error("Bad gateway: {0}")]
    BadGateway(String),

    #[allow(dead_code)]
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[allow(dead_code)]
    #[error("Request timeout: {0}")]
    RequestTimeout(String),

    #[allow(dead_code)]
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[allow(dead_code)]
    #[error("Too many requests: {0}")]
    TooManyRequests(String),
}

impl AppError {
    #[allow(dead_code)]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    #[allow(dead_code)]
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized(message.into())
    }

    #[allow(dead_code)]
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    #[allow(dead_code)]
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    #[allow(dead_code)]
    pub fn rate_limit_exceeded(message: impl Into<String>) -> Self {
        Self::RateLimitExceeded(message.into())
    }

    #[allow(dead_code)]
    pub fn bad_gateway(message: impl Into<String>) -> Self {
        Self::BadGateway(message.into())
    }

    #[allow(dead_code)]
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable(message.into())
    }

    #[allow(dead_code)]
    pub fn request_timeout(message: impl Into<String>) -> Self {
        Self::RequestTimeout(message.into())
    }

    #[allow(dead_code)]
    pub fn unprocessable_entity(message: impl Into<String>) -> Self {
        Self::UnprocessableEntity(message.into())
    }

    #[allow(dead_code)]
    pub fn too_many_requests(message: impl Into<String>) -> Self {
        Self::TooManyRequests(message.into())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Database(_) | AppError::R2D2(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::RateLimitExceeded(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Log and send error to Sentry for all errors except NotFound and Validation
        if !matches!(self, AppError::NotFound(_) | AppError::Validation(_)) {
            error!("Error occurred: {}", self);

            if let Ok(config) = Config::load() {
                if config.environment != Environment::Development {
                    capture_error(self);
                }
            }
        }

        let status = self.status_code();
        let details = match self {
            AppError::Database(_) | AppError::R2D2(_) => {
                Some("Database is temporarily unavailable".to_string())
            }
            _ => None,
        };

        let error_response = ErrorResponse {
            status: "error",
            message: self.to_string(),
            code: status.as_u16(),
            details,
        };

        HttpResponse::build(status)
            .content_type("application/json")
            .json(error_response)
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl ApiError {
    pub fn new(code: &str, message: &str, details: Option<serde_json::Value>) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details,
        }
    }

    pub fn validation(message: &str, details: Option<serde_json::Value>) -> Self {
        Self::new("VALIDATION_ERROR", message, details)
    }

    pub fn not_found(message: &str) -> Self {
        Self::new("NOT_FOUND", message, None)
    }

    pub fn database_error(message: &str) -> Self {
        Self::new("DATABASE_ERROR", message, None)
    }

    pub fn conflict(message: &str, details: Option<serde_json::Value>) -> Self {
        Self::new("RESOURCE_CONFLICT", message, details)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self.code.as_str() {
            "VALIDATION_ERROR" => StatusCode::BAD_REQUEST,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "DATABASE_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "RESOURCE_CONFLICT" => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AppError> for ApiError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Database(e) => Self::database_error(&e.to_string()),
            AppError::NotFound(msg) => Self::not_found(&msg),
            AppError::Validation(msg) => Self::validation(&msg, None),
            AppError::Conflict(msg) => Self::conflict(&msg, None),
            AppError::R2D2(e) => Self::database_error(&e.to_string()),
            AppError::EnvVar(e) => Self::new("CONFIG_ERROR", &e.to_string(), None),
            AppError::Io(e) => Self::new("IO_ERROR", &e.to_string(), None),
            AppError::Configuration(msg) => Self::new("CONFIG_ERROR", &msg, None),
            _ => Self::new("INTERNAL_SERVER_ERROR", &error.to_string(), None),
        }
    }
}
