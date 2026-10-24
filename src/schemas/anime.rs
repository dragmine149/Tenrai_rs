//! Structs related to anime endpoints.

use crate::{
    generic_type,
    schemas::common::{
        AgeRating, Aired, AlternativeTitle, DemographicsType, External, GenreType, ImageDetails,
        Images, RelationDetails, Season, StatsScore, ThemeType,
    },
};
use regex::Regex;
use serde::{Deserialize, Serialize, de::Visitor};
use strum_macros::Display;

generic_type!(Licensor);
generic_type!(Producer);
generic_type!(Studio);

/// Information about the anime entry in question.
///
/// NOTE: Some fields are only available when doing [TenraiAPI::anime_full_details](crate::TenraiAPI::anime_full_details) query instead, and will be marked as such.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
    #[serde(rename = "moreinfo")]
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
    pub relations: Option<Vec<RelationDetails>>,
    /// The average rating score out of 10.
    pub score: Option<f64>,
    /// How many users have scored this anime.
    pub scored_by: Option<u64>,
    /// Season of premier.
    pub season: Option<Season>,
    /// The source material type.
    ///
    /// TODO: Enum-ify
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
    pub anime_type: Option<AnimeType>,
    /// The direct MAL webpage URL of the anime
    pub url: String,
    /// Year or premier.
    pub year: Option<u64>,
}

/// Weekly broadcast schedule day, time and timezone.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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

/// Information about the youtube video representing the trailer.
///
/// All fields are self-explanitor.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
    pub region_restriction: Option<RegionalRestrictions>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub views: Option<u64>,
    pub youtube_id: Option<String>,
}

/// Countries where YouTube allows or blocks the video, as two-letter country codes.
///
/// Only one of the two lists is present.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct RegionalRestrictions {
    pub allowed: Option<Vec<String>>,
    pub blocked: Option<Vec<String>>,
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

/// Status of the anime.
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

/// Which field to order the anime by.
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

/// Theme songs found in the anime.
///
/// TODO: Implement custom de/serialize to parse the data so this is much more usable.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ThemeSongs {
    pub openings: Vec<ThemeSong>,
    pub endings: Vec<ThemeSong>,
}

/// Information about the theme song.
///
/// NOTE: The data is parsed from a string using regex and hence can't easily be linked back to an entry on MAL.
///
/// NOTE: Index Data is lost on serializing.
#[derive(Debug, PartialEq, Eq)]
pub struct ThemeSong {
    /// The name of the song
    pub name: String,
    /// The artist(s) that sung it.
    pub artists: Vec<String>,
    /// The episodes that version was used in.
    ///
    /// When a song is only used in one episode, both values will be the same.
    ///
    /// NOTE: It is possible for no episodes to be listed. In which case an empty vec will be provided.
    pub episodes: Vec<(u64, u64)>,
}

/// Helper struct used for the custom deserializer for [[ThemeSong]]
///
/// The deserializer takes in the string and splits out it (via regex) to a more usable format.
struct ThemeSongVisitor;
impl<'de> Visitor<'de> for ThemeSongVisitor {
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "A theme song in the format of '{{name}} by {{composer}} ({{eps}})'"
        )
    }

    type Value = ThemeSong;
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // SEE: https://regex101.com/r/bkhn4r/2
        let regex =
            Regex::new(r#"(?mux) ^(?:\d+:\s+)? "(.+?)" \s+by\s+ (.+?)\s* (?:\(eps\s+(.+?)\))?$"#)
                .unwrap();
        let captures = regex
            .captures(v)
            .ok_or_else(|| serde::de::Error::custom("(themesong regex) Failed to parse regex"))?;

        let name = captures
            .get(1)
            .ok_or_else(|| serde::de::Error::missing_field("(themesong regex) Name"))?
            .as_str()
            .to_string();
        let artists = captures
            .get(2)
            .ok_or_else(|| serde::de::Error::missing_field("(themesong regex) Artists"))?
            .as_str()
            .split(",")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let episodes = captures
            .get(3)
            .map(|eps| {
                // .ok_or_else(|| serde::de::Error::missing_field("Episodes"))?
                eps.as_str()
                    .split(",")
                    .map(|ep| {
                        ep.split_once("-").map_or_else(
                            || {
                                let num = ep
                                    .trim()
                                    .parse::<u64>()
                                    .unwrap_or_else(|_| panic!("Expected digit found {}", ep));
                                (num, num)
                            },
                            |(a, b)| {
                                (
                                    a.trim()
                                        .parse()
                                        .unwrap_or_else(|_| panic!("Expected digit found {}", a)),
                                    b.trim()
                                        .parse()
                                        .unwrap_or_else(|_| panic!("Expected digit found {}", b)),
                                )
                            },
                        )
                    })
                    .collect::<Vec<(u64, u64)>>()
            })
            .unwrap_or_default();

        Ok(ThemeSong {
            name,
            artists,
            episodes,
        })
    }
}
impl<'de> Deserialize<'de> for ThemeSong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ThemeSongVisitor)
    }
}
impl Serialize for ThemeSong {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{:?} by {} (eps {})",
            self.name,
            self.artists.join(","),
            self.episodes
                .iter()
                .map(|ep| {
                    if ep.0 == ep.1 {
                        ep.0.to_string()
                    } else {
                        format!("{}-{}", ep.0, ep.1)
                    }
                })
                .fold(String::new(), |acc, x| acc + &x + ",")
        ))
    }
}

/// Information about an individual episode in the anime.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeEpisode {
    /// WHen the anime aired. Forrmat of YYYY-MM-DDTHH:mm:SS+TZ:tz
    ///
    /// TODO: Use a parser to make this into a more usable format.
    pub aired: Option<String>,
    /// The length of the episode in seconds.
    pub duration: Option<u64>,
    /// Is this episode a filler episode?
    pub filler: bool,
    /// The link to the forum URL for discussing about this episode.
    pub forum_url: Option<String>,
    /// Images related to the episode
    pub images: Images,
    /// The ID of this entry on MAL
    pub mal_id: u64,
    /// Is this a recap episode?
    pub recap: bool,
    /// Number of forum replies for this episode
    pub replies: Option<u64>,
    /// Average score of this episode.
    pub score: Option<f64>,
    /// Synopsis of the episode itself
    pub synopsis: Option<String>,
    /// The standard title of the episode
    pub title: String,
    /// The title in the japanese language
    pub title_japanese: Option<String>,
    /// The title romanjised
    pub title_romanji: Option<String>,
    /// The link to the MAL entry on the episode.
    pub url: String,
}

/// How to order the listing of articles
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
pub enum ArticleOrdering {
    #[strum(to_string = "published_at")]
    PublishedDate,
    #[strum(to_string = "views")]
    ViewCount,
}

/// How to order the listing of forums
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
pub enum ForumOrdering {
    EpisodeNumber,
    #[strum(to_string = "created_on")]
    CreationDate,
    CommentCount,
    LastPost,
}

/// Information about every video related to the anime
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeVideo {
    /// Episodes videos
    pub episodes: Vec<AnimeVideoEpisode>,
    /// Any music videos.
    pub music_videos: Vec<AnimeVideoMusic>,
    /// Any promotion videos.
    pub promo: Vec<AnimeVideoPromo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeVideoEpisode {
    /// The episode index?
    ///
    /// NOTE: Not the same as [[AnimeVideoEpisode::title]]
    pub episode: String,
    /// Images related to the episode
    pub images: Images,
    /// The entry of the episode on MAL
    pub mal_id: u64,
    /// The title of the episode
    pub title: String,
    /// The URL to the mal information of the episode.
    pub url: String,
}

/// Information about the music video.
///
/// This can be trusted, but many anime entries are missing videos so use something like <https://animethemes.moe> instead
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeVideoMusic {
    /// Information about the music video
    pub meta: AnimeVideoMusicMeta,
    /// Title of the video as visible in MAL.
    ///
    /// Contains:
    /// - op/ed/in num
    /// - title
    /// - authors
    ///
    /// See [AnimeVideoMusic::meta] for a split version
    pub title: String,
    /// Youtube entry of the music video
    pub video: Trailer,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeVideoMusicMeta {
    /// Author of the music video
    pub author: String,
    /// Title of the music video
    pub title: String,
}

/// Information about a promotion video for the anime
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeVideoPromo {
    /// Title of the video as shown on MAL.
    ///
    /// Is not the same as [Trailer::title]
    pub title: String,
    /// The details about the video.
    pub trailer: Trailer,
}

/// Statistics about the specified anime
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AnimeStatistics {
    pub completed: u64,
    pub dropped: u64,
    pub on_hold: u64,
    pub plan_to_watch: u64,
    /// Breakdown of each score and how much it contributes
    pub scores: Vec<StatsScore>,
    pub total: u64,
    pub watching: u64,
}

/// Information about a user entry
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct User {
    /// Their profile pictures
    pub images: Option<Images>,
    /// A direct link to their page on MAL
    pub url: String,
    /// Their username
    pub username: String,
}
