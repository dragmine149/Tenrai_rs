use serde::{Deserialize, Serialize};
pub mod anime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Paginator {
    pub last_visible_page: u64,
    pub has_next_page: bool,
    pub current_page: u64,
    pub items: PaginatorItems,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatorItems {
    pub count: u64,
    pub total: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Date {
    pub day: Option<u64>,
    pub month: Option<u64>,
    pub year: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenraiResponse<T> {
    pub data: T,
}

pub struct TenraiPaginatorResponse<T> {
    pub pagination: Paginator,
    pub data: Vec<T>,
}
