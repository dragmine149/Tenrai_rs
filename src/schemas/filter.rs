//! Schemas of stuff used for filters.
//!
//! Kinda don't deserve their own files due to how small they are.

use core::str;

use crate::schemas::common::Season;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SeasonOrder {
    Score,
    Members,
    StartDate,
    MalId,
    Episodes,
    Favorites,
    EndDate,
    ScoredBy,
    Title,
}

/// Conjoint year and season.
#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonLookup {
    pub year: u64,
    pub seasons: Vec<Season>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum TopFilter {
    Airing,
    Upcoming,
    #[strum(to_string = "bypopularity")]
    Popularity,
    Favorite,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum GenreFilter {
    Genres,
    ExplicitGenres,
    Themes,
    Demographics,
}

/// Information about a specific item
#[derive(Debug, Serialize, Deserialize)]
pub struct BasicInfo {
    /// ID on MAL
    pub mal_id: u64,
    /// Name of genre
    pub name: String,
    /// Direct URL to genre
    pub url: String,
    /// How many entries have this item
    pub count: u64,
}

/// Order magaines by this
#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MagazineOrder {
    MalId,
    Name,
    Count,
}

/// How to order episodes returned by [TenraiAPI::top_episodes](crate::TenraiAPI::top_episodes)
#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum TopEpisodeOrder {
    /// Highest average vote first, among episodes with at least 100 votes.
    Score,
    /// Most Votes first.
    Votes,
    /// Most forum replies first.
    Comments,
}

/// Only entries consisting of either...
#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum EntryType {
    Anime,
    Manga,
}
