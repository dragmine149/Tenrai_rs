use crate::{
    cache::{RequestCache, TryRequestCache},
    schemas::TenraiRaw,
};
use derive_builder::Builder;
use reqwest::{Client, header::HeaderMap};
#[cfg(feature = "rate")]
use std::cell::RefCell;
use std::{default, time::Duration};
#[cfg(feature = "rate")]
use tokio::time::Instant;

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
/// There is a local based rate limiter implement with the `rate` feature enabled.
/// ## `rate` feature
/// With this enabled, requests are slowed down depending on if you have a server-key or not.
/// - Without a server-key: One request every 500ms (2/s)
/// - With a server-key: One request every 200ms (5/s)
///
/// Wilst technically we can go up to 4/s w/o a server key. The limit is at 2/s to avoid hitting the 120/m limit.
/// Server keys don't have this restriction as they can do 300/min which is in-line with the 5/s limit.
///
/// NOTE: The rate feature does not guarantee that a limit will never be hit. Especially with multiple clients using the same IP/Server-key.
/// It is here to reduce the chance of this one client hitting the limit.
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
    #[builder(setter(skip), default = "TenraiAPI::default_client()")]
    client: Client,
    /// To use local caching as well.
    #[builder(setter, default = true)]
    pub cache: bool,
    #[cfg(feature = "rate")]
    last_message: RefCell<Instant>,
}

// This impl is more for the technically stuff related to the api.
impl TenraiAPI {
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
            #[cfg(feature = "rate")]
            {
                let last = Instant::now()
                    .duration_since(*self.last_message.borrow())
                    .saturating_sub(match self.server_key {
                        Some(_) => Duration::from_millis(200),
                        None => Duration::from_millis(500),
                    });
                tokio::time::sleep(last).await;
            }

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
            #[cfg(feature = "rate")]
            {
                let mut now = self.last_message.borrow_mut();
                *now = Instant::now();
            }

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

impl Default for TenraiAPI {
    fn default() -> Self {
        Self {
            server_key: None,
            client: Self::default_client(),
            cache: true,
            #[cfg(feature = "rate")]
            last_message: RefCell::new(Instant::now()),
        }
    }
}

// this impl is more for the user-facing stuff of the API.
impl TenraiAPI {
    /// Create a new instance of the API.
    ///
    /// The server key is used for higher rate limits if you have supported on patreon.
    /// See more <https://api.tenrai.org/documentation#description/server-keys>
    pub fn new(server_key: Option<String>) -> Self {
        Self {
            server_key,
            ..Default::default()
        }
    }

    fn default_client() -> Client {
        Client::builder()
            .user_agent(format!(
                "Tenrai Rust Client - {}",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .unwrap()
    }

    pub fn builder() -> TenraiAPIBuilder {
        TenraiAPIBuilder::default()
    }
}
