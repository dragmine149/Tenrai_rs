//! Schemas of stuff used for filters.
//!
//! Kinda don't deserve their own files due to how small they are.

use core::str;

use crate::schemas::common::Season;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum SeasonOrder {
    /// Score
    Score,
    /// Members,
    Members,
    /// Start date
    StartDate,
    /// MAL ID
    MalId,
    /// Number of episodes
    Episodes,
    /// Favorites
    Favorites,
    /// End date
    EndDate,
    /// Number of users who scored it
    ScoredBy,
    /// Title
    Title,
    /// English title
    TitleEnglish,
    /// Japanese title
    TitleJapanese,
    /// Rank
    Rank,
    /// Popularity rank
    Popularity,
    /// Date MAL added it, newest first with sort=desc; approved anime only, even with unapproved
    Added,
    /// Change in MAL score over 30 days
    #[strum(serialize = "score_change_30d")]
    ScoreChange30d,
    /// Standard deviation of user scores; higher means more disagreement
    ScoreSpread,
    /// Net change in members over 7 days
    #[strum(serialize = "members_gained_7d")]
    MembersGained7d,
}

/// Conjoint year and season.
#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonLookup {
    pub year: u32,
    pub seasons: Vec<Season>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum TopAnimeFilter {
    Airing,
    Upcoming,
    #[strum(to_string = "bypopularity")]
    Popularity,
    Favorite,
}
#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum TopMangaFilter {
    Publishing,
    Upcoming,
    #[strum(to_string = "bypopularity")]
    Popularity,
    Favorite,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
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
    pub mal_id: u32,
    /// Name of genre
    pub name: String,
    /// Direct URL to genre
    pub url: String,
    /// How many entries have this item
    pub count: u32,
}

/// Order magaines by this
#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum MagazineOrder {
    MalId,
    Name,
    Count,
}

/// How to order episodes returned by [TenraiAPI::top_episodes](crate::TenraiAPI::top_episodes)
#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum TopEpisodeOrder {
    /// Uses the existing weighted ranking. (Default)
    Rank,
    /// Highest average vote first, among episodes with at least 100 votes.
    Score,
    /// Most Votes first.
    Votes,
    /// Most forum replies first.
    Comments,
    Aired,
    /// Episode length in seconds
    Duation,
    Title,
    TitleJapanese,
    TitleEnglish,
}

/// Only entries consisting of either...
#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum EntryType {
    Anime,
    Manga,
}

/// Only entries consisting of either...
#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum TopVideoFilter {
    All,
    Airing,
    Upcoming,
    Finished,
}
