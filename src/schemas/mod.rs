//! Schemas used throughout the client.
//! Everything here is returned from the API in one way or another.
//!
//! Structs used in this module are global between all endpoints. See individual sub-modules for more related info.
use crate::errors::{MachineErrorType, TenraiAPIError};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Display;

pub mod anime;
pub mod article;
pub mod characters;
pub mod common;
pub mod manga;
pub mod people;
pub mod query;
pub mod reviews;
pub mod schedules;
pub mod stacks;

/// Display function helper for turning a vector into a CSV. Mainly used for query parameters.
pub(crate) fn query_vec<T: Display>(var_name: &str, vec: &[T]) -> String {
    format!(
        "{var_name}={}",
        vec.iter()
            .map(|s| format!("{s}"))
            .collect::<Vec<String>>()
            .join(",")
    )
}

/// Standard pagination metadata showing page count, total records, and navigation state.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Paginator {
    /// The highest page number available for this query
    pub last_visible_page: u64,
    /// Whether there are more results on the next page.
    pub has_next_page: bool,
    /// The current page number requested.
    pub current_page: Option<u64>,
    /// Detailed item pagination counts
    pub items: Option<PaginatorItems>,
}

/// Detailed item pagination counts
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaginatorItems {
    /// Number of records returned on the current page.
    pub count: u64,
    /// Total number of matching records in the database.
    pub total: u64,
    /// Maximum number of records per page.
    pub per_page: u64,
}

/// Special struct for when returning data from the API.
///
/// Data stored is based on the endpoint called. Also see [TenraiPaginatorResponse] for a pagination version.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TenraiResponse<T> {
    /// The data stored based on the query.
    pub data: T,
}

/// Special struct for when returning data from the API.
///
/// Data stored is based on the endpoint called. With additional total count of entries returned.
///
/// Also see [TenraiResponse] for the default version.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TenraiTotalResponse<T> {
    /// The data stored based on the query.
    pub data: T,
    /// The total count of entries.
    pub total: u64,
}

/// Special struct for when returning data from the API.
///
/// Data stored is based on the endpoint called. With additional paginator information stored.
///
/// Also see [TenraiResponse] for the non-paginator version.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TenraiPaginatorResponse<T> {
    /// Information about the current page.
    pub pagination: Paginator,
    /// The data stored based on the query.
    pub data: Vec<T>,
}

/// Internal struct to contain all the possible values that the API can return.
///
/// This is used before the data gets split up into `Result<Response, Error>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TenraiRaw {
    pub data: Option<serde_json::Value>,
    pub pagination: Option<Paginator>,
    pub total: Option<u64>,
    pub status: Option<u64>,
    #[serde(rename = "type")]
    pub error_type: Option<MachineErrorType>,
    pub message: Option<String>,
    pub error: Option<String>,
    pub path: Option<String>,
}

// Convert the raw data from the API into individual responses depending on what was requested.
impl TenraiRaw {
    fn build_error(self) -> TenraiAPIError {
        TenraiAPIError {
            status: self.status.unwrap(),
            error_type: self.error_type.unwrap(),
            message: self.message.unwrap(),
            error: self.error.unwrap(),
            path: self.path.unwrap(),
        }
    }

    pub(crate) fn result<T>(self) -> Result<TenraiResponse<T>, TenraiAPIError>
    where
        T: DeserializeOwned,
    {
        if let Some(data) = self.data {
            Ok(TenraiResponse {
                // shouldn't fail because it's part of the API.
                data: serde_json::from_value(data).unwrap(),
            })
        } else {
            Err(self.build_error())
        }
    }
    pub(crate) fn result_paginator<T>(self) -> Result<TenraiPaginatorResponse<T>, TenraiAPIError>
    where
        T: DeserializeOwned,
    {
        if let Some(data) = self.data {
            Ok(TenraiPaginatorResponse {
                pagination: self.pagination.unwrap(),
                // shouldn't fail because it's part of the API.
                data: serde_json::from_value::<Vec<T>>(data).unwrap(),
            })
        } else {
            Err(self.build_error())
        }
    }
    pub(crate) fn result_total<T>(self) -> Result<TenraiTotalResponse<T>, TenraiAPIError>
    where
        T: DeserializeOwned,
    {
        if let Some(data) = self.data {
            Ok(TenraiTotalResponse {
                data: serde_json::from_value(data).unwrap(),
                total: self.total.unwrap(),
            })
        } else {
            Err(self.build_error())
        }
    }
}
