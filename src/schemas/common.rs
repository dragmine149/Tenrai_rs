//! Struct related to common data.
//!
//! This data doesn't come under a specific category as so many different endpoints return it.
//! NOTE: This is different that the ones found in [tenrai::schemas](crate::schemas) as those are more return helpers whilst this is data.

use serde::{Deserialize, Serialize, de::Visitor};
use std::fmt::Display;
use strum_macros::Display;

use crate::{generic_type, schemas::anime::User};

/// Stores the date related to the entry.
#[derive(Debug, Serialize, PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct External {
    /// The name of the link
    pub name: String,
    /// The destination of the link
    pub url: String,
}

/// Information about the recommendation.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Recommendation {
    /// Link to the recommended media
    pub url: String,
    /// How many people voted on this recommendation
    pub votes: u64,
    /// More info about the media itself.
    pub entry: Entry,
}

/// Information about the top recommendations. This contains a lot more information that the [Recommendation] alternative.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct TopRecommendation {
    /// Text of the first recommendation of the pair.
    pub content: Option<String>,
    /// When the first recommendation was written.
    pub date: Option<String>,
    /// More info about the Recommendation itself.
    pub entry: Vec<Entry>,
    /// Two MAL IDs of the pair. Lowest first
    ///
    /// TODO: Custom deserializer / serializer
    pub mal_id: String,
    /// THe URL on MAL
    pub url: String,
    /// The user who wrote the first recommendation of the pair.
    pub user: User,
    /// Number of users who recommended this pair in either direction
    pub votes: u64,
}

/// Information about an individual score and how much it contributes to the overall score.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct StatsScore {
    /// Percent out of total scores
    pub percentage: f64,
    /// How many users voted for this
    pub votes: u64,
    /// The score index in question
    pub score: u64,
}

/// More info.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct MoreInfo {
    #[serde(rename = "moreinfo")]
    pub more_info: Option<String>,
}

/// Information about a forum post
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Forum {
    /// The URL of the author
    pub author_url: Option<String>,
    /// The username of the author
    pub author_username: Option<String>,
    /// How many comments are on this post
    pub comments: u64,
    /// Information about the last comment
    pub last_comment: CommentInfo,
    /// The ID of the forum thread.
    pub mal_id: u64,
    /// Topic title. Can be empty.
    pub title: String,
    /// The URL to the forum post.
    pub url: String,
    /// Date forum was created.
    ///
    /// TODO: Date object
    pub date: String,
}

/// Basic information about a comment.
///
/// Doesn't include comment information.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct CommentInfo {
    /// The url of the author who posted the comment
    pub author_url: Option<String>,
    /// The username of the author who posted the comment
    pub author_username: Option<String>,
    /// The date/time when the comment was posted.
    ///
    /// TODO: Convert into proper date/time object
    pub date: Option<String>,
    /// The url to go to the comment in question.
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
pub enum IDsQueryInclude {
    Title,
    Type,
}

/// All unique valid IDs stored in the database.
///
/// Enum response will depend on if [AnimeIdsQuery::include](crate::schemas::query::AnimeIdsQuery::include) is present or not.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum IDList {
    /// A raw list of ids
    IDs(Vec<u64>),
    /// A raw list of ids plus some extra information
    Context(Vec<IDContext>),
}

/// Extra information along with the id itself
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct IDContext {
    pub mal_id: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub id_type: String,
}

/// Statistics about an entry for a certain day.
///
/// If the [StatsHistory::date] doesn't have any information. None will be returned.
///
/// SEE: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/statistics/history> for more information.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct StatsHistory<T> {
    /// Date queried
    pub date: Date,
    pub favorites: Option<u64>,
    pub members: Option<u64>,
    pub popularity: Option<u64>,
    pub rank: Option<u64>,
    pub score: Option<f64>,
    pub scored_by: Option<u64>,
    #[serde(rename = "statistics")]
    pub stats: Option<T>,
}

/// Basic ordering between MAL and Title
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "snake_case")]
pub enum BasicOrder {
    #[strum(to_string = "mal_id")]
    MalID,
    Title,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AlternativeTitle {
    /// The alternative title
    pub title: String,
    /// The type or language classification of the title
    #[serde(rename = "type")]
    pub title_type: String,
}

/// Date information about this media
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Aired {
    /// When the media started
    ///
    /// Formatted as ISO-8601. A date known only to the year or month is given as year or year-month instead of a full date.
    pub from: Option<String>,
    /// Exact integer split of the airing dates.
    pub prop: AiredProp,
    /// When the media finished
    ///
    /// Formatted as ISO-8601. A date known only to the year or month is given as year or year-month instead of a full date.
    pub to: Option<String>,
    /// String format which MAL uses to display the dates.
    pub string: Option<String>,
}

/// Information about the form and to dates
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct AiredProp {
    /// When this first started
    pub from: Date,
    /// When this finished.
    pub to: Date,
}

generic_type!(Demographics);
generic_type!(Genre);
generic_type!(Theme);

/// Combines a relationship between this entry and another entry.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelationDetails {
    /// The relation between the two.
    pub relation: Relation,
    /// The other entry to compare to.
    pub entry: Vec<RelationEntry>,
}

/// The relation between two entries.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Relation {
    Adaptation,
    Sequel,
    Prequel,
    #[serde(rename = "Spin-Off")]
    SpinOff,
    #[serde(rename = "Side Story")]
    SideStory,
    #[serde(rename = "Alternative Version")]
    AlternativeVersion,
    #[serde(rename = "Alternative Setting")]
    AlternativeSetting,
    #[serde(rename = "Parent Story")]
    ParentStory,
    Other,
    Summary,
    Character,
    #[serde(rename = "Full Story")]
    FullStory,
}

/// Details about the related entry.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct RelationEntry {
    /// The ID on MAL.
    pub mal_id: u64,
    /// The type of this entry.
    #[serde(rename = "type")]
    pub entry_type: String,
    /// The name of the entry
    pub name: String,
    /// The URL to the entry
    pub url: String,
    /// The media type of this entry.
    ///
    /// NOTE: This is different that [RelationEntry::entry_type].
    /// For example, the [RelationEntry::entry_type] might be `manga` whilst this is `light_novel`. or `anime` and `TV` respctably.
    pub media_type: Option<String>,
    /// Images relating to the entry.
    pub images: Images,
}

/// More detailed information about a recommendation
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct FullRecommendationInfo {
    /// ID of the recommendation
    ///
    /// Format is provided as (id-id) where:
    /// - first id is "if you liked this"
    /// - second id is "i would recommend that"
    ///
    /// TODO: Custom deserializer / serializer
    pub mal_id: String,
    /// Each individual entry
    pub entry: Vec<Entry>,
    /// Content of the review
    pub content: Option<String>,
    /// Date the review was made
    ///
    /// TODO: Date Object
    pub date: String,
    /// Who made the review
    pub user: User,
}

/// Basic information about a tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    /// Name of this tag
    pub name: String,
    /// How many entries have this tag
    pub count: u64,
}
