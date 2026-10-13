use serde::{Deserialize, Serialize};
pub mod anime;
use std::fmt::Display;

use crate::errors::TenraiAPIError;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TenraiPaginatorResponse<T> {
    pub pagination: Paginator,
    pub data: Vec<T>,
}

pub(crate) fn query_vec<T: Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|s| format!("{s}"))
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TenraiRaw<T> {
    pub data: Option<T>,
    pub status: Option<u64>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub error: Option<String>,
    pub path: Option<String>,
}

impl<T> TenraiRaw<T> {
    fn build_error(self) -> TenraiAPIError {
        TenraiAPIError {
            status: self.status.unwrap(),
            error_type: self.error_type.unwrap(),
            message: self.message.unwrap(),
            error: self.error.unwrap(),
            path: self.path.unwrap(),
        }
    }

    pub(crate) fn result(self) -> Result<TenraiResponse<T>, TenraiAPIError> {
        if let Some(data) = self.data {
            Ok(TenraiResponse { data })
        } else {
            Err(self.build_error())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TenraiRawPaginator<T> {
    pub data: Option<Vec<T>>,
    pub pagination: Option<Paginator>,
    pub status: Option<u64>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub error: Option<String>,
    pub path: Option<String>,
}

impl<T> TenraiRawPaginator<T> {
    fn build_error(self) -> TenraiAPIError {
        TenraiAPIError {
            status: self.status.unwrap(),
            error_type: self.error_type.unwrap(),
            message: self.message.unwrap(),
            error: self.error.unwrap(),
            path: self.path.unwrap(),
        }
    }

    pub(crate) fn result(self) -> Result<TenraiPaginatorResponse<T>, TenraiAPIError> {
        if let Some(data) = self.data {
            Ok(TenraiPaginatorResponse {
                // pagination should exist here.
                pagination: self.pagination.unwrap(),
                data,
            })
        } else {
            Err(self.build_error())
        }
    }
}
