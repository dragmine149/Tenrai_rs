use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::cache::CacheError;

#[derive(Debug)]
pub enum Error {
    /// An error happened with the API
    Tenrai(TenraiAPIError),
    /// An error happened in local caching.
    Cache(std::io::Error),
    /// An error happened in local reqwest
    Reqwest(reqwest::Error),
    /// An error happened while deserializing data.
    Serde(serde_json::Error),
}
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Tenrai(e) => e.to_string(),
                Error::Cache(e) => e.to_string(),
                Error::Reqwest(e) => e.to_string(),
                Error::Serde(e) => e.to_string(),
            }
        )
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
impl From<CacheError> for Error {
    fn from(value: CacheError) -> Self {
        match value {
            CacheError::Cache(e) => Self::Cache(e),
            CacheError::Reqwest(e) => Self::Reqwest(e),
            CacheError::Serde(e) => Self::Serde(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenraiAPIError {
    pub status: u64,
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    pub error: String,
    pub path: String,
}

impl Display for TenraiAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) {} - {} ({})",
            self.status, self.error_type, self.error, self.path
        )
    }
}
