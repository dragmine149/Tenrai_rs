use derive_builder::Builder;
use reqwest::{Client, header::HeaderMap};
use serde::{Deserialize, de::DeserializeOwned};

use crate::cache::RequestCache;

mod cache;
pub mod endpoints;
mod macros;
pub mod schemas;

#[derive(Builder)]
pub struct TenariAPI {
    #[builder(setter(into, strip_option), default)]
    server_key: Option<String>,
    #[builder(setter(skip))]
    client: Client,
    cache: bool,
}

impl TenariAPI {
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

    pub fn builder() -> TenariAPIBuilder {
        TenariAPIBuilder::default()
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

    async fn request<T: DeserializeOwned>(&self, endpoint: String) -> anyhow::Result<T> {
        self.client
            .get(format!("https://api.tenrai.org/v1/{endpoint}"))
            .headers(self.build_headers())
            .cache_json()
            .await
    }
}
