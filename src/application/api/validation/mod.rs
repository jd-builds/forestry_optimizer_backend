use crate::common::error::ApiError;

pub mod request;
pub mod types;

/// Common trait for validatable types
pub trait Validate {
    fn validate(&self) -> Result<(), ApiError>;
}

/// Common validation utilities
pub mod utils {
    use crate::common::error::{ApiError, ErrorCode, ErrorContext};
    use tracing::warn;

    pub fn validate_string_length(
        value: &str,
        field: &str,
        min_length: usize,
        max_length: usize,
    ) -> Result<(), ApiError> {
        let trimmed = value.trim();
        
        if trimmed.is_empty() || trimmed.len() < min_length {
            warn!(
                error_code = %ErrorCode::ValidationError,
                field = field,
                "Field length is less than minimum required length"
            );
            return Err(ApiError::new(
                ErrorCode::ValidationError,
                &format!("{} must be at least {} characters", field, min_length),
                ErrorContext::new().with_details(serde_json::json!({
                    "field": field,
                    "code": "MIN_LENGTH",
                    "min_length": min_length,
                    "actual_length": trimmed.len()
                }))
            ));
        }

        if trimmed.len() > max_length {
            warn!(
                error_code = %ErrorCode::ValidationError,
                field = field,
                "Field length exceeds maximum allowed length"
            );
            return Err(ApiError::new(
                ErrorCode::ValidationError,
                &format!("{} cannot be longer than {} characters", field, max_length),
                ErrorContext::new().with_details(serde_json::json!({
                    "field": field,
                    "code": "MAX_LENGTH",
                    "max_length": max_length,
                    "actual_length": trimmed.len()
                }))
            ));
        }

        Ok(())
    }
}