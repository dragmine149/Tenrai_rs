//! Every struct related to making queries
//!
//!

use crate::{
    query,
    schemas::{
        anime::{
            AnimeIDsQueryInclude, AnimeReviewSort, AnimeStatus, AnimeType, ArticleOrdering,
            BasicAnimeOrder, ForumOrdering, OrderBy, Sentiment, StackOrder,
        },
        common::{AgeRating, Date, OnlyBool, SortDirection},
        query_vec,
    },
};
use derive_builder::Builder;

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
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("?sfw-strict={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(Anime {
    /// Title search query (fuzzy matched against English, Japanese and default titles). Cannot be combined with [AnimeQuery::letter]
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by media type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Only anime with this airing status
    status: AnimeStatus,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Also include unapproved entries.
    unapproved: OnlyBool,
    /// Only entries with this exact score. Use [AnimeQuery::min_score] and [AnimeQuery::max_score] for a range.
    score: f64,
    /// Only entries with this score or higher
    min_score: f64,
    /// Only entries with this score or lower
    max_score: f64,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
    /// Field to sort the results by.
    order_by: OrderBy,
    /// Sort direction. Only applies when [AnimeQuery::order_by] is used.
    sort: SortDirection,
    /// Only anime whose title starts with this letter. Cannot be combined with [AnimeQuery::query].
    letter: char,
    /// List of produces to filter by. Use the ID of the producer.
    producers: Vec<u64>:query_vec,
    /// Only anime that started airing on or after this date.
    start_date: Date,
    /// Only anime that finished airing on or before this date.
    end_date: Date,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    score: "Score must be between 1 and 10": (|l: &f64| *l >= 1.0 && *l < 10.0),
    min_score: "Score must be between 1 and 10": (|l: &f64| *l >= 1.0 && *l < 10.0),
    max_score: "Score must be between 1 and 10": (|l: &f64| *l >= 1.0 && *l < 10.0),
    rating: "Rating can not be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
});

query!(AnimeReview {
    /// Page number
    page: u64,
    /// Sort direction.
    sort: AnimeReviewSort,
    /// Include reviews written before all episodes have been watched
    preliminary: OnlyBool,
    /// Include reviews which might contain spoilers
    spoilers: OnlyBool,
    /// Filter by what the review overall thinks.
    sentiment: Sentiment,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(Stack {
    /// The name of the stack to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page Number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// How to sort the stacks.
    stack_order: StackOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Include stacks that contain spoilers
    spoilers: OnlyBool,
    /// Include stacks that are challenges. NOTE: [StackQuery::official] also needs to not be false.
    challenges: OnlyBool,
    /// Includes stacks that are made by MAL instead of a user.
    official: OnlyBool,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("?sfw-strict={v}")),
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(AnimeIds {
	/// Filter by media type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Only anime with this airing status
    status: AnimeStatus,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Fields to include per entry. Allowed: Title, Type
    include: Vec<AnimeIDsQueryInclude>:query_vec,
    /// Filter entries updated on or after this ISO 8601 date-time or Unix timestamp. This is as updated on site, not API. May or may not be fully accurate.
    updated_at: String,
}, {});

query!(BasicAnime {
    /// The name of the anime to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// How to sort the anime
    order_by: BasicAnimeOrder,
    /// Order in which to sort.
    sort: SortDirection,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(
/// How to query the forum.   NOTE: This does not include the `filter` param from the official API as that does nothing rn.
Forum {
    page: u64,
    order_by: ForumOrdering,
    sort: SortDirection,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(Article {
    page: u64,
    order_by: ArticleOrdering,
    sort: SortDirection,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(
/// Query for getting historical stats.
StatsHistory {
    page: u64,
    limit: u64,
    sort: SortDirection,
    /// Only days **on** or **after** this date
    start_date: Date,
    /// Only days **on** or **before** this date
    end_date: Date,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});
