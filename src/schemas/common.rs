//! Struct related to common data.
//!
//! This data doesn't come under a specific category as so many different endpoints return it.
//! NOTE: This is different that the ones found in [tenrai::schemas] as those are more return helpers whilst this is data.

use crate::query;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::Display;

/// Stores the date related to the entry.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Date {
    pub day: Option<u64>,
    pub month: Option<u64>,
    pub year: Option<u64>,
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // we can assume that the next field won't exist without the previous.
        if let Some(year) = self.year {
            write!(f, "{:0>4}", year)?;
            if let Some(month) = self.month {
                write!(f, "-{:0>2}", month)?;
                if let Some(day) = self.day {
                    write!(f, "-{:0>2}", day)?
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
/// The age rating of the anime.
///
/// # Display vs Serialization
/// When using `format!("{}", AgeRating::{whatever})`. A different result will appear as compared to serialization.
/// This is to allow both the query and the API Json response to use the same enum.
pub enum AgeRating {
    /// This entry has no rating. DO not assume the rating is something like [G](AgeRating::G)
    ///
    /// See <https://myanimelist.net/anime/1863/Silk_Road_Shounen_Yuuto> for an example
    #[strum(to_string = "")]
    None,
    /// All Ages
    #[serde(rename = "G - All Ages")]
    G,
    /// Children
    #[serde(rename = "PG - Children")]
    Pg,
    /// Teens 13 or older
    #[serde(rename = "PG-13 - Teens 13 or older")]
    Pg13,
    /// 17+ (violence & profanity)
    #[serde(rename = "R - 17+ (violence & profanity)")]
    R17,
    /// Mild Nudity
    #[serde(rename = "R+ - Mild Nudity")]
    R,
    /// Hentai
    #[serde(rename = "Rx - Hentai")]
    Rx,
}

/// The season when the given entry started.
///
/// # real-life Comparison
/// This follows the real life comparison that a season is 3 months long. Where winter is January -> March, spring is April -> June, etc.
///
/// NOTE: In some cases, a season can be the next season even if it started before the current season ends.
///
/// *that being said, you've probably watched enough anime to gather all of this*
#[derive(Debug, Clone, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}

query!(
/// Filter out anime which are not SFW. NOTE: In some cases it might look like no filtering in done. In these cases, only external relations have been filtered.
SFW {
	/// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("?sfw-strict={v}")),
}, {});

query!(
/// Basic paginator query. Allows the selection of a different page.
BasicPage {
    page: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(
/// A combination of [SfwQuery] and [BasicPageQuery]
PageSFW {
    page: u64,
    sfw: bool,
    sfw_struct: bool:(|_, v| format!("?sfw-strict={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});
