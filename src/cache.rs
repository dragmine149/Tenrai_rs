use bytes::Bytes;
use reqwest::{RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;
use std::{
    fs::{self, OpenOptions},
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
    path::PathBuf,
    time::Duration,
};

pub fn cache_dir() -> anyhow::Result<PathBuf> {
    Ok(dirs::cache_dir()
        .ok_or(std::io::Error::other("Failed to get cache directory"))?
        .join("AniTorrent"))
}

fn get_cache_file(url: String) -> anyhow::Result<PathBuf> {
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

pub(crate) fn clear_cache() -> anyhow::Result<()> {
    let folder = cache_dir()?;
    Ok(fs::remove_dir_all(folder)?)
}

pub(crate) fn cache_size() -> anyhow::Result<u64> {
    let folder = cache_dir()?;
    Ok(dir_size(folder)?)
}

/// Read the cache from disk. Returns an error if something failed with the read, and option depending on the contents.
pub(crate) fn read_cache(url: &Url) -> anyhow::Result<Option<Bytes>> {
    let file = get_cache_file(url.to_string())?;
    // println!("{:?}", file);
    if !file.exists() || file.metadata()?.modified()?.elapsed()? > Duration::from_hours(24) {
        return Ok(None);
    }

    Ok(Some(Bytes::copy_from_slice(&fs::read(file)?)))
    // Ok(Some(fs::read_to_string(file)?))
}

/// Write the response to a file in the [dirs::cache_dir()] returns the text result anyway for processing.
pub(crate) async fn cache_response(response: Response) -> anyhow::Result<Bytes> {
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
    fn json<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned;
}

impl Jsonify for String {
    fn json<T>(&self) -> Result<T, serde_json::Error>
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
    async fn cache(self) -> anyhow::Result<Bytes>;
    /// Does [RequestCache::cache] and then returns the result as a string. This is a more usable format for most than bytes.
    ///
    /// Additional errors include:
    /// - Not being able to convert to utf8.
    async fn cache_str(self) -> anyhow::Result<String>
    where
        Self: Sized,
    {
        Ok(String::from_utf8(self.cache().await?.to_vec())?)
    }
    /// Does [RequestCache::cache] and then returns the result as the requested object.
    ///
    /// Additional errors included:
    /// - Any error that can happen with `serde_json`
    async fn cache_json<T>(self) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        Self: Sized,
    {
        Ok(serde_json::from_slice(&self.cache().await?)?)
    }

    async fn cached(self) -> anyhow::Result<Cached>;
}

impl RequestCache for RequestBuilder {
    async fn cache(self) -> anyhow::Result<Bytes> {
        let (client, request) = self.build_split();
        let request = request?;
        let url = request.url();

        if let Some(cache) = read_cache(&url)? {
            return Ok(cache);
        }
        // println!("No cache, requesting");
        Ok(cache_response(client.execute(request).await?).await?)
    }
    async fn cached(self) -> anyhow::Result<Cached> {
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

    pub fn json<T>(self) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(serde_json::from_slice(&self.result)?)
    }
    pub fn str(self) -> anyhow::Result<String> {
        Ok(String::from_utf8(self.result.to_vec())?)
    }
    /// Return the bytes of the result.
    ///
    /// This NEVER FAILS. But is done with `Result` to allow for a quick swap between the other options.
    pub fn bytes(self) -> anyhow::Result<Bytes> {
        Ok(self.result)
    }
}
