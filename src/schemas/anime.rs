use std::{collections::HashMap, fmt::Display, str};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    generic_type, query,
    schemas::{Date, query_vec},
};

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

#[derive(Debug, Clone)]
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
impl Display for AnimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimeType::Tv => write!(f, "tv"),
            AnimeType::Movie => write!(f, "movie"),
            AnimeType::Ova => write!(f, "ova"),
            AnimeType::Special => write!(f, "special"),
            AnimeType::Ona => write!(f, "ona"),
            AnimeType::Music => write!(f, "music"),
            AnimeType::Cm => write!(f, "cm"),
            AnimeType::Pv => write!(f, "pv"),
            AnimeType::TvSpecial => write!(f, "tv_special"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnimeStatus {
    Airing,
    Complete,
    Upcoming,
}
impl Display for AnimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimeStatus::Airing => write!(f, "airing"),
            AnimeStatus::Complete => write!(f, "complete"),
            AnimeStatus::Upcoming => write!(f, "upcoming"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnimeRating {
    G,
    Pg,
    Pg13,
    R17,
    R,
    Rx,
}
impl Display for AnimeRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimeRating::G => write!(f, "g"),
            AnimeRating::Pg => write!(f, "pg"),
            AnimeRating::Pg13 => write!(f, "pg13"),
            AnimeRating::R17 => write!(f, "r17"),
            AnimeRating::R => write!(f, "r"),
            AnimeRating::Rx => write!(f, "rx"),
        }
    }
}

query!(Anime {
    q: String,
    page: u64,
    limit: u64,
    anime_type: Vec<AnimeType>:query_vec,
    status: Vec<AnimeStatus>:query_vec,
    rating: Vec<AnimeRating>:query_vec,
    sfw: String,
    sfw_strict: String,
    unapproved: String,
    score: u64,
    min_score: u64,
    max_score: u64,
    genres: String,
    genres_exclude: String,
    order_by: String,
    sort: String,
    letter: String,
    producers: String,
    start_date: String,
    end_date: String
});
