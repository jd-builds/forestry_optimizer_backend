use crate::{
    application::api::types::organization::{CreateOrganizationInput, UpdateOrganizationInput},
    common::error::{ApiError, ErrorCode, ErrorContext},
};
use tracing::warn;
use crate::application::api::validation::Validate;

pub fn validate_organization_name(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        warn!(
            error_code = %ErrorCode::ValidationError,
            field = "name",
            "Organization name cannot be empty"
        );
        return Err(ApiError::new(
            ErrorCode::ValidationError,
            "Organization name cannot be empty",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "name",
                "code": "REQUIRED",
                "value": name
            }))
        ));
    }
    if name.len() > 255 {
        warn!(
            error_code = %ErrorCode::ValidationError,
            field = "name",
            "Organization name cannot be longer than 255 characters"
        );
        return Err(ApiError::new(
            ErrorCode::ValidationError,
            "Organization name cannot be longer than 255 characters",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "name",
                "code": "MAX_LENGTH",
                "max_length": 255,
                "actual_length": name.len()
            }))
        ));
    }
    Ok(())
}

impl Validate for CreateOrganizationInput {
    fn validate(&self) -> Result<(), ApiError> {
        validate_organization_name(&self.name)
    }
}

impl Validate for UpdateOrganizationInput {
    fn validate(&self) -> Result<(), ApiError> {
        validate_organization_name(&self.name)
    }
} 