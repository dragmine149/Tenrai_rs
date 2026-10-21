//! Schemas of stuff used for filters.
//!
//! Kinda don't deserve their own files due to how small they are.

use crate::schemas::common::Season;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SeasonOrder {
    Score,
    Members,
    StartDate,
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

/// Information about a specific genre when returned from `genre_`
#[derive(Debug, Serialize, Deserialize)]
pub struct GenreInfo {
    /// ID on MAL
    pub mal_id: u64,
    /// Name of genre
    pub name: String,
    /// Direct URL to genre
    pub url: String,
    /// How many entries have this genre
    pub count: u64,
}
