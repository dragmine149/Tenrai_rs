use std::{any::Any, collections::HashMap, str};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{generic_type, schemas::Date};

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
