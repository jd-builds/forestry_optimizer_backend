use crate::error::{ApiError, ErrorCode, ErrorContext};
use serde::Serialize;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: Option<String>,
    pub code: ValidationErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub enum ValidationErrorCode {
    Required,
    InvalidFormat,
    TooLong,
    TooShort,
    OutOfRange,
    InvalidValue,
    Custom(String),
}

impl ValidationError {
    pub fn new(code: ValidationErrorCode, message: impl Into<String>) -> Self {
        Self {
            field: None,
            code,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(field) = &self.field {
            write!(f, "{}: {}", field, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(error: ValidationError) -> Self {
        let mut details = HashMap::new();
        if let Some(field) = error.field {
            details.insert("field", serde_json::Value::String(field));
        }
        details.insert("code", serde_json::to_value(&error.code).unwrap());
        if let Some(extra) = error.details {
            details.insert("details", extra);
        }

        ApiError::new(
            ErrorCode::ValidationError,
            error.message,
            ErrorContext::new().with_details(serde_json::to_value(details).unwrap())
        )
    }
} 