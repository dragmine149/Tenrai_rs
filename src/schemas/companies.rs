use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::schemas::common::{AlternativeTitle, External, Images};

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum ProducerOrder {
    MalId,
    Count,
    Favorites,
    Established,
}

/// Information about a producer of an anime
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    /// istorical and context background information
    pub about: Option<String>,
    /// Number of approved anime that credit this company as a `producer`,`studio`,`licensor`
    pub count: u64,
    /// When the company was established
    ///
    /// ISO 8601 format. A date MAL gives only as a month and year is shown on the 1st of that month. One given only as a year is null.
    pub establied: Option<String>,
    /// Exxternal websites and links for this commpany
    ///
    /// Requires full details
    pub external: Option<Vec<External>>,
    /// Total list favorited count on MAL
    pub favorites: u64,
    /// Images of the company logo
    pub images: Images,
    /// ID on MAL
    pub mal_id: u64,
    /// Translations or alternative titles for this company
    pub titles: Vec<AlternativeTitle>,
    /// Direct link to the MAL page
    pub url: String,
}
