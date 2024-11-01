use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<serde_json::Value>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T, meta: Option<serde_json::Value>, message: &str) -> Self {
        Self {
            data,
            meta,
            message: message.to_string(),
        }
    }
}
