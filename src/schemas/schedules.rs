use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Filter by day of the week
#[derive(Debug, Serialize, Deserialize, Display, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "lowercase")]
pub enum ScheduleDayFilter {
    Monday,
    Tuesday,
    Wednesday,
    Friday,
    Saturday,
    Sunday,
    Unknown,
    Other,
    /// Uses japan time (UTC + 8)
    Today,
    /// Uses japan time (UTC + 8)
    Tomorrow,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[strum(serialize_all = "lowercase")]
pub enum ScheduleOrder {
    /// MAL ID
    MalId,
    /// Title
    Title,
    /// English title
    TitleEnglish,
    /// Japanese title
    TitleJapanese,
    /// Start date
    StartDate,
    /// End date
    EndDate,
    /// Number of episodes
    Episodes,
    /// Score
    Score,
    /// Number of users who scored it
    ScoredBy,
    /// Rank
    Rank,
    /// Popularity rank
    Popularity,
    /// Members
    Members,
    /// Favorites
    Favorites,
    /// Date MAL added it, newest first with sort=desc; approved anime only, even with unapproved
    Added,
    /// Change in MAL score over 30 days
    ScoreChange30D,
    /// Standard deviation of user scores; higher means more disagreement
    ScoreSpread,
    /// Net change in members over 7 days
    MembersGained7D,
}
