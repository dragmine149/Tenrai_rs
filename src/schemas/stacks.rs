use serde::{Deserialize, Serialize};
use strum_macros::Display;

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
