use crate::{cache::CacheError, error};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

error!(Tenrai {
    /// An error returned by the API.
    Tenrai: TenraiAPIError,
    /// An error happened within the local caching system.
    Cache: std::io::Error,
    /// An error happened within the reqwest crate.
    Reqwest: reqwest::Error,
    /// An error happened within translating the data.
    Serde: serde_json::Error,
});

pub type Error = TenraiError;

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
