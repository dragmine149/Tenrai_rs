use crate::error;
use bytes::{Buf, Bytes};
use reqwest::{RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;
use std::{
    fs::{self, OpenOptions},
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
    path::PathBuf,
    time::Duration,
};

error!(Cache {
    Cache: std::io::Error,
    Reqwest: reqwest::Error,
    Serde: serde_json::Error,
});

pub fn cache_dir() -> std::io::Result<PathBuf> {
    Ok(dirs::cache_dir()
        .ok_or(std::io::Error::other("Failed to get cache directory"))?
        .join(env!("CARGO_PKG_NAME")))
}

fn get_cache_file(url: String) -> std::io::Result<PathBuf> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let url_hash = hasher.finish().to_string();

    let folder = cache_dir()?;
    fs::create_dir_all(&folder)?;
    let save_path = folder.join(format!("{}.txt", url_hash));

    if !save_path.exists() {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(folder.join("map.csv"))?;
        writeln!(file, "{},{}", url_hash, url)?;
    }

    Ok(save_path)
}

/// Source: https://stackoverflow.com/a/60042340
fn dir_size(path: impl Into<PathBuf>) -> std::io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> std::io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    dir_size(fs::read_dir(path.into())?)
}

#[allow(dead_code)]
pub(crate) fn clear_cache() -> std::io::Result<()> {
    let folder = cache_dir()?;
    fs::remove_dir_all(folder)
}

#[allow(dead_code)]
pub(crate) fn cache_size() -> std::io::Result<u64> {
    let folder = cache_dir()?;
    dir_size(folder)
}

/// Read the cache from disk. Returns an error if something failed with the read, and option depending on the contents.
pub(crate) fn read_cache(url: &Url) -> std::io::Result<Option<Bytes>> {
    let file = get_cache_file(url.to_string())?;
    // println!("{:?}", file);
    if !file.exists()
        || file
            .metadata()?
            .modified()?
            .elapsed()
            .unwrap_or(Duration::MAX)
            > Duration::from_hours(24)
    {
        return Ok(None);
    }

    Ok(Some(Bytes::copy_from_slice(&fs::read(file)?)))
    // Ok(Some(fs::read_to_string(file)?))
}

/// Write the response to a file in the [dirs::cache_dir()] returns the text result anyway for processing.
pub(crate) async fn cache_response(response: Response) -> Result<Bytes, CacheError> {
    let success = response.status().is_success();
    let url = response.url().to_string();
    let text = response.bytes().await?;

    if success {
        // println!("{:?}", get_cache_file(url.clone()));
        fs::write(get_cache_file(url)?, &text)?;
    }

    Ok(text)
}

#[allow(dead_code)]
pub trait Jsonify {
    fn json<T>(&self) -> serde_json::Result<T>
    where
        T: DeserializeOwned;
}

impl Jsonify for String {
    fn json<T>(&self) -> serde_json::Result<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(self)
    }
}

/// An extension to anything to cache the data before returning it. Should also read from cache where possible.
#[allow(dead_code)]
pub(crate) trait RequestCache {
    /// Send the request and cache the result. Returns the raw bytes of the request for usage.
    ///
    /// If the URL is already cached, returns that instead of sending the response. Cache auto-expires after 24 hours.
    ///
    /// Errors can occur from:
    /// - invalid request object
    /// - error in network request itself
    /// - no ability to read/write to disk
    async fn cache(self) -> Result<Bytes, CacheError>;
    /// Does [RequestCache::cache] and then returns the result as a string. This is a more usable format for most than bytes.
    ///
    /// Additional errors include:
    /// - Not being able to convert to utf8.
    async fn cache_str(self) -> Result<String, CacheError>
    where
        Self: Sized,
    {
        Ok(String::from_utf8_lossy(self.cache().await?.chunk()).to_string())
    }
    /// Does [RequestCache::cache] and then returns the result as the requested object.
    ///
    /// Additional errors included:
    /// - Any error that can happen with `serde_json`
    async fn cache_json<T>(self) -> Result<T, CacheError>
    where
        T: DeserializeOwned,
        Self: Sized,
    {
        Ok(serde_json::from_slice(&self.cache().await?)?)
    }
}

/// An alternative version of RequestCache but returns the original if there is no cache
///
/// Note that the error is consumed and not returned.
#[allow(dead_code)]
pub(crate) trait TryRequestCache {
    fn try_cache(self) -> Result<Bytes, Self>
    where
        Self: Sized;
    fn try_cache_str(self) -> Result<String, Self>
    where
        Self: Sized,
    {
        let cache = self.try_cache()?;
        Ok(String::from_utf8_lossy(cache.chunk()).to_string())
    }
    fn try_cache_json<T>(self) -> Result<T, Self>
    where
        T: DeserializeOwned,
        Self: Sized,
    {
        let cache = self.try_cache()?;
        Ok(serde_json::from_slice(cache.chunk()).expect("This was malformed data"))
    }
}

impl RequestCache for RequestBuilder {
    async fn cache(self) -> Result<Bytes, CacheError> {
        let (client, request) = self.build_split();
        let request = request?;
        let url = request.url();

        if let Some(cache) = read_cache(url)? {
            return Ok(cache);
        }
        // println!("No cache, requesting");
        cache_response(client.execute(request).await?).await
    }
}
impl RequestCache for Response {
    async fn cache(self) -> Result<Bytes, CacheError> {
        cache_response(self).await
    }
}
impl TryRequestCache for RequestBuilder {
    fn try_cache(self) -> Result<Bytes, Self>
    where
        Self: Sized,
    {
        let (client, request) = self.build_split();
        let request = request.expect("I don't understand how this errored...");
        let url = request.url();

        if let Ok(cache) = read_cache(url)
            && let Some(cache) = cache
        {
            Ok(cache)
        } else {
            Err(RequestBuilder::from_parts(client, request))
        }
    }
}
