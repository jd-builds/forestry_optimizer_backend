mod api_error;
mod error_code;
mod error_context;

pub use api_error::ApiError;
pub use error_code::ErrorCode;
pub use error_context::ErrorContext;

pub type Result<T> = std::result::Result<T, ApiError>;
