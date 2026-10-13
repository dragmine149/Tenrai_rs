use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub mod anime;
use crate::errors::TenraiAPIError;
use std::fmt::Display;

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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TenraiRaw {
    pub data: Option<serde_json::Value>,
    pub pagination: Option<Paginator>,
    pub status: Option<u64>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub error: Option<String>,
    pub path: Option<String>,
}

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
}
