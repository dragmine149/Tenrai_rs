use serde::{Deserialize, Serialize, de::DeserializeOwned};
use strum_macros::Display;
pub mod anime;
use crate::errors::{MachineErrorType, TenraiAPIError};
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

/// Standard pagination metadata showing page count, total records, and navigation state.
#[derive(Debug, Serialize, Deserialize)]
pub struct Paginator {
    /// The highest page number available for this query
    pub last_visible_page: u64,
    /// Whether there are more results on the next page.
    pub has_next_page: bool,
    /// The current page number requested.
    pub current_page: u64,
    /// Detailed item pagination counts
    pub items: PaginatorItems,
}

/// Detailed item pagination counts
#[derive(Debug, Serialize, Deserialize)]
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
pub struct TenraiResponse<T> {
    /// The data stored based on the query.
    pub data: T,
}

/// Special struct for when returning data from the API.
///
/// Data stored is based on the endpoint called. With additional paginator information stored.
///
/// Also see [TenraiResponse] for the non-paginator version.
#[derive(Debug, Serialize, Deserialize)]
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
pub(crate) struct TenraiRaw {
    pub data: Option<serde_json::Value>,
    pub pagination: Option<Paginator>,
    pub status: Option<u64>,
    #[serde(rename = "type")]
    pub error_type: Option<MachineErrorType>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Date {
    pub day: Option<u64>,
    pub month: Option<u64>,
    pub year: Option<u64>,
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(year) = self.year {
            write!(f, "{:0>4}", year)?;
            if let Some(month) = self.month {
                write!(f, "-{:0>2}", month)?;
                if let Some(day) = self.day {
                    write!(f, "-{:0>2}", day)?
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
/// The age rating of the anime.
///
/// # Display vs Serialization
/// When using `format!("{}", AgeRating::{whatever})`. A different result will appear as compared to serialization.
/// This is to allow both the query and the API Json response to use the same enum.
pub enum AgeRating {
    /// All Ages
    #[serde(rename = "G - All Ages")]
    G,
    /// Children
    #[serde(rename = "PG - Children")]
    Pg,
    /// Teens 13 or older
    #[serde(rename = "PG-13 - Teens 13 or older")]
    Pg13,
    /// 17+ (violence & profanity)
    #[serde(rename = "R - 17+ (violence & profanity)")]
    R17,
    /// Mild Nudity
    #[serde(rename = "R+ - Mild Nudity")]
    R,
    /// Hentai
    #[serde(rename = "Rx - Hentai")]
    Rx,
}

/// The season when the given entry started.
///
/// # real-life Comparison
/// This follows the real life comparison that a season is 3 months long. Where winter is January -> March, spring is April -> June, etc.
///
/// NOTE: In some cases, a season can be the next season even if it started before the current season ends.
///
/// *that being said, you've probably watched enough anime to gather all of this*
#[derive(Debug, Clone, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}
