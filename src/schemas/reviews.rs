use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::schemas::anime::User;

/// How to sort a review list
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
pub enum ReviewSort {
    Newest,
    Oldest,
    MostHelpful,
}

/// If the reviewer would recommend the media or not.
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
pub enum Sentiment {
    Recommended,
    MixedFeelings,
    NotRecommended,
}

/// Information about an review for an anime
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    /// Date it was posted
    ///
    /// TODO: Convert to date object
    pub date: Option<String>,
    /// How many episodes watched (only if [Review::is_preliminary] is true)
    pub episodes_watched: Option<u64>,
    /// Has this been posted before they finished it
    pub is_preliminary: bool,
    /// Does this entry contain spoilers
    pub is_spoiler: bool,
    /// The MAL ID for the review
    pub mal_id: u64,
    /// Reactions to the review provided by the community.
    pub reactions: ReviewReaction,
    /// The review itself
    pub review: String,
    /// The score given by the reviewer
    pub score: u64,
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
#[serde(deny_unknown_fields)]
pub struct ReviewReaction {
    pub confusing: u64,
    pub creative: u64,
    pub funny: u64,
    pub informative: u64,
    pub love_it: u64,
    pub nice: u64,
    pub overall: u64,
    pub well_written: u64,
}
