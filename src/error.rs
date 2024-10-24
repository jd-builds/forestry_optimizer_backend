use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;
use sentry::capture_error;
use log::error;
use std::env;

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

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = actix_web::HttpResponse::new(self.status_code());
        
        // Log and send error to Sentry for all errors except NotFound
        if !matches!(self, AppError::NotFound(_)) {
            error!("Error occurred: {}", self);
            
            // Only send to Sentry if not in development environment
            if env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()) != "development" {
                capture_error(self);
            }
        }

        response
    }
}

pub type AppResult<T> = Result<T, AppError>;
