//! Struct related to common data.
//!
//! This data doesn't come under a specific category as so many different endpoints return it.
//! NOTE: This is different that the ones found in [tenrai::schemas](crate::schemas) as those are more return helpers whilst this is data.

use serde::{Deserialize, Serialize, de::Visitor};
use std::fmt::Display;
use strum_macros::Display;

/// Stores the date related to the entry.
#[derive(Debug, Serialize, PartialEq, Eq, Clone, Default)]
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
struct DateVisitor;
impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(DateVisitor)
    }
}
impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Date in the format of YYYY-MM-DD")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut split = v.splitn(3, "-").map(|s| s.parse::<u64>().ok());
        let year = split.next().flatten();
        let month = split.next().flatten();
        let day = split.next().flatten();
        Ok(Date { day, month, year })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut date = Date::default();
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "day" => {
                    date.day = map.next_value().ok();
                }
                "month" => {
                    date.month = map.next_value().ok();
                }
                "year" => {
                    date.year = map.next_value().ok();
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        Ok(date)
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

/// Extension of [bool] but with `Only`
#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OnlyBool {
    /// Exclude
    ///
    /// Will **NEVER** show these items
    False,
    /// Include
    ///
    /// Will Show these items
    True,
    /// Exclusive
    ///
    /// Will **ONLY** Show these items
    Only,
}

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
/// NOTE: Some endpoints won't return certain images. This is more generic of all images.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageDetails {
    pub image_url: Option<String>,
    pub large_image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub maximum_image_url: Option<String>,
    pub medium_image_url: Option<String>,
}

/// Direction to sort the results in.
#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SortDirection {
    #[strum(to_string = "asc")]
    Ascending,
    #[strum(to_string = "desc")]
    Descending,
}

/// Entries are short structs containing very basic "connection" information in a sense.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    /// Identifier of this entry on MAL
    pub mal_id: u64,
    /// Images related to this entry.
    pub images: Images,
    /// Title of this entry
    ///
    /// Characters use `name` instead.
    #[serde(alias = "name")]
    pub title: String,
    /// URL of this entry
    pub url: String,
}

/// Basic information containing a link to an external website.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct External {
    /// The name of the link
    pub name: String,
    /// The destination of the link
    pub url: String,
}
