use crate::{
    cache::{RequestCache, TryRequestCache},
    schemas::TenraiRaw,
};
use derive_builder::Builder;
use reqwest::{Client, header::HeaderMap};
use std::time::Duration;

mod cache;
pub mod endpoints;
pub mod errors;
pub mod schemas;

#[macro_use]
mod macros;

pub type Result<T> = std::result::Result<T, crate::errors::Error>;

/// Wrapper for the [TenraiAPI](https://tenrai.org).
///
/// # Rate-Limits
/// By default, this client provides no protection against being rate-limited. If you send too many requests, they
/// will just end up being retried until the server allows them through.
///
/// # Caching
/// Caching is done locally by default. Anything cached will stay valid for 1 day after the cache has been modified.
/// This can be bypassed by setting `cache` to false.
///
#[derive(Builder)]
pub struct TenraiAPI {
    /// Server key used for higher rate limits. Received by becoming a patron member: <https://api.tenrai.org/documentation#description/server-keys>
    #[builder(setter(into, strip_option), default)]
    server_key: Option<String>,
    /// Internal client object
    #[builder(setter(skip))]
    client: Client,
    /// To use local caching as well.
    #[builder(setter, default = true)]
    pub cache: bool,
}

impl TenraiAPI {
    /// Create a new instance of the API.
    ///
    /// The server key is used for higher rate limits if you have supported on patreon.
    /// See more <https://api.tenrai.org/documentation#description/server-keys>
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
    async fn request(&self, endpoint: String) -> crate::Result<TenraiRaw> {
        let url = format!("https://api.tenrai.org/v1/{endpoint}");
        let request = self.client.get(url).headers(self.build_headers());

        loop {
            let request = request
                .try_clone()
                .expect("We should not be sending a stream");
            let request = if self.cache {
                match request.try_cache_json() {
                    Ok(c) => return Ok(c),
                    Err(e) => e,
                }
            } else {
                request
            };

            let response = request.send().await?;

            let headers = response.headers();
            let status = response.status();
            #[cfg(feature = "auto_retry")]
            if status.as_u16() == 429 {
                // if this ain't provided i'm going to be worried.
                let retry = headers.get("Retry-After").unwrap();
                let timeout = retry.to_str().unwrap().parse().unwrap_or(1_u64);
                #[cfg(feature = "log")]
                log::info!("Waiting {} seconds to auto-retry.", timeout);
                tokio::time::sleep(Duration::from_secs(timeout)).await;
                continue;
            }

            break match self.cache {
                true => Ok(response.cache_json().await?),
                false => Ok(response.json().await?),
            };
        }
    }
}
