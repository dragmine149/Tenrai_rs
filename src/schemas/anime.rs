//! Structs related to anime endpoints.

use crate::{
    generic_type,
    schemas::common::{AgeRating, Date, Season},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, str};
use strum_macros::Display;

/// Information about the anime entry in question.
///
/// NOTE: Some fields are only available when doing the [TenraiAPI::anime_full_details] query instead, and will be marked as such.
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
    /// External links and resources related to this anime
    ///
    /// Requires `full` details
    pub external: Option<Vec<External>>,
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
    /// More information related to the anime outside of the background/synopsis
    ///
    /// Requires `full` details
    pub more_info: Option<String>,
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
    /// Relationships with other media in the same series
    ///
    /// Requires `full` details
    pub relations: Option<Vec<Relation>>,
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
    /// External sites where you can watch the anime
    ///
    /// Requires `full` details
    pub streaming: Option<Vec<External>>,
    /// Animation Studios
    pub studios: Vec<StudioType>,
    /// Main plot summary description
    pub synopsis: Option<String>,
    /// Theme songs used in the anime.
    ///
    /// Requires `full` details
    pub theme: Option<ThemeSongs>,
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
/// Information about the form and to dates
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
    pub webp: Option<ImageDetails>,
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
    #[serde(rename = "TV Special")]
    TvSpecial,
    Unknown,
}
#[derive(Debug, Clone, Display, Deserialize, Serialize, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeStatus {
    #[serde(rename = "Currently Airing")]
    Airing,
    #[serde(rename = "Finished Airing")]
    Complete,
    #[serde(rename = "Not yet aired")]
    Upcoming,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct External {
    /// The name of the link
    pub name: String,
    /// The destination of the link
    pub url: String,
}

/// Combines a relationship between this entry and another entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    /// The relation between the two.
    pub relation: String,
    /// The other entry to compare to.
    pub entry: RelationEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationEntry {
    /// The ID on MAL.
    pub mal_id: u64,
    /// The type of this entry.
    #[serde(rename = "type")]
    pub entry_type: String,
    pub name: String,
    pub url: String,
    /// The media type of this entry.
    ///
    /// NOTE: This is different that [RelationEntry::entry_type].
    /// For example, the [RelationEntry::entry_type] might be `manga` whilst this is `light_novel`. or `anime` and `TV` respctably.
    pub media_type: String,
    pub images: Images,
}

/// Theme songs found in the anime.
///
/// TODO: Implement custom de/serialize to parse the data so this is much more usable.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeSongs {
    pub openeings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeRecommendation {
    pub url: String,
    pub votes: u64,
    pub entry: AnimeRecommendationEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeRecommendationEntry {
    pub mal_id: u64,
    pub images: Images,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeCharacter {
    character: AnimeCharacterEntry,
    favorites: u64,
    role: AnimeCharacterRole,
    voice_actors: AnimeCharacterVA,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeCharacterEntry {
    pub mal_id: u64,
    pub images: Images,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnimeCharacterRole {
    Main,
    Supporting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeCharacterVA {
    pub language: String,
    pub person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub favorites: u64,
    pub images: Images,
    pub mal_id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeStaff {
    pub person: Person,
    pub position: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeEpisode {
    pub aired: Option<String>,
    pub duration: Option<u64>,
    pub filler: bool,
    pub forum_url: Option<String>,
    pub images: Images,
    pub mal_id: u64,
    pub recap: bool,
    /// Number of forum replies for this episode
    pub replies: Option<u64>,
    pub score: Option<f64>,
    pub synopsis: Option<String>,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeArticle {
    /// Author profile URL
    pub author_url: String,
    /// Author MAL Username
    pub author_username: String,
    /// Comment Count
    pub comments: u64,
    /// Date posted (ISO8601 format)
    pub date: String,
    /// Excerpt (or some kind of preview)
    pub except: String,
    pub images: Images,
    pub mal_id: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum ArticleOrdering {
    #[strum(to_string = "published_at")]
    PublishedDate,
    #[strum(to_string = "views")]
    ViewCount,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum ForumOrdering {
    EpisodeNumber,
    #[strum(to_string = "created_on")]
    CreationDate,
    CommentCount,
    LastPost,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeForum {
    pub author_url: Option<String>,
    pub author_username: Option<String>,
    pub comments: u64,
    pub last_comment: CommentInfo,
    pub mal_id: u64,
    /// Topic title. Can be empty.
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentInfo {
    pub author_url: Option<String>,
    pub author_username: Option<String>,
    pub date: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeVideo {
    pub episodes: AnimeVideoEpisode,
    pub music_videos: AnimeVideoMusic,
    pub promo: Vec<AnimeVideoPromo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeVideoEpisode {
    pub episode: String,
    pub images: Images,
    pub mal_id: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeVideoMusic {
    pub meta: AnimeVideoMusicMeta,
    pub title: String,
    pub video: Trailer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeVideoMusicMeta {
    pub author: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeVideoPromo {
    pub title: String,
    pub trailer: Trailer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeStatistics {
    pub completed: u64,
    pub dropped: u64,
    pub on_hold: u64,
    pub plan_to_watch: u64,
    pub scores: AnimeStatsScore,
    pub total: u64,
    pub watching: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeStatsScore {
    pub percentage: f64,
    pub votes: u64,
    pub score: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeMoreInfo {
    #[serde(rename = "moreinfo")]
    pub more_info: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeReviewSort {
    Newest,
    Oldest,
    MostHelpful,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Sentiment {
    Recommended,
    MixedFeelings,
    NotRecommended,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeReview {
    pub date: Option<String>,
    pub episodes_watched: Option<u64>,
    pub is_preliminary: bool,
    pub is_spoiler: bool,
    pub mal_id: u64,
    pub reactions: AnimeReviewReaction,
    pub review: String,
    pub score: u64,
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub review_type: String,
    pub url: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeReviewReaction {
    pub confusing: u64,
    pub creative: u64,
    pub funny: u64,
    pub informative: u64,
    pub love_it: u64,
    pub nice: u64,
    pub overall: u64,
    pub well_written: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub images: Images,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum StackOrder {
    CreatedAt,
    RestackCount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeStack {
    pub author_url: Option<String>,
    pub author_username: Option<String>,
    pub created_at: Option<String>,
    pub description: String,
    pub entry_count: u64,
    pub is_challenge: u64,
    pub is_official: u64,
    pub is_spoiler: u64,
    pub mal_id: u64,
    pub restack_count: u64,
    pub stack_type: AnimeStackType,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeStackType {
    Anime,
    Manga,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum BasicAnimeOrder {
    #[strum(to_string = "mal_id")]
    MalID,
    Title,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeIDQueryInclude {
    Title,
    #[strum(to_string = "type")]
    AnimeType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnimeIDList {
    IDs(Vec<u64>),
    Context(Vec<AnimeIDContext>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeIDContext {
    pub mal_id: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub id_type: String,
}
