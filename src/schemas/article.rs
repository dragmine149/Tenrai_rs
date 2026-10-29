use crate::schemas::common::Images;
use serde::{Deserialize, Serialize};

/// Information about an article
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct NewsArticle {
    /// Author profile URL
    pub author_url: Option<String>,
    /// Author MAL Username
    pub author_username: String,
    /// Comment Count
    pub comments: Option<u64>,
    /// Content in an article. Only available when requesting the article directly.
    pub content: Option<ArticleContent>,
    /// Date posted (ISO8601 format)
    pub date: String,
    /// Excerpt (or some kind of preview)
    pub excerpt: String,
    /// Images on the entry.
    pub images: Images,
    /// The ID of the article
    pub mal_id: u64,
    /// Article relation to anything. Only available when requesting the article directly.
    pub relations: Option<ArticleRelation>,
    /// The title of the article
    pub title: String,
    /// The URL to the article
    pub url: String,
    /// The URL to the forum post on the article. Not available when requesting the article directly.
    pub forum_url: Option<String>,
}

/// Information about an News Article
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Article {
    /// Author profile URL
    pub author_url: Option<String>,
    /// Author MAL Username
    pub author_username: String,
    /// Content in an article. Only available when requesting the article directly.
    pub content: Option<ArticleContent>,
    /// Date posted (ISO8601 format)
    pub published_at: String,
    /// Excerpt (or some kind of preview)
    pub excerpt: String,
    /// Images on the entry.
    pub images: Images,
    /// The ID of the article
    pub mal_id: u64,
    /// Article relation to anything. Only available when requesting the article directly.
    pub relations: Option<ArticleRelation>,
    /// Summary of the article. Normally a version of [Article::excerpt] expect it finishes the line.
    ///
    /// Only available when requesting the article directly.
    pub summary: Option<String>,
    /// Tags related to the article. Only available when requesting the article directly.
    pub tags: Option<Vec<String>>,
    /// The title of the article
    pub title: String,
    /// The URL to the article
    pub url: String,
    /// How many views this article has. Only available when requesting the article directly.
    pub views: Option<u64>,
}

/// Content in an article
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ArticleContent {
    /// Parsed HTML oof the article
    pub html: String,
    /// Markdown version of the article
    pub markdown: String,
}

/// Article relation to a piece of media
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ArticleRelation {
    pub anime: Vec<BasicRelationInfo>,
    pub manga: Vec<BasicRelationInfo>,
    /// Only available when requesting article details.
    pub characters: Option<Vec<BasicRelationInfo>>,
    /// Only available when requesting article details.
    pub people: Option<Vec<BasicRelationInfo>>,
}

/// Very basic information to link media together
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct BasicRelationInfo {
    pub mal_id: u64,
    pub name: String,
    pub url: String,
}
