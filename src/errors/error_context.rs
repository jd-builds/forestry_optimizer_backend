use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct ErrorContext {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Implementation of error context building methods
#[allow(dead_code)]
impl ErrorContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add structured details to the error context
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Add key-value metadata to the error context
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Check if the error context contains any data
    pub fn is_empty(&self) -> bool {
        self.metadata.is_empty() && self.details.is_none()
    }
} 