//! A Wrapper for the [Tenrai API](https://api.tenrai.org) in rust.
//!
//! # Usage
//! ```
//! use tenrai::{TenraiAPI, schemas::query::SfwQuery};
//!
//! #[tokio::main]
//! async fn main() {
//!     // create a client
//!     let client = TenraiAPI::default();
//!     // send a request to fetch konosuba
//!     let result = client.anime_full_details(30831, SfwQuery::default()).await;
//!     // use said data
//!     if let Ok(data) = result {
//!         println!("{}", data.data.url); // https://myanimelist.net/anime/30831/Kono_Subarashii_Sekai_ni_Shukufuku_wo
//!     }
//! }
//! ```
//!
//! ## Queries
//! A lot of endpoints have a required query parameter. If you don't want to attack any query to the request, provide `T::default()`.
//!
//! There are also various different types of queries depending on the combination of potential options.
//!
//! # Endpoint Notes
//! Most endpoints function names will follow the official ones provide by tenrai. However, some functions have been renamed. A table has been provided below
//! |Original|Renamed To|Reason|
//! |--------|----------|------|
//! |Producer|Company   |Company encomposises the returned data better|
//!
//! ## Server Keys
//! Some endpoints require the use of a server key to access. We check to see if a server key has been provided before a request is even sent out. This cannot check the validicity of the server key though.
//!
//! # See
//! - The official api docs this is based on: <https://api.tenrai.org/documentation>
//! - The github repo for more information: <https://github.com/dragmine149/Tenrai_rs>

use crate::{
    cache::{RequestCache, TryRequestCache},
    schemas::TenraiRaw,
};
use derive_builder::Builder;
use reqwest::{Client, header::HeaderMap};
#[cfg(feature = "rate")]
use std::cell::RefCell;
use std::time::Duration;
#[cfg(feature = "rate")]
use tokio::time::Instant;

mod cache;
mod endpoints;
pub mod errors;
pub mod schemas;

#[macro_use]
mod macros;

/// Version of [Result<T, E>](std::result::Result) with our errors autofilled in.
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
/// As well as the API cache system (<https://api.tenrai.org/documentation#description/caching-data>), we also cache the results locally.
///
/// By default, local cache is valid for one day after the cache was written. Cache is written to the path provided by `dirs::cache_dir().join("tenrai")`.
///
/// See [TenraiAPI::cache] for more info on how to deal with the duration of valid cache.
///
#[derive(Builder)]
pub struct TenraiAPI {
    /// Server key used for higher rate limits. Received by becoming a patron member: <https://api.tenrai.org/documentation#description/server-keys>
    #[builder(setter(into, strip_option), default)]
    server_key: Option<String>,
    /// Internal client object
    #[builder(setter(skip), default = "TenraiAPI::default_client()")]
    client: Client,
    /// How long should the cache be valid for.
    ///
    /// If you don't want caching, then set the duration to 0. *([Duration::is_zero])* This will skip even writing to a file.
    #[builder(setter, default = Duration::from_hours(24))]
    pub cache: Duration,
    #[cfg(feature = "rate")]
    #[builder(setter(skip), default = "RefCell::new(Instant::now())")]
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

    fn check_server_key(&self) -> crate::Result<()> {
        self.server_key.as_ref().ok_or_else(|| {
            crate::errors::TenraiError::Custom("A server key is required for this endpoint")
        })?;
        Ok(())
    }

    /// Internal function for sending the request itself.
    async fn request(&self, endpoint: String) -> crate::Result<TenraiRaw> {
        let url = format!("https://api.tenrai.org/v1/{endpoint}");
        let request = self.client.get(url).headers(self.build_headers());
        #[cfg(feature = "rate")]
        let delay = match self.server_key {
            Some(_) => Duration::from_millis(200),
            None => Duration::from_millis(500),
        };

        loop {
            #[cfg(feature = "rate")]
            {
                let now = Instant::now();
                let last = delay.saturating_sub(now.duration_since(*self.last_message.borrow()));
                tokio::time::sleep(last).await;
            }

            let request = request
                .try_clone()
                .expect("We should not be sending a stream");
            let request = if !self.cache.is_zero() {
                match request.try_cache_json(self.cache) {
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

            break match self.cache.is_zero() {
                true => Ok(response.json().await?),
                false => Ok(response.cache_json(self.cache).await?),
            };
        }
    }

    /// Returns a boolean based on if a server key has been provided or not.
    pub fn has_server_key(&self) -> bool {
        self.server_key.is_some()
    }
}

impl Default for TenraiAPI {
    fn default() -> Self {
        Self {
            server_key: None,
            client: Self::default_client(),
            cache: Duration::from_hours(24),
            #[cfg(feature = "rate")]
            last_message: RefCell::new(Instant::now()),
        }
    }
}

/// Various methods to create new versions of the API.
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

    // this sis the new default client. Unlike the basic client, we use our own custom user agent.
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
