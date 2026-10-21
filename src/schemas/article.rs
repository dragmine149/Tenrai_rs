use crate::schemas::common::Images;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Article {
    /// Author profile URL
    pub author_url: String,
    /// Author MAL Username
    pub author_username: String,
    /// Comment Count
    pub comments: Option<u64>,
    /// Date posted (ISO8601 format)
    pub date: String,
    /// Excerpt (or some kind of preview)
    pub excerpt: String,
    /// Images on the entry.
    pub images: Images,
    /// The ID of the article
    pub mal_id: u64,
    /// The title of the article
    pub title: String,
    /// The URL to the article
    pub url: String,
    /// The URL to the forum post on the article
    pub forum_url: String,
}
