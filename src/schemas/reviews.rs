use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::schemas::{anime::User, common::Entry};

/// How to sort a review list
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
pub enum ReviewSort {
    Newest,
    Oldest,
    MostHelpful,
}

/// If the reviewer would recommend the media or not.
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
pub enum Sentiment {
    Recommended,
    MixedFeelings,
    NotRecommended,
}

/// Information about an review for an anime
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Review {
    /// Date it was posted
    ///
    /// TODO: Convert to date object
    pub date: Option<String>,
    /// Link to the MAL entry of the review source
    ///
    /// Only included if the review source did not come from anime/manga
    pub entry: Option<Entry>,
    /// How far through they are (only if [Review::is_preliminary] is true)
    #[serde(alias = "episodes_watched")]
    #[serde(alias = "chapters_read")]
    pub progress: Option<u32>,
    /// Has this been posted before they finished it
    pub is_preliminary: bool,
    /// Does this entry contain spoilers
    pub is_spoiler: bool,
    /// The MAL ID for the review
    pub mal_id: u32,
    /// Reactions to the review provided by the community.
    pub reactions: ReviewReaction,
    /// The review itself
    pub review: String,
    /// The score given by the reviewer
    pub score: u32,
    /// Any tags on the review
    pub tags: Vec<String>,
    /// What the review is on (anime/manga/light novel/etc)
    #[serde(rename = "type")]
    pub review_type: String,
    /// URL to the review
    pub url: String,
    /// Information about the user who posted the review
    pub user: User,
}

/// What reaction the community gave the review and how many are of that reaction.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ReviewReaction {
    pub confusing: u32,
    pub creative: u32,
    pub funny: u32,
    pub informative: u32,
    pub love_it: u32,
    pub nice: u32,
    pub overall: u32,
    pub well_written: u32,
}
