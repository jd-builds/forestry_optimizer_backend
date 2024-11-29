use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common fields for all models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamps {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Base trait for all domain models
pub trait BaseModel {
    fn id(&self) -> Uuid;
    fn timestamps(&self) -> &Timestamps;
    fn timestamps_mut(&mut self) -> &mut Timestamps;
}

/// Helper methods for timestamps
impl Timestamps {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}

impl Default for Timestamps {
    fn default() -> Self {
        Self::new()
    }
} 