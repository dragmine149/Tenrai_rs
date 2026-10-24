use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::schemas::{
    common::{
        Aired, AlternativeTitle, DemographicsType, External, GenreType, Images, RelationDetails,
        StatsScore, ThemeType,
    },
    people::{MangaAuthor, MangaSerialization},
};

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "lowercase")]
/// The type of the manga
pub enum MangaType {
    Manga,
    Novel,
    #[serde(rename = "Light Novel")]
    LightNovel,
    #[serde(rename = "One-shot")]
    Oneshot,
    #[serde(rename = "Doujinshi")]
    Doujin,
    Manhwa,
    Manhua,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "lowercase")]
/// What status is this manga in
pub enum MangaStatus {
    Publishing,
    #[serde(rename = "Finished")]
    Complete,
    Upcoming,
    Discontinued,
    #[serde(rename = "On Hiatus")]
    Hiatus,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
/// How to order the manga
pub enum MangaOrder {
    MalID,
    Title,
    StartDate,
    EndDate,
    Chapters,
    Volumes,
    Score,
    ScoredBy,
    Rank,
    Popularity,
    Members,
    Favorites,
}

/// Information about the manga entry in question.
///
/// NOTE: Some fields are only available when doing [TenraiAPI::manga_full_details](crate::TenraiAPI::manga_full_details) and will be marked as such.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Manga {
    /// Indicates whether the entry has been officially approved by MAL moderators
    pub approved: bool,
    /// Authors and artists credited for the manga.
    pub authors: Vec<MangaAuthor>,
    /// Historical or context information about the production
    pub background: Option<String>,
    /// Total chapters (null if ongoing)
    pub chapters: Option<u64>,
    /// Demographic classifications (e.g. Shonen, Seinen).
    pub demographics: Vec<DemographicsType>,
    /// Explicit genre classification (E.g. Hentai, Ecchi)
    pub explicit_genres: Vec<GenreType>,
    /// External links and resources related to this manga
    ///
    /// Requires `full` details
    pub external: Option<Vec<External>>,
    /// Total users who have favorited the anime.
    pub favorites: Option<u64>,
    /// Genre Classification
    pub genres: Vec<GenreType>,
    /// Images used on the website.
    pub images: Images,
    /// The unique MAL ID of this entry.
    pub mal_id: u64,
    /// More information related to the manga outside of the background/synopsis
    ///
    /// Requires `full` details
    #[serde(rename = "moreinfo")]
    pub more_info: Option<String>,
    /// Total users who have added this to their lists.
    pub members: Option<u64>,
    /// Global popularity rank based on [Manga::members].
    pub popularity: Option<u64>,
    /// Publication date details.
    pub published: Aired,
    /// Whether the manga is currently in active serializaation
    pub publishing: bool,
    /// Global rank based on average score
    pub rank: Option<u64>,
    /// Relationships with other media in the same series
    ///
    /// Requires `full` details
    pub relations: Option<Vec<RelationDetails>>,
    /// The average rating score out of 10.
    pub score: Option<f64>,
    /// How many users have scored this anime.
    pub scored_by: Option<u64>,
    /// Publications or magazines where the manga was serialized.
    pub serializations: Vec<MangaSerialization>,
    /// Status of the manga
    pub status: MangaStatus,
    /// Synopsis of the manga
    pub synopsis: Option<String>,
    /// Theme Classification.
    pub themes: Vec<ThemeType>,
    /// The primary default title of the anime.
    pub title: String,
    /// Official English title of the anime.
    pub title_english: Option<String>,
    /// Official Japanese title of the anime.
    pub title_japanese: Option<String>,
    /// Alternative names or synonyms of the title.
    pub title_synonyms: Vec<String>,
    /// List of alternative titles and translations of the anime.
    ///
    /// *Might contain duplicate entries of the titles in this struct.*
    pub titles: Vec<AlternativeTitle>,
    /// The publication format of the manga.
    #[serde(rename = "type")]
    pub manga_type: Option<MangaType>,
    /// The link to MAL of this manga
    pub url: String,
    /// Total volumes published (null if ongoing)
    pub volumes: Option<u64>,
}

/// Statistics about the specified anime
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct MangaStatistics {
    pub completed: u64,
    pub dropped: u64,
    pub on_hold: u64,
    pub plan_to_read: u64,
    /// Breakdown of each score and how much it contributes
    pub scores: Vec<StatsScore>,
    pub total: u64,
    pub reading: u64,
}

/// Which MyAnimeList list of adapted manga to return
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum MangaAdaptedFilter {
    All,
    Upcoming,
    Airing,
}
