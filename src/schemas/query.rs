//! Every struct related to making queries
//!
//!

use crate::{
    query,
    schemas::{
        anime::{AnimeSource, AnimeStatus, AnimeType, ArticleOrdering, ForumOrdering, OrderBy},
        common::{AgeRating, BasicOrder, Date, IDsQueryInclude, OnlyBool, Season, SortDirection},
        companies::ProducerOrder,
        filter::{EntryType, GenreFilter, MagazineOrder, SeasonOrder, TopEpisodeOrder, TopFilter},
        manga::{MangaAdaptedFilter, MangaOrder, MangaStatus, MangaType},
        people::PeopleOrder,
        query_vec,
        reviews::{ReviewSort, Sentiment},
        schedules::ScheduleDayFilter,
        stacks::StackOrder,
    },
};
use derive_builder::Builder;

query!(
/// Filter out anime which are not SFW. NOTE: In some cases it might look like no filtering in done. In these cases, only external relations have been filtered.
SFW {
	/// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
}, {});

query!(
/// Basic paginator query. Allows the selection of a different page.
BasicPage {
    /// Page Number
    page: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(
/// Full paginator query. Allows the selection of a different page.
Page {
    /// Page Number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(
/// A combination of [SfwQuery] and [BasicPageQuery]
PageSFW {
    page: u64,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(
/// A combination of [SfwQuery] and [PageQuery]
FullPageSFW {
    page: u64,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Results per page (Default: 25)
    limit: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
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
    /// Filter by source material. If multiple provided, will match with any of them.
    source: Vec<AnimeSource>:query_vec,
    /// Filter by a main trailer ID or URL. Playback availability is not checked.
    has_trailed: bool,
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
    /// Only entries that have been scored by this many users (inclusive) (`>=`)
    min_scored_by: u64,
    /// Only entries with this member count or higher
    min_members: u64,
    /// Only entries with this member count or lower
    max_members: u64,
    /// Only entries with this favourite count or higher
    min_favorites: u64,
    /// Only entries with this favourite count or lower
    max_favorites: u64,
    /// Only entries with this episode count or higher
    min_episodes: u64,
    /// Only entries with this episode count or lower
    max_episodes: u64,
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
    /// Filter by MAL Debut-season classification
    season: Season,
    /// Calendar year of the aired date. Cannot be combined with [AnimeQuery::min_year] or [AnimeQuery::max_year]
    year: u64,
    /// Anime that aired this *provided* year or later
    min_year: u64,
    /// Anime that aired this *provided* year or earlier
    max_year: u64,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    source: "Can only have 25 types of sources": (|s: &[AnimeSource]| s.len() > 25),
    score: "Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    max_score: "Max Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    min_episodes: "Must be at least 1 episode": (|l: &u64| *l == 0),
    max_episodes: "Must be at least 1 episode": (|l: &u64| *l == 0),
    rating: "Rating can not be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
}, {
    year:min_year:max_year,
    query:letter,
});

query!(Review {
    /// Page number
    page: u64,
    /// Sort direction.
    sort: ReviewSort,
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
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
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
    include: Vec<IDsQueryInclude>:query_vec,
    /// Filter entries updated on or after this ISO 8601 date-time or Unix timestamp. This is as updated on site, not API. May or may not be fully accurate.
    updated_at: String,
}, {});

query!(Basic {
    /// The name of the anime to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// How to sort the anime
    order_by: BasicOrder,
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
    /// Page number
    page: u64,
    order_by: ForumOrdering,
    sort: SortDirection,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(Article {
    /// Page number
    page: u64,
    /// Field to sort the results by.
    order_by: ArticleOrdering,
    /// Direction to sort the results
    sort: SortDirection,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
});

query!(
/// Query for getting historical stats.
StatsHistory {
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Direction to sort the results
    sort: SortDirection,
    /// Only days **on** or **after** this date
    start_date: Date,
    /// Only days **on** or **before** this date
    end_date: Date,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(Schedule {
    /// Filter by day of week
    filter: ScheduleDayFilter,
    /// Filter out kids entries
    kids: bool,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Include unapproved entries
    unapproved: OnlyBool,
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by anime type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    rating: "Rating can not be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
});

query!(Manga {
    /// The name of the anime to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by manga type, multiple can be selected
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Only manga with this publishing status
    status: MangaStatus,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only entries with this exact score
    score: f64,
    /// Only entries with this score or higher
    min_score: f64,
    /// Only entries with this score or lower
    max_score: f64,
    /// Also include unapproved entries
    unapproved: bool,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
    /// Sort the results by...
    order_by: MangaOrder,
    /// Sort direction. Only applies when [MangaQuery::order_by] is used.
    sort: SortDirection,
    /// Only manga whose title starts with this letter. Cannot be combined with [MangaQuery::query].
    letter: char,
    /// Magazines this manga has been published in. Use the ID of the magazine.
    magazines: Vec<u64>:query_vec,
    /// Only manga that started publishing on or after this date.
    start_date: Date,
    /// Only manga that finished publishing on or before this date. Manga with no end date, *such as one-shots*, are left out.
    end_date: Date,
    /// Only entries that have been scored by this many users (inclusive) (`>=`)
    min_scored_by: u64,
    /// Only entries with this member count or higher
    min_members: u64,
    /// Only entries with this member count or lower
    max_members: u64,
    /// Only entries with this favourite count or higher
    min_favorites: u64,
    /// Only entries with this favourite count or lower
    max_favorites: u64,
    /// Only entries with this volume count or higher
    min_volumes: u64,
    /// Only entries with this volume count or lower
    max_volumes: u64,
    /// Only entries with this chapter count or higher
    min_chapters: u64,
    /// Only entries with this chapter count or lower
    max_chapters: u64,
    /// Calendar year of the published start date. Cannot be combined with [MangaQuery::min_year] or [MangaQuery::max_year]
    year: u64,
    /// Manga that started publishing this *provided* year or later
    min_year: u64,
    /// Manga that started publishing this *provided* year or earlier
    max_year: u64,
    /// Filter by credited author or artist. If multiple are provided will match any of them. Requires MAL person IDs
    authors: Vec<String>: query_vec,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    score: "Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    max_score: "Max Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    min_chapters: "Must be at least 1 chapter": (|l: &u64| *l == 0),
    max_chapters: "Must be at least 1 chapter": (|l: &u64| *l == 0),
    min_volumes: "Must be at least 1 volume": (|l: &u64| *l == 0),
    max_volumes: "Must be at least 1 volume": (|l: &u64| *l == 0),
    authors: "Cannot have more than 25 author entries": (|a: &[String]| a.len() > 25),
}, {
    query:letter,
    year:min_year:max_year,
});

query!(Adapted {
    /// Which MyAnimeList list of adapted manga to return. Default: [MangaAdaptedFilter::All]
    filter: MangaAdaptedFilter,
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by manga type, multiple can be selected
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Only manga with this publishing status
    status: MangaStatus,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only entries with this exact score
    score: f64,
    /// Only entries with this score or higher
    min_score: f64,
    /// Only entries with this score or lower
    max_score: f64,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// Sort the results by...
    order_by: MangaOrder,
    /// Sort direction. Only applies when [MangaQuery::order_by] is used.
    sort: SortDirection,
    /// Only manga that started publishing on or after this date.
    start_date: Date,
    /// Only manga that finished publishing on or before this date. Manga with no end date, *such as one-shots*, are left out.
    end_date: Date,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    score: "Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    max_score: "Max Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
});

query!(MangaIds {
	/// Filter by media type. Multiple types can be selected.
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Only anime with this airing status
    status: MangaStatus,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Fields to include per entry. Allowed: Title, Type
    include: Vec<IDsQueryInclude>:query_vec,
    /// Filter entries updated on or after this ISO 8601 date-time or Unix timestamp. This is as updated on site, not API. May or may not be fully accurate.
    updated_at: String,
}, {});

query!(Character {
    /// The name of the character to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// How to sort the character list
    order_by: BasicOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Only characters whose name start with this letter
    letter: char,
    /// Only entries with this favourite count or higher
    min_favorites: u64,
    /// Only entries with this favourite count or lower
    max_favorites: u64,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(People {
    /// The name of the person to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// How to sort the people list
    order_by: PeopleOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Only people whose name start with this letter
    letter: char,
    /// Only entries with this favourite count or higher
    min_favorites: u64,
    /// Only entries with this favourite count or lower
    max_favorites: u64,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(Producer {
    /// The name of the producer to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// How to sort the producer list
    order_by: ProducerOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Only producers whose name start with this letter
    letter: char,
    /// Only entries with this favourite count or higher
    min_favorites: u64,
    /// Only entries with this favourite count or lower
    max_favorites: u64,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(Seasonal {
    /// Filter by media type
    filter: AnimeType,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Also include unapproved entries.
    unapproved: OnlyBool,
    /// Include TV series continubing from a previous series
    continuing: bool,
    /// Filter out kids entries
    kids: bool,
    /// Field to sort the results by
    order_by: Vec<SeasonOrder>:query_vec,
    /// Order in which to sort.
    sort: SortDirection,
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
    /// Only entries with this member count or higher
    min_members: u64,
    /// Only entries with this member count or lower
    max_members: u64,
    /// Only entries with this episode count or higher
    min_episodes: u64,
    /// Only entries with this episode count or lower
    max_episodes: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(TopAnime {
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Filter by media type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Limit the ranking to a subset
    filter: TopFilter,
    /// Only entries with this member count or higher
    min_members: u64,
    /// Only entries with this member count or lower
    max_members: u64,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
    /// Sort direction.
    sort: SortDirection,
    /// Only entries with this episode count or higher
    min_episodes: u64,
    /// Only entries with this episode count or lower
    max_episodes: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    min_episodes: "Must be at least 1 episode": (|l: &u64| *l == 0),
    max_episodes: "Must be at least 1 episode": (|l: &u64| *l == 0),
});

query!(TopManga {
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Filter by media type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Limit the ranking to a subset
    filter: TopFilter,
    /// Only entries with this member count or higher
    min_members: u64,
    /// Only entries with this member count or lower
    max_members: u64,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u64>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u64>:query_vec,
    /// Sort direction.
    sort: SortDirection,
    /// Only entries with this volume count or higher
    min_volumes: u64,
    /// Only entries with this volume count or lower
    max_volumes: u64,
    /// Only entries with this chapter count or higher
    min_chapters: u64,
    /// Only entries with this chapter count or lower
    max_chapters: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
    min_chapters: "Must be at least 1 chapter": (|l: &u64| *l == 0),
    max_chapters: "Must be at least 1 chapter": (|l: &u64| *l == 0),
    min_volumes: "Must be at least 1 volume": (|l: &u64| *l == 0),
    max_volumes: "Must be at least 1 volume": (|l: &u64| *l == 0),
});

query!(TopReview {
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Sort direction.
    sort: ReviewSort,
    /// Include reviews written before all episodes have been watched
    preliminary: OnlyBool,
    /// Include reviews which might contain spoilers
    spoilers: OnlyBool,
    /// Filter by what the review overall thinks.
    sentiment: Sentiment,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only reviews of the specified type
    entry_type: EntryType:(|_, v| format!("type={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(Genre {
	/// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Filter by category
    filter: GenreFilter,
}, {});

query!(Magazine {
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// The name of the magazine to search for
    query: String:(|_, v| format!("q={v}")),
    /// Only magazines whose name start with this letter
    letter: char,
    /// Order in which to sort.
    sort: SortDirection,
    /// How to sort the results
    order_by: MagazineOrder,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(News {
    /// The name of the article to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only items with this tag. See [TenraiAPI::news_tags](crate::TenraiAPI::news_tags) for the tags.
    tag: String,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(FullArticle {
    /// The name of the article to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only items with this tag. See [TenraiAPI::article_tags](crate::TenraiAPI::article_tags) for the tags.
    tag: String,
    /// Field to sort the results by.
    order_by: ArticleOrdering,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(TopEpisode {
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Order of the list.
    order_by: TopEpisodeOrder,
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});

query!(TypeSfwPage {
    /// Page number
    page: u64,
    /// Results per page (Default: 25)
    limit: u64,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only reviews of the specified type
    entry_type: EntryType:(|_, v| format!("type={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u64| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u64| *l > 100),
});
