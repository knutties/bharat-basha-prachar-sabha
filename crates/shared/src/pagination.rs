use serde::{Deserialize, Serialize};

/// Pagination request parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

/// Paginated response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata included in responses.
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

impl PaginationParams {
    /// Calculate the SQL OFFSET for this page.
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.page_size
    }

    /// Build pagination metadata from a total count.
    pub fn meta(&self, total_items: u64) -> PaginationMeta {
        PaginationMeta {
            page: self.page,
            page_size: self.page_size,
            total_items,
            total_pages: (total_items + self.page_size - 1) / self.page_size,
        }
    }
}
