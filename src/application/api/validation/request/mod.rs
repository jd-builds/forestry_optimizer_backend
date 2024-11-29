use crate::common::{
    error::{ApiError, ErrorCode, ErrorContext},
    pagination::PaginationParams,
};

pub fn validate_pagination(params: &PaginationParams) -> Result<(), ApiError> {
    if params.limit <= 0 {
        return Err(ApiError::new(
            ErrorCode::ValidationError,
            "Limit must be greater than 0",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "limit",
                "code": "MIN_VALUE",
                "min_value": 1,
                "actual_value": params.limit
            }))
        ));
    }

    if params.offset < 0 {
        return Err(ApiError::new(
            ErrorCode::ValidationError,
            "Offset must be greater than or equal to 0",
            ErrorContext::new().with_details(serde_json::json!({
                "field": "offset",
                "code": "MIN_VALUE",
                "min_value": 0,
                "actual_value": params.offset
            }))
        ));
    }

    Ok(())
}

impl super::Validate for PaginationParams {
    fn validate(&self) -> Result<(), ApiError> {
        validate_pagination(self)
    }
} 