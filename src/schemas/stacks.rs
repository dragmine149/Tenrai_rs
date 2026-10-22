use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::schemas::common::Images;

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
pub enum StackOrder {
    CreatedAt,
    RestackCount,
}

/// Information about an anime stack.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stack {
    /// URL of the author who made the stack
    pub author_url: Option<String>,
    /// Username of the author who made the stack
    pub author_username: Option<String>,
    /// When the stack was created
    ///
    /// TODO: Date object
    pub created_at: Option<String>,
    /// Description of the stack
    pub description: String,
    /// How many entries are in the stack (Max 50)
    pub entry_count: u64,
    /// Individual entries in a stack. Only available when requesting the stack
    pub entries: Option<Vec<StackEntry>>,
    /// Is the stack a challenge stack
    ///
    /// *complete 1 entry in some format by some date*
    pub is_challenge: bool,
    /// Is the stack an official stack made by MAL themselves
    pub is_official: bool,
    /// Does the stack potentially contain spoilers
    pub is_spoiler: bool,
    /// The ID of the stack on MAL
    pub mal_id: u64,
    /// How many users have restacked it
    pub restack_count: u64,
    /// What type is the stack
    pub stack_type: StackType,
    /// Title of the stack
    pub title: String,
    /// Direct URL to the stack
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum StackType {
    Anime,
    Manga,
}

/// Information about an entry on a stack
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StackEntry {
    /// Score given by the author
    pub author_score: Option<u64>,
    /// Images of the entry
    pub images: Images,
    /// ID of the entry on MAL
    pub mal_id: u64,
    /// Any notes left by the author
    pub note: Option<String>,
    /// The position in the stack
    pub position: u64,
    /// The title of the entry
    pub title: String,
    /// English version of the entry title
    pub title_english: Option<String>,
    /// Type of the entry
    ///
    /// NOTE: This can't be easily assigned to [AnimeType](crate::schemas::anime::AnimeType) or [MangaType](crate::schemas::manga::MangaType) as we don't have the context of the list.
    /// (and rust isn't going to be that easy)
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    /// URL of the entry
    pub url: String,
    /// Which year the anime first aired.
    pub aired_from_year: Option<u64>,
    /// How many episodes in the anime.
    pub episodes: Option<u64>,
    /// Which year the manga first published.
    pub published_from_year: Option<u64>,
    /// How many volumes in the manga.
    pub volumes: Option<u64>,
}
