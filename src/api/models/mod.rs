//! Common types used across the API layer

pub mod organization;
pub mod pagination;
pub mod responses;
pub mod health;

// Re-export commonly used types
pub use organization::{CreateOrganizationInput, OrganizationDto, UpdateOrganizationInput};
pub use pagination::{PaginatedResponse, PaginationParams};
pub use responses::{ApiResponse, ApiResponseBuilder, ErrorResponse};
