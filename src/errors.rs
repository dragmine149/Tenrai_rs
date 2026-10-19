//! Custom errors that may occur during usage.

use crate::{cache::CacheError, error};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::Display;

error!(
/// Custom error combining all other possible errors.
Tenrai {
    /// An error returned by the API.
    Tenrai: TenraiAPIError,
    /// An error happened within the local caching system.
    Cache: std::io::Error,
    /// An error happened within the reqwest crate.
    Reqwest: reqwest::Error,
    /// An error happened within translating the data.
    Serde: serde_json::Error,
});

/// Public generic crate type alias.
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

/// Information about any error that happened within the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TenraiAPIError {
    /// The status code returned.
    pub status: u64,
    /// Machine readable error category
    #[serde(rename = "type")]
    pub error_type: MachineErrorType,
    /// Human readable explanation
    pub message: String,
    /// Additional context or recovery guidance
    pub error: String,
    /// Request path that produced the error.
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

/// Machine readable error category.
///
/// *Although this isn't fully documented, they said it was machine readable so...*
///
/// # Route Not Found
/// `RouteNotFound` is only available via the `custom` feature. This is because every endpoint should be found.
#[derive(Debug, Serialize, Deserialize, Display, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum MachineErrorType {
    BadRequestException,
    UnauthorizedException,
    ForbiddenException,
    NotFoundException,
    MethodNotAllowedException,
    PayloadTooLargeException,
    UriTooLongException,
    RateLimitException,
    RequestHeaderFieldsTooLargeException,
    InternalServerErrorException,
    BadGatewayException,
    ServiceUnavailableException,
    GatewayTimeoutException,
    ResourceNotFoundException,
    #[cfg(feature = "custom")]
    RouteNotFound,
}
