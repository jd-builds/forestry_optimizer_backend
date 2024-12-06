use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    #[serde(flatten)]
    pub data: T,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

pub struct ApiResponseBuilder<T> {
    status: u16,
    message: String,
    data: Option<T>,
    metadata: Option<serde_json::Value>,
}

impl<T> ApiResponseBuilder<T> {
    pub fn success() -> Self {
        Self {
            status: 200,
            message: String::from("success"),
            data: None,
            metadata: None,
        }
    }

    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            data: self.data.expect("Data must be set before building response"),
            metadata: self.metadata,
            message: self.message,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str, details: Option<serde_json::Value>) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details,
        }
    }
}

// Implement Display for ErrorResponse
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error {}: {}",
            self.code,
            self.message
        )
    }
} 