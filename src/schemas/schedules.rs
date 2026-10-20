use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Filter by day of the week
#[derive(Debug, Serialize, Deserialize, Display, PartialEq, Eq, Clone)]
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
}
