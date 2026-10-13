use crate::{
    generic_type, query,
    schemas::{Date, query_vec},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, str};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub aired: Aired,
    pub airing: bool,
    pub approved: bool,
    pub background: Option<String>,
    pub broadcast: AnimeBroadcast,
    pub demographics: Vec<DemographicsType>,
    pub duration: Option<String>,
    pub episodes: Option<u64>,
    pub explicit_genres: Vec<GenreType>,
    pub favorites: Option<u64>,
    pub genres: Vec<GenreType>,
    pub images: Images,
    pub licensors: Vec<LicensorType>,
    pub mal_id: u64,
    pub members: Option<u64>,
    pub popularity: Option<u64>,
    pub producers: Vec<ProducerType>,
    pub rank: Option<u64>,
    pub rating: Option<String>,
    pub score: Option<f64>,
    pub scored_by: Option<u64>,
    pub season: Option<String>,
    pub source: String,
    pub status: String,
    pub studios: Vec<StudioType>,
    pub synopsis: Option<String>,
    pub themes: Vec<ThemeType>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    pub titles: Vec<AlternativeTitle>,
    pub trailer: Trailer,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub url: String,
    pub year: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Aired {
    pub from: Option<String>,
    pub prop: AiredProp,
    pub to: Option<String>,
    pub string: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiredProp {
    pub from: Date,
    pub to: Date,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnimeBroadcast {
    pub day: Option<String>,
    pub string: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
}

generic_type!(Demographics);
generic_type!(Genre);
generic_type!(Licensor);
generic_type!(Producer);
generic_type!(Studio);
generic_type!(Theme);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Images {
    pub jpg: ImageDetails,
    pub webp: ImageDetails,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageDetails {
    pub image_url: Option<String>,
    pub large_image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub maximum_image_url: Option<String>,
    pub medium_image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlternativeTitle {
    pub title: String,
    #[serde(rename = "type")]
    pub title_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trailer {
    pub comment_count: Option<u64>,
    pub dislikes: Option<u64>,
    pub duration: Option<String>,
    pub embed_url: Option<String>,
    pub embeddable: Option<bool>,
    pub images: ImageDetails,
    pub likes: Option<u64>,
    pub privacy_status: Option<String>,
    pub published_at: Option<String>,
    pub region_restriction: Option<HashMap<String, Value>>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub views: Option<u64>,
    pub youtube_id: Option<String>,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeType {
    Tv,
    Movie,
    Ova,
    Special,
    Ona,
    Music,
    Cm,
    Pv,
    TvSpecial,
}
#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeStatus {
    Airing,
    Complete,
    Upcoming,
}
#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
/// The age rating of the anime.
pub enum AnimeRating {
    /// All Ages
    G,
    /// Children
    Pg,
    /// Teens 13 or older
    Pg13,
    /// 17+ (violence & profanity)
    R17,
    /// Mild Nudity
    R,
    /// Hentai
    Rx,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Unapproved {
    False,
    True,
    Only,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OrderBy {
    MalId,
    Title,
    StartDate,
    EndDate,
    Episodes,
    Score,
    ScoredBy,
    Rank,
    Popularity,
    Members,
    Favorites,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SortDirection {
    #[strum(to_string = "asc")]
    Ascending,
    #[strum(to_string = "desc")]
    Descending,
}

query!(Anime {
    /// Title search query (fuzzy matched against English, Japanese and default titles). Cannot be combined with [AnimeQuery::letter]
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by media type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Only anime with this airing status
    status: AnimeStatus,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AnimeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Also include unapproved entries.
    unapproved: Unapproved,
    /// Only entries with this exact score. Use [AnimeQuery::min_score] and [AnimeQuery::max_score] for a range.
    score: f64,
    /// Only entries with this score or higher
    min_score: f64,
    /// Only entries with this score or lower
    max_score: f64,
    /// Comma Separated IDs
    genres: String,
    /// Comma Separated IDs
    genres_exclude: String,
    /// Field to sort the results by.
    order_by: OrderBy,
    /// Sort direction. Only applies when [AnimeQuery::order_by] is used.
    sort: SortDirection,
    /// Only anime whose title starts with this letter. Cannot be combined with [AnimeQuery::query].
    letter: char,
    /// Comma Separated IDs
    producers: String,
    /// Only anime that started airing on or after this date.
    start_date: Date,
    /// Only anime that finished airing on or before this date.
    end_date: Date,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p < 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    score: "Score must be between 1 and 10": (|l: &f64| *l >= 1.0 && *l < 10.0),
    min_score: "Score must be between 1 and 10": (|l: &f64| *l >= 1.0 && *l < 10.0),
    max_score: "Score must be between 1 and 10": (|l: &f64| *l >= 1.0 && *l < 10.0),
});
