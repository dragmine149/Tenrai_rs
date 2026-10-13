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

#[derive(Debug)]
pub enum CacheError {
    Cache(std::io::Error),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}
impl std::error::Error for CacheError {}
impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                CacheError::Cache(e) => e.to_string(),
                CacheError::Reqwest(e) => e.to_string(),
                CacheError::Serde(e) => e.to_string(),
            }
        )
    }
}
impl From<std::io::Error> for CacheError {
    fn from(value: std::io::Error) -> Self {
        Self::Cache(value)
    }
}
impl From<reqwest::Error> for CacheError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
impl From<serde_json::Error> for CacheError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

pub fn cache_dir() -> std::io::Result<PathBuf> {
    Ok(dirs::cache_dir()
        .ok_or(std::io::Error::other("Failed to get cache directory"))?
        .join("AniTorrent"))
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
            .write(true)
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

pub(crate) fn clear_cache() -> std::io::Result<()> {
    let folder = cache_dir()?;
    Ok(fs::remove_dir_all(folder)?)
}

pub(crate) fn cache_size() -> std::io::Result<u64> {
    let folder = cache_dir()?;
    Ok(dir_size(folder)?)
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

pub trait RequestCache {
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

    async fn cached(self) -> Result<Cached, CacheError>;
}

impl RequestCache for RequestBuilder {
    async fn cache(self) -> Result<Bytes, CacheError> {
        let (client, request) = self.build_split();
        let request = request?;
        let url = request.url();

        if let Some(cache) = read_cache(&url)? {
            return Ok(cache);
        }
        // println!("No cache, requesting");
        Ok(cache_response(client.execute(request).await?).await?)
    }
    async fn cached(self) -> Result<Cached, CacheError> {
        let (client, request) = self.build_split();
        let request = request?;
        let url = request.url();

        if let Some(cache) = read_cache(url)? {
            return Ok(Cached {
                used_cache: true,
                result: cache,
            });
        }

        let cache = cache_response(client.execute(request).await?).await?;
        Ok(Cached {
            used_cache: false,
            result: cache,
        })
    }
}

pub struct Cached {
    used_cache: bool,
    result: Bytes,
}

impl Cached {
    pub fn used(&self) -> bool {
        self.used_cache
    }

    pub fn json<T>(self) -> serde_json::Result<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(&self.result)
    }
    pub fn str(self) -> String {
        String::from_utf8_lossy(self.result.chunk()).to_string()
    }
    /// Return the bytes of the result.
    pub fn bytes(self) -> Bytes {
        self.result
    }
}
