use crate::config::{Config, Environment};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use sentry::capture_error;
use serde::Serialize;
use thiserror::Error;

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

    #[error("Validation error: {0}")]
    Validation(String),

    #[allow(dead_code)]
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[allow(dead_code)]
    #[error("Forbidden: {0}")]
    Forbidden(String),

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
