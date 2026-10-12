use derive_builder::Builder;
use reqwest::{Client, header::HeaderMap};
use serde::{Deserialize, de::DeserializeOwned};

use crate::cache::{Jsonify, RequestCache};

mod cache;
pub mod endpoints;
mod macros;
pub mod schemas;

/// Wrapper for the [TenraiAPI](https://tenrai.org).
///
/// # Rate-Limits
///
/// # Caching
/// Caching is done locally by default. Anything cached will stay valid for 1 day after the cache has been modified.
/// This can be bypassed by setting `cache` to false.
///
#[derive(Builder)]
pub struct TenraiAPI {
    /// Server key used for higher rate limits. Received by becoming a patron member: https://api.tenrai.org/documentation#description/server-keys
    #[builder(setter(into, strip_option), default)]
    server_key: Option<String>,
    /// Internal client object
    #[builder(setter(skip))]
    client: Client,
    /// To use local caching as well.
    pub cache: bool,
}

impl TenraiAPI {
    /// Create a new instance of the API.
    ///
    /// The server key is used for higher rate limits if you have supported on patreon.
    /// See more https://api.tenrai.org/documentation#description/server-keys
    pub fn new(server_key: Option<String>) -> Self {
        Self {
            server_key,
            client: Client::builder()
                .user_agent(format!(
                    "Tenrai Rust Client - {}",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .unwrap(),
            cache: true,
        }
    }

    pub fn builder() -> TenraiAPIBuilder {
        TenraiAPIBuilder::default()
    }

    /// build the required headers.
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/json".parse().unwrap());
        if let Some(key) = self.server_key.as_ref() {
            headers.insert("X-Server-Key", key.parse().unwrap());
        }
        headers
    }

    /// Internal function for sending the request itself.
    async fn request<T: DeserializeOwned>(&self, endpoint: String) -> anyhow::Result<T> {
        let builder = self
            .client
            .get(format!("https://api.tenrai.org/v1/{endpoint}"))
            .headers(self.build_headers());
        match self.cache {
            true => builder.cache_json().await,
            false => Ok(builder.send().await?.text().await?.json()?),
        }
    }
}
