use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    pub page: i64,
    pub per_page: i64,
}

impl PaginationParams {
    #[allow(dead_code)]
    pub fn new(page: i64, per_page: i64) -> Self {
        Self { 
            page: page.max(1), 
            per_page: per_page.clamp(1, 100)  // Limit page size between 1 and 100
        }
    }

    #[allow(dead_code)]
    pub fn get_offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    #[allow(dead_code)]
    pub fn get_limit(&self) -> i64 {
        self.per_page
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total_items: i64,
    pub total_pages: i64,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl<T> PaginatedResponse<T> {
    #[allow(dead_code)]
    pub fn new(data: Vec<T>, total: i64, pagination: &PaginationParams) -> Self {
        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;
        Self {
            data,
            meta: PaginationMeta {
                current_page: pagination.page,
                per_page: pagination.per_page,
                total_items: total,
                total_pages,
                has_next_page: pagination.page < total_pages,
                has_previous_page: pagination.page > 1,
            },
        }
    }
} 