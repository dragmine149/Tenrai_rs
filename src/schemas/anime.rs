use crate::{
    generic_type, query,
    schemas::{AgeRating, Date, Season, query_vec},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, str};
use strum_macros::Display;

/// Information about the anime entry in question.
#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    /// Airing dates of the anime.
    pub aired: Aired,
    /// Whether the anime is currently broadcasting live episodes
    pub airing: bool,
    /// Indicates whether the entry has been officially approved by MAL moderators
    pub approved: bool,
    /// Historical or context information about the production
    pub background: Option<String>,
    /// Weekly broadcast schedule day, time and timezone.
    pub broadcast: AnimeBroadcast,
    /// Demographic classifications (e.g. Shonen, Seinen).
    pub demographics: Vec<DemographicsType>,
    /// Length per episode or total run time of the anime
    pub duration: Option<String>,
    /// Total episodes broadcast.
    pub episodes: Option<u64>,
    /// Explicit genre classification (E.g. Hentai, Ecchi)
    pub explicit_genres: Vec<GenreType>,
    /// Total users who have favorited the anime.
    pub favorites: Option<u64>,
    /// Genre Classification
    pub genres: Vec<GenreType>,
    /// Images used on the website.
    pub images: Images,
    /// Licensing Agencies
    pub licensors: Vec<LicensorType>,
    /// The unique MAL ID of this entry.
    pub mal_id: u64,
    /// Total users who have added this to their lists.
    pub members: Option<u64>,
    /// Global popularity rank based on [Anime::members].
    pub popularity: Option<u64>,
    /// Production companies or publishers
    pub producers: Vec<ProducerType>,
    /// Global rank based on average score
    pub rank: Option<u64>,
    /// Age rating categorization
    pub rating: Option<AgeRating>,
    /// The average rating score out of 10.
    pub score: Option<f64>,
    /// How many users have scored this anime.
    pub scored_by: Option<u64>,
    /// Season of premier.
    pub season: Option<Season>,
    /// The source material type.
    pub source: String,
    /// Airing status of the anime.
    pub status: AnimeStatus,
    /// Animation Studios
    pub studios: Vec<StudioType>,
    /// Main plot summary description
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
    /// Youtube trailer information
    pub trailer: Trailer,
    /// The broadcast or media type
    #[serde(rename = "type")]
    pub anime_type: AnimeType,
    /// The direct MAL webpage URL of the anime
    pub url: String,
    /// Year or premier.
    pub year: Option<u64>,
}

/// Date when this anime aired.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Aired {
    /// When the anime started
    ///
    /// Formatted as ISO-8601. A date known only to the year or month is given as year or year-month instead of a full date.
    pub from: Option<String>,
    /// Exact integer split of the airing dates.
    pub prop: AiredProp,
    /// When the anime finished
    ///
    /// Formatted as ISO-8601. A date known only to the year or month is given as year or year-month instead of a full date.
    pub to: Option<String>,
    /// String format which MAL uses to display the dates.
    pub string: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiredProp {
    pub from: Date,
    pub to: Date,
}

/// Weekly broadcast schedule day, time and timezone.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnimeBroadcast {
    /// Day of week the broadcast happened
    pub day: Option<String>,
    /// String format displayed on the MAL website
    pub string: Option<String>,
    /// Time at which the broadcast happened
    pub time: Option<String>,
    /// Which timezone this information is related to.
    pub timezone: Option<String>,
}

generic_type!(Demographics);
generic_type!(Genre);
generic_type!(Licensor);
generic_type!(Producer);
generic_type!(Studio);
generic_type!(Theme);

/// Images used on the website.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Images {
    /// Jpg versions of the images
    pub jpg: ImageDetails,
    /// Webp versions of the images
    pub webp: ImageDetails,
}

/// Details about the sizes of the images used on the website.
///
/// NOTE: Depending on the endpoint, not all images url's will be provided.
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
    /// The type or language classification of the title
    #[serde(rename = "type")]
    pub title_type: String,
}

/// Information about the youtube video representing the trailer.
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

/// The format that this anime is in.
#[derive(Debug, Clone, Display, Serialize, Deserialize, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeType {
    #[serde(rename = "TV")]
    Tv,
    Movie,
    #[serde(rename = "OVA")]
    Ova,
    Special,
    #[serde(rename = "ONA")]
    Ona,
    Music,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "PV")]
    Pv,
    #[serde(rename = "TV_SPECIAL")]
    TvSpecial,
}
#[derive(Debug, Clone, Display, Deserialize, Serialize, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeStatus {
    Airing,
    #[serde(rename = "Finished Airing")]
    Complete,
    Upcoming,
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
    rating: Vec<AgeRating>:query_vec,
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
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
    /// Field to sort the results by.
    order_by: OrderBy,
    /// Sort direction. Only applies when [AnimeQuery::order_by] is used.
    sort: SortDirection,
    /// Only anime whose title starts with this letter. Cannot be combined with [AnimeQuery::query].
    letter: char,
    /// List of produces to filter by. Use the ID of the producer.
    producers: Vec<u64>:query_vec,
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
