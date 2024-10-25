use crate::config::{Config, Environment};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use sentry::capture_error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("R2D2 error: {0}")]
    R2D2(#[from] r2d2::Error),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Log and send error to Sentry for all errors except NotFound
        if !matches!(self, AppError::NotFound(_)) {
            error!("Error occurred: {}", self);

            // Only send to Sentry if not in development environment
            match Config::load() {
                Ok(config) => {
                    if config.environment != Environment::Development {
                        capture_error(self);
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to load configuration, error not sent to Sentry: {}",
                        e
                    );
                }
            }
        }

        // Create the response with error details in the body
        let error_message = format!("{{\"error\": \"{}\"}}", self);
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(error_message)
    }
}

pub type AppResult<T> = Result<T, AppError>;
