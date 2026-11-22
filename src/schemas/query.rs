//! Every struct related to making queries
//!
//! # Query Types
//! There are two different types of queries. `Sub` and `Main`. A `main` query is decided if it's used in an endpoint function, whilst a `sub` is used in a `main` query.
//! A `main` query can be a `sub` query as well.
//!
//! `Sub` queries are not designed to be made by the user.
//!
//! Due to limitations, `sub-queries` may have documentation which is more generalised.
//!
//! # Sealed Traits
//! For convince in making and using sub-queries. A `sealed` trait exists for every query. This trait just provides setter functions and shouldn't be used externally.
//!
//! # Query Data Types
//! Each query comes with 4 different data types
//! - (*Query) One main type that will be passed into functions
//! - (*QueryBuilder) One builder type for building (derive_builder)
//! - (*QueryBuilderError) One BuilderError for building (derive_builder)
//! - (*QueryHelper) One sealed trait for some functions (related to building)
//!
//! Only the main and builder types should be used in code. The helper trait is sealed as it's intended for the builder functions. (it's also not that useful outside of it)
//!
//! The values stored in the main trait are private and can only be gotten as references. This is to prevent editing of values before sending to the API and hence bypassing the verification done in the builder.
//! You can however do something like `QueryBuilder::from(Query)` to convert it back into a builder. This will consume the `Query` struct in doing so but allow modification of the struct once again.

use crate::{
    query, query_extension,
    schemas::{
        anime::{
            AnimeSource, AnimeStatus, AnimeType, ArticleOrdering, EpisodeOrder, ForumOrdering,
            NewsArticleOrdering, OrderBy,
        },
        common::{
            AgeRating, BasicOrder, Date, IDsQueryInclude, Language, Nulls, OnlyBool, SearchMode,
            Season, SortDirection,
        },
        companies::ProducerOrder,
        filter::{EntryType, GenreFilter, MagazineOrder, SeasonOrder, TopEpisodeOrder, TopFilter},
        manga::{MangaAdaptedFilter, MangaOrder, MangaStatus, MangaType},
        people::{CharacterRole, MangaAuthorRole, PeopleOrder},
        query_vec,
        reviews::{ReviewOrder, ReviewReactionEnum, ReviewSort, Sentiment},
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
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
}, {});

query!(
/// Basic paginator query. Allows the selection of a different page.
BasicPage {
    /// Page Number
    page: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
});

query!(
/// Full paginator query. Allows the selection of a different page.
Page {
    /// Page Number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(
/// A combination of [SfwQuery] and [BasicPageQuery]
PageSFW {
    page: u32,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
});

query!(
/// A combination of [SfwQuery] and [PageQuery]
FullPageSFW {
    page: u32,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Results per page (Default: 25)
    limit: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(
    /// Sub query for getting ids.
    IDs {
        /// Minimum MAL ID (inclusive)
        min_id: u32,
        /// Maximum MAL ID (inclusive)
        max_id: u32,
        /// Only search within the entries whose MAL IDs you choose, such as the ones on a user's list. All other filters and sorting still apply.
        // TODO: Don't cache.
        ids: Vec<u32>:query_vec,
        /// Only search within the entries whose MAL IDs you choose, such as the ones on a user's list. All other filters and sorting still apply.
        // TODO: Don't cache.
        exclude_ids: Vec<u32>:query_vec,
    }, {
        /il/ min_id:1,
        /il/ max_id:1,
        /vl/ ids:1000,
        /v/  ids:1,
    }
);

query!(
    /// Sub query for scoring information
    Score {
        /// Only entries with this exact score. Use [min_score](ScoreQuery::min_score) and [max_score](ScoreQuery::max_score) for a range.
        score: f64,
        /// Only entries with this score or higher
        min_score: f64,
        /// Only entries with this score or lower
        max_score: f64,
        /// Only entries that have been scored by this many users (inclusive) (`>=`)
        min_scored_by: u32,
        /// Only entries that have been scored by this many users (inclusive) (`>=`)
        max_scored_by: u32,
    }, {
        score: "Score must be between 1 and 10 (inclusive)": (|l: &f64| *l < 1.0 || *l > 10.0),
        score: "Score must not have more than 2dp": (|s: &f64| (s * 100.0).fract() != 0.0),
        min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l < 0.0 || *l > 10.0),
        min_score: "Min Score must not have more than 2dp": (|s: &f64| (s * 100.0).fract() != 0.0),
        max_score: "Max Score must be between 1 and 10 (inclusive)": (|l: &f64| *l < 1.0 || *l > 10.0),
        max_score: "Max Score must not have more than 2dp": (|s: &f64| (s * 100.0).fract() != 0.0),
    },
    {
        score:min_score:max_score,
    }
);

query!(
    /// Sub query for scoring information
    MiniScore {
        /// Only entries with this exact score. Use [min_score](ScoreQuery::min_score) and [max_score](ScoreQuery::max_score) for a range.
        score: f64,
        /// Only entries with this score or higher
        min_score: f64,
        /// Only entries with this score or lower
        max_score: f64,
    }, {
        score: "Score must be between 1 and 10 (inclusive)": (|l: &f64| *l < 1.0 || *l > 10.0),
        score: "Score must not have more than 2dp": (|s: &f64| (s * 100.0).fract() != 0.0),
        min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l < 0.0 || *l > 10.0),
        min_score: "Min Score must not have more than 2dp": (|s: &f64| (s * 100.0).fract() != 0.0),
        max_score: "Max Score must be between 1 and 10 (inclusive)": (|l: &f64| *l < 1.0 || *l > 10.0),
        max_score: "Max Score must not have more than 2dp": (|s: &f64| (s * 100.0).fract() != 0.0),
    },
    {
        score:min_score:max_score,
    }
);

query!(
    /// Sub query for year information
    Year {
        /// Calendar year of the aired or published start date. Cannot be combined with [min_year](YearQuery::min_year) or [max_year](YearQuery::max_year). For anime, this can differ from MAL's season year. Multiple years match any selected year.
        year:Vec<u32>:query_vec,
        /// Minimum calendar year of the aired or published start date.
        min_year:u32,
        /// Maximum calendar year of the aired or published start date.
        max_year:u32,
    }, {
        year: "Year must be between 1 and 9999 (inclusive)": (|y: &[u32]| y.iter().any(|y| *y < 1 || *y > 9999)),
        min_year: "Min Year must be between 1 and 9999 (inclusive)": (|y: &u32| *y < 1 || *y > 9999),
        max_year: "Max Year must be between 1 and 9999 (inclusive)": (|y: &u32| *y < 1 || *y > 9999),
    }, {
        year:min_year:max_year,
    }
);

query!(Anime {
    /// Title search query (fuzzy matched against English, Japanese and default titles). Cannot be combined with [AnimeQuery::letter]
    query: String:(|_, v| format!("q={v}")),
    /// Only anime whose title starts with this letter. Cannot be combined with [AnimeQuery::query].
    letter: char,

    /// Only entries with any selected type. Multiple values match any selected type.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Exclude entries with any selected type. Multiple values exclude any selected type.
    anime_type_exclude: Vec<AnimeType>:(|_, v| query_vec("type_exclude", v)),

    /// Only entries with any selected status. Multiple values match any selected status.
    status: Vec<AnimeStatus>:query_vec,
    /// Exclude entries with any selected status. Multiple values match any selected status.
    status_exclude: Vec<AnimeStatus>:query_vec,

    /// Only entries with any selected rating. Multiple values match any selected rating.
    rating: Vec<AgeRating>:query_vec,
    /// Exclude entries with any selected rating. Multiple values match any selected rating.
    rating_exclude: Vec<AgeRating>:query_vec,

    /// Only entries with any selected source material. Multiple values match any selected source.
    source: Vec<AnimeSource>:query_vec,
    /// Exclude entries with any selected source material. Multiple values match any selected source.
    source_exclude: Vec<AnimeSource>:query_vec,

    /// Filter by a main trailer ID or URL. Playback availability is not checked.
    has_trailer: bool,
    /// Only anime with a streaming link from any of these providers, e.g. Crunchyroll, Netflix or HIDIVE. Names ignore capitalization, spacing and common spelling variants. Links are not checked for current availability or region.
    streaming_service: Vec<String>:query_vec,

    /// Filter recorded MyAnimeList relations: true includes entries with or without relations, false returns entries with no recorded relations, and only returns entries with at least one recorded relation. Includes anime and manga relations.
    has_relations: OnlyBool,

    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,

    /// List of produces to filter by. Use the ID of the producer.
    producers: Vec<u32>:query_vec,
    /// List of produces to filter by. Use the ID of the producer.
    producers_exclude: Vec<u32>:query_vec,
    /// Filter by credited staff MAL ID. Multiple IDs match any selected person.
    staff: Vec<u32>:query_vec,
    /// Filter by recorded MAL character IDs. Multiple IDs match any selected character.
    characters: Vec<u32>:query_vec,
    /// Only count the characters in characters when MAL lists them in this role. Requires characters.
    character_role: CharacterRole,
    /// Filter by recorded voice-actor MAL person IDs. Multiple IDs match any selected person.
    voice_actors: Vec<u32>:query_vec,
    /// Only anime with a voice cast credited in any of these languages, e.g. an English dub. When used with voice_actors, a selected voice actor must be credited in that language. A credited dub doesn't mean it can be streamed.
    voice_language: Vec<Language>:query_vec,

    /// Query parameters related to IDs.
    *custom: ids: IdsQuery,
    /// Query parameters related to score
    *custom: score: ScoreQuery,

    /// Minimum number of available reviews, inclusive, including spoiler and preliminary reviews. This is not a complete MyAnimeList-wide total. 0 applies no limit.
    min_reviews: u32,
    /// Maximum number of available reviews, inclusive, including spoiler and preliminary reviews. This is not a complete MyAnimeList-wide total. 0 returns entries with no stored reviews.
    max_reviews: u32,
    /// Only entries with this member count or higher
    min_members: u32,
    /// Only entries with this member count or lower
    max_members: u32,
    /// Only entries with this favourite count or higher
    min_favorites: u32,
    /// Only entries with this favourite count or lower
    max_favorites: u32,
    /// Only entries with this episode count or higher
    min_episodes: u32,
    /// Only entries with this episode count or lower
    max_episodes: u32,

    /// Query parameters related to year.
    *custom: year: YearQuery,
    /// Filter by MAL Debut-season classification. Multiple seasons match any.
    season: Vec<Season>:query_vec,
    /// Filter by MAL's debut-season year. This can differ from the calendar start year. Multiple years match any selected year; with season, any selected season of any selected year matches.
    season_year: Vec<u32>:query_vec,
    /// Filter by the day of the week the anime airs in Japan (JST). Multiple days match any selected day. other matches anime that MAL lists without a regular day. Anime without a recorded broadcast day never match. today and tomorrow follow the date in Japan, so they move on at midnight Japan time (15:00 UTC).
    broadcast_day: Vec<ScheduleDayFilter>:query_vec,

    /// Only anime that started airing on or after this date.
    start_date: Date,
    /// Only anime that finished airing on or before this date.
    end_date: Date,
    /// Only anime added on or after this date. Only works with approved anime.
    added_after: Date,
    /// Only anime added before this date (not on it). Only works with approved anime.
    added_before: Date,

    /// Filter audlt themes
    *custom: sfw: SfwQuery,
    /// Also include unapproved entries.
    unapproved: OnlyBool,

    /// Field to sort the results by.
    order_by: OrderBy,
    /// Sort direction. Only applies when [AnimeQuery::order_by] is used.
    sort: SortDirection,

    /// Extra filtering for [AnimeQuery::order_by]. *See [Nulls] for more information*
    nulls: Nulls,

    *custom: page: PageQuery,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),

    rating: "Rating cannot be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
    rating_exclude: "Rating Exclude cannot be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),

    season_year: "All elements must be between 1 (inclusive) and 9999 (inclusive)": (|sy: &[u32]| sy.iter().any(|sy| *sy < 1 || *sy > 9999)),
    broadcast_day: "Can't be \"unknown\".": (|b: &[ScheduleDayFilter]| b.contains(&ScheduleDayFilter::Unknown)),

    /vl/ streaming_service:25,
    /v/  genres:1,
    /v/  genres_exclude:1,
    /v/  producers:1,
    /vl/ producers:25,
    /v/  producers_exclude:1,
    /vl/ producers_exclude:25,
    /v/  staff:1,
    /vl/ staff:25,
    /v/  characters:1,
    /vl/ characters:25,
    /v/  voice_actors:1,
    /vl/ voice_actors:25,
    /il/ min_episodes:1,
    /il/ max_episodes:1,
    /vl/ season_year:25,
}, {
    query:letter,
    // Technically it still works... but eh.
    unapproved:added_after:added_before,
    /nulls:order_by,
});

query_extension!(Anime, Ids);
query_extension!(Anime, Score);
query_extension!(Anime, Year);
query_extension!(Anime, Sfw);
query_extension!(Anime, Page);

query!(
/// Query for IDs but with extra details
ExtendedDetails {
    /// Extra ids to search for at the same time. Returned as (TenraiIncludesResponse::included)[crate::schemas::TenraiIncludesResponse::included]
    ids: Vec<u32>:query_vec,
    *custom:sfw: SfwQuery,
    /// Which archive to search. If omitted, searches the regular catalogue.
    mode: SearchMode,
},{
    /vl/ ids:25,
});

query_extension!(ExtendedDetails, Sfw);

query!(Episode {
    /// Filter by if an episode is a filler (true) or not (false). If not provided will give both fillers and not fillers.
    filler: bool,
    /// Filter by if an episode is a recap (true) or not (false). If not provided will give both recap's and not recap's.
    recap: bool,
    /// Field to sort episodes.
    order_by: EpisodeOrder,
    /// Sort Direction
    sort: SortDirection,
    /// Extra sorting for [EpisodeQuery::order_by]. See [Nulls] for more detail.
    nulls: Nulls,
    /// Page number to search
    *custom:basic_page:BasicPageQuery,
}, {},
{
    /nulls:order_by,
});
query_extension!(Episode, BasicPage);

query!(Review {
    /// Only content that has been written by this MAL username. Matches the full username
    username: String,
    /// Filter by what the review overall thinks.
    sentiment: Vec<Sentiment>: query_vec,
    /// Only reviews with at least [ReviewQuery::min_reaction_count] reactions of all of the selected.
    reaction: Vec<ReviewReactionEnum>: query_vec,
    /// Minimum reactions of each selected kin, Inclusive.
    min_reaction_count: u32,
    /// Minimum total reactions on the review, Inclusive. Uses [Review::reactions::overall](crate::schemas::reviews::ReviewReaction::overall) as the counter.
    min_reactions: u32,
    /// Maximum total reactions on the review, Inclusive. Uses [Review::reactions::overall](crate::schemas::reviews::ReviewReaction::overall) as the counter.
    max_reactions: u32,
    /// Include reviews written before all episodes have been watched. Preliminary reviews are left out if this is not set.
    preliminary: OnlyBool,
    /// Include reviews which might contain spoilers. Spoiler reviews are left out if this is not set.
    spoilers: OnlyBool,
    *custom: mini_score: MiniScoreQuery,
    /// Only reviews published on or after this date/timestamp.
    published_after: String,
    /// Only reviews published on or before this date/timestamp.
    published_before: String,
    /// How to order the results
    order_by: ReviewOrder,
    /// Sort direction.
    sort: ReviewSort,
    /// Extra sorting for [ReviewQuery::order_by]. See [Nulls] for more detail.
    nulls: Nulls,
    *custom:basic_page:BasicPageQuery,

}, {
    username: "Username cannot be more than 30 characters": (|u: &String| u.len() > 30),
    /il/ min_reaction_count: 1,
}, {
    /min_reaction_count:reaction,
});
query_extension!(Review, BasicPage);
query_extension!(Review, MiniScore);

query!(Stack {
    /// The name of the stack to search for
    query: String:(|_, v| format!("q={v}")),
    /// Only content that has been written by this MAL username. Matches the full username, ignoring case.
    author_username: String,
    /// Include stacks that contain spoilers
    spoilers: OnlyBool,
    /// Include stacks that are challenges. NOTE: [StackQuery::official] also needs to not be false.
    challenges: OnlyBool,
    /// Includes stacks that are made by MAL instead of a user.
    official: OnlyBool,
    /// Whether at least one entry has a recorded note.
    has_notes: bool,
    /// Minimum number of entries with a recorded note (inclusive)
    min_noted_entries: u32,
    /// Whether at least one entry has a recorded score.
    has_author_scores: bool,
    /// Minimum number of entries with a recorded score (inclusive)
    min_scored_entries: u32,
    /// Minimum entry count (inclusive)
    min_entries: u32,
    /// Maximum entry count (inclusive)
    max_entries: u32,
    /// Minimum restack count (inclusive)
    min_restacks: u32,
    /// Maximum restack count (inclusive)
    max_restacks: u32,
    /// Only stacks created on or after this date/timestamp.
    created_after: String,
    /// Only stacks created on or before this date/timestamp.
    created_before: String,
    /// Maximum stacks per author using the selected sort before pagination. Stacks without a known author count separately.
    max_per_author: u32,
    *custom:sfw:SfwQuery,
    /// How to sort the stacks.
    stack_order: StackOrder:(|_, v| format!("order_by={v}")),
    /// Order in which to sort.
    sort: SortDirection,
    *custom:page:PageQuery,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    author_username: "Author username cannot be more than 30 characters": (|u: &String| u.len() > 30),
    /il/ max_per_author:1,
});
query_extension!(Stack, Page);
query_extension!(Stack, Sfw);

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
    *custom:page:PageQuery,
    *custom:sfw:SfwQuery,
    /// The name of the anime to search for
    query: String:(|_, v| format!("q={v}")),
    /// How to sort the anime
    order_by: BasicOrder,
    /// Order in which to sort.
    sort: SortDirection,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
});
query_extension!(Basic, Page);
query_extension!(Basic, Sfw);

query!(
/// How to query the forum.  NOTE: This does not include the `filter` param from the official API as that does nothing rn.
Forum {
    /// Page number
    page: u32,
    order_by: ForumOrdering,
    sort: SortDirection,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
});

query!(
/// Query for getting news articles. Similar to [ArticleQuery] however has a different [order_by](NewsArticleQuery::order_by) field.
NewsArticle {
    /// Only content that has been written by this MAL username. Matches the full username
    author_username: String,
    /// Only items published on or after this date/timestamp.
    published_after: String,
    /// Only items published on or before this date/timestamp.
    published_before: String,
    *custom:sfw:SfwQuery,
    /// Field to sort the results by.
    order_by: NewsArticleOrdering,
    /// Direction to sort the results
    sort: SortDirection,
    *custom:page:PageQuery,
}, {
    author_username: "Author username cannot be more than 30 characters": (|u: &String| u.len() > 30),
});
query_extension!(NewsArticle, Page);
query_extension!(NewsArticle, Sfw);

query!(
/// Query for getting articles. Similar to [NewsArticleQuery] however has a different [order_by](ArticleQuery::order_by) field.
Article {
    *custom:page:PageQuery,
    *custom:sfw:SfwQuery,

    /// Field to sort the results by.
    order_by: ArticleOrdering,
    /// Direction to sort the results
    sort: SortDirection,
    /// Only content that has been written by this MAL username. Matches the full username
    author_username: String,
    /// Only items published on or after this date/timestamp.
    published_after: String,
    /// Only items published on or before this date/timestamp.
    published_before: String,
}, {
    author_username: "Author username cannot be more than 30 characters": (|u: &String| u.len() > 30),
});

query_extension!(Article, Page);
query_extension!(Article, Sfw);

query!(
/// Query for getting historical stats.
StatsHistory {
	*custom:page:PageQuery,
    /// Direction to sort the results
    sort: SortDirection,
    /// Only days **on** or **after** this date
    start_date: Date,
    /// Only days **on** or **before** this date
    end_date: Date,
}, {});
query_extension!(StatsHistory, Page);

query!(Manga {
    /// The name of the anime to search for
    query: String:(|_, v| format!("q={v}")),
    /// Only manga whose title starts with this letter. Cannot be combined with [MangaQuery::query].
    letter: char,
    /// Only manga with this type.
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Exclude manga with any of these types
    manga_type_exclude: Vec<MangaType>:(|_, v| query_vec("type_exclude", v)),
    /// Only manga with this publishing status
    status: Vec<MangaStatus>:query_vec,
    /// Exclude manga with this publishing status
    status_exclude: Vec<MangaStatus>:query_vec,
    /// Filter recorded MyAnimeList relations: true includes entries with or without relations, false returns entries with no recorded relations, and only returns entries with at least one recorded relation. Includes anime and manga relations.
    has_relations: OnlyBool,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Magazines this manga has been published in. Use the ID of the magazine.
    magazines: Vec<u32>:query_vec,
    /// Exclude manga if it has been published in any of these magazines. Use the ID of the magazine
    magazines_exclude: Vec<u32>:query_vec,
    /// Filter by credited author or artist. If multiple are provided will match any of them. Requires MAL person IDs
    authors: Vec<u32>: query_vec,
    /// Only count the authors in [MangaQuery::authors] when MAL credits them with this role. Author credited with both matches both.
    author_role: MangaAuthorRole,
    #[doc="Filter by who made the manga\n\n- True: Written and drawn by one person\n\n- False: Writing and drawing creddited to multiple.\n\nManga that MAL lists without both a writer and artist match neither"]
    solo_author:bool,
    /// Exclude manga by any of these authors in any role.
    authors_exclude: Vec<u32>: query_vec,
    /// Filter by characters that appear in the manga.
    characters: Vec<u32>:query_vec,
    /// Only count the character when MAL lists them as this role.
    character_role: CharacterRole,
    *custom:ids:IdsQuery,
    *custom:score:ScoreQuery,
    /// Minimum number of available reviews including all types. Not a complete total. 0/NA = no limit.
    min_reviews: u32,
    /// Maximum number of available reviews including all types. Not a complete total. 0 returns entries with no reviews.
    max_reviews: u32,
    /// Only manga with this member count or higher
    min_members: u32,
    /// Only manga with this member count or lower
    max_members: u32,
    /// Only manga with this favourite count or higher
    min_favorites: u32,
    /// Only manga with this favourite count or lower
    max_favorites: u32,
    /// Only manga with this chapter count or higher
    min_chapters: u32,
    /// Only manga with this chapter count or lower
    max_chapters: u32,
    /// Only manga with this volume count or higher
    min_volumes: u32,
    /// Only manga with this volume count or lower
    max_volumes: u32,

    *custom:year:YearQuery,
    /// Only manga that started publishing on or after this date.
    start_date: Date,
    /// Only manga that finished publishing on or before this date. Manga with no end date, *such as one-shots*, are left out.
    end_date: Date,

    *custom:sfw:SfwQuery,
    /// Also include unapproved entries
    unapproved: bool,
    /// Sort the results by...
    order_by: MangaOrder,
    /// Sort direction. Only applies when [MangaQuery::order_by] is used.
    sort: SortDirection,
    /// Extra filtering for [MangaQuery::order_by]*See [Nulls] for more information*
    nulls: Nulls,

    *custom:page:PageQuery,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    min_volumes: "Must be at least 1 volume": (|l: &u32| *l == 0),
    max_volumes: "Must be at least 1 volume": (|l: &u32| *l == 0),
    author_role: "Role cannot be both": (|r: &MangaAuthorRole| *r == MangaAuthorRole::Both),

    /v/  genres:1,
    /v/  genres_exclude:1,
    /v/  magazines:1,
    /vl/ magazines:25,
    /v/  magazines_exclude:1,
    /vl/ magazines_exclude:25,
    /v/  authors:1,
    /vl/ authors:25,
    /v/  authors_exclude:1,
    /vl/ authors_exclude:25,
    /v/  characters:1,
    /vl/ characters:25,
    /il/ min_chapters:1,
    /il/ max_chapters:1,
    /il/ min_volumes:1,
    /il/ max_volumes:1,
}, {
    query:letter,

    /author_role:authors,
    /character_role:characters,
    /nulls:order_by,
});
query_extension!(Manga, Page);
query_extension!(Manga, Ids);
query_extension!(Manga, Score);
query_extension!(Manga, Sfw);
query_extension!(Manga, Year);

query!(Adapted {
    /// Which MyAnimeList list of adapted manga to return. Default: [MangaAdaptedFilter::All]
    filter: MangaAdaptedFilter,
    /// Filter by manga type, multiple can be selected
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Exclude entries with this type.
    manga_type_exclude: Vec<MangaType>:(|_, v| query_vec("type_exclude", v)),
    /// Only manga with this publishing status
    status: Vec<MangaStatus>:query_vec,
    /// Exclude manga with these publishing status
    status_exclude: Vec<MangaStatus>:query_vec,

    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Magazines this manga has been published in. Use the ID of the magazine.
    magazines: Vec<u32>:query_vec,
    /// Exclude manga if it has been published in any of these magazines. Use the ID of the magazine
    magazines_exclude: Vec<u32>:query_vec,
    #[doc="Filter by who made the manga\n\n- True: Written and drawn by one person\n\n- False: Writing and drawing creddited to multiple.\n\nManga that MAL lists without both a writer and artist match neither"]
    solo_author:bool,
    /// Exclude the entries whose MAL IDs you choose, such as the ones already on a user's list. Comma-separated. TODO: Never cache. *server doesn't cache*
    exclude_ids: Vec<u32>:query_vec,

    *custom:score:ScoreQuery,
    /// Minimum number of available reviews including all types. Not a complete total. 0/NA = no limit.
    min_reviews: u32,
    /// Maximum number of available reviews including all types. Not a complete total. 0 returns entries with no reviews.
    max_reviews: u32,
    /// Only manga with this member count or higher
    min_members: u32,
    /// Only manga with this member count or lower
    max_members: u32,
    /// Only manga with this favourite count or higher
    min_favorites: u32,
    /// Only manga with this favourite count or lower
    max_favorites: u32,
    /// Only manga with this chapter count or higher
    min_chapters: u32,
    /// Only manga with this chapter count or lower
    max_chapters: u32,
    /// Only manga with this volume count or higher
    min_volumes: u32,
    /// Only manga with this volume count or lower
    max_volumes: u32,


    *custom:year:YearQuery,
    /// Only manga that started publishing on or after this date.
    start_date: Date,
    /// Only manga that finished publishing on or before this date. Manga with no end date, *such as one-shots*, are left out.
    end_date: Date,

    *custom:sfw:SfwQuery,

    /// Sort the results by...
    order_by: MangaOrder,
    /// Sort direction. Only applies when [MangaQuery::order_by] is used.
    sort: SortDirection,
    /// Extra filtering for [AnimeQuery::order_by]. *See [Nulls] for more information*
    nulls: Nulls,

    *custom:page:PageQuery,
}, {
    manga_type: "Cannot be light novel or novel":(|t: &[MangaType]| t.iter().any(|t| *t == MangaType::LightNovel || *t == MangaType::Novel)),
    manga_type_exclude: "Cannot be light novel or novel":(|t: &[MangaType]| t.iter().any(|t| *t == MangaType::LightNovel || *t == MangaType::Novel)),

    /v/  genres:1,
    /v/  genres_exclude:1,
    /v/  magazines:1,
    /vl/ magazines:25,
    /v/  magazines_exclude:1,
    /vl/ magazines_exclude:25,
    /v/  exclude_ids:1,
    /vl/ exclude_ids:1000,
    /il/ min_chapters:1,
    /il/ max_chapters:1,
    /il/ min_volumes:1,
    /il/ max_volumes:1,
}, {
    /nulls:order_by,
});
query_extension!(Adapted, Page);
query_extension!(Adapted, Sfw);
query_extension!(Adapted, Score);
query_extension!(Adapted, Year);

query!(MangaIds {
	/// Filter by media type. Multiple types can be selected.
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Only anime with this airing status
    status: MangaStatus,
    *custom:sfw:SfwQuery,
    /// Fields to include per entry. Allowed: Title, Type
    include: Vec<IDsQueryInclude>:query_vec,
    /// Filter entries updated on or after this ISO 8601 date-time or Unix timestamp. This is as updated on site, not API. May or may not be fully accurate.
    updated_at: String,
}, {});
query_extension!(MangaIds, Sfw);

query!(Character {
    /// The name of the character to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// How to sort the character list
    order_by: BasicOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Only characters whose name start with this letter
    letter: char,
    /// Only entries with this favourite count or higher
    min_favorites: u32,
    /// Only entries with this favourite count or lower
    max_favorites: u32,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(People {
    /// The name of the person to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// How to sort the people list
    order_by: PeopleOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Only people whose name start with this letter
    letter: char,
    /// Only entries with this favourite count or higher
    min_favorites: u32,
    /// Only entries with this favourite count or lower
    max_favorites: u32,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(Producer {
    /// The name of the producer to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// How to sort the producer list
    order_by: ProducerOrder,
    /// Order in which to sort.
    sort: SortDirection,
    /// Only producers whose name start with this letter
    letter: char,
    /// Only entries with this favourite count or higher
    min_favorites: u32,
    /// Only entries with this favourite count or lower
    max_favorites: u32,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
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
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// Filter by anime type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
    rating: "Rating can not be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
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
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Only entries with this member count or higher
    min_members: u32,
    /// Only entries with this member count or lower
    max_members: u32,
    /// Only entries with this episode count or higher
    min_episodes: u32,
    /// Only entries with this episode count or lower
    max_episodes: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(TopAnime {
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
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
    min_members: u32,
    /// Only entries with this member count or lower
    max_members: u32,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Sort direction.
    sort: SortDirection,
    /// Only entries with this episode count or higher
    min_episodes: u32,
    /// Only entries with this episode count or lower
    max_episodes: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
    min_episodes: "Must be at least 1 episode": (|l: &u32| *l == 0),
    max_episodes: "Must be at least 1 episode": (|l: &u32| *l == 0),
});

query!(TopManga {
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
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
    min_members: u32,
    /// Only entries with this member count or lower
    max_members: u32,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Sort direction.
    sort: SortDirection,
    /// Only entries with this volume count or higher
    min_volumes: u32,
    /// Only entries with this volume count or lower
    max_volumes: u32,
    /// Only entries with this chapter count or higher
    min_chapters: u32,
    /// Only entries with this chapter count or lower
    max_chapters: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
    min_chapters: "Must be at least 1 chapter": (|l: &u32| *l == 0),
    max_chapters: "Must be at least 1 chapter": (|l: &u32| *l == 0),
    min_volumes: "Must be at least 1 volume": (|l: &u32| *l == 0),
    max_volumes: "Must be at least 1 volume": (|l: &u32| *l == 0),
});

query!(TopReview {
    /// Only content that has been written by this MAL username. Matches the full username
    username: String,
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
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
    username: "Username cannot be more than 300 characters": (|u: &String| u.len() > 300),
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
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
    /// The name of the magazine to search for
    query: String:(|_, v| format!("q={v}")),
    /// Only magazines whose title starts with this letter. Cannot be combined with [MagazineQuery::query].
    letter: char,

    *custom:sfw:SfwQuery,
    /// How to sort the results
    order_by: MagazineOrder,
    /// Order in which to sort.
    sort: SortDirection,

    *custom:page:PageQuery,
}, {
    query: "Query length is too long": (|q: &String| q.len() > 300),
}, {
    query:letter,
});
query_extension!(Magazine, Sfw);
query_extension!(Magazine, Page);

query!(News {
    /// The name of the article to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only items with this tag. See [TenraiAPI::news_tags](crate::TenraiAPI::news_tags) for the tags.
    tag: String,
    /// Only content that has been written by this MAL username. Matches the full username
    author_username: String,
}, {
    author_username: "Author username cannot be more than 300 characters": (|u: &String| u.len() > 300),
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(FullArticle {
    /// The name of the article to search for
    query: String:(|_, v| format!("q={v}")),
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only items with this tag. See [TenraiAPI::article_tags](crate::TenraiAPI::article_tags) for the tags.
    tag: String,
    /// Field to sort the results by.
    order_by: ArticleOrdering,
    /// Only content that has been written by this MAL username. Matches the full username
    author_username: String,
}, {
    author_username: "Author username cannot be more than 300 characters": (|u: &String| u.len() > 300),
    query: "Query length is too long": (|q: &String| q.len() > 300),
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(TopEpisode {
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Order of the list.
    order_by: TopEpisodeOrder,
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(TypeSfwPage {
    /// Page number
    page: u32,
    /// Results per page (Default: 25)
    limit: u32,
    /// Filter out: R+, Rx, Hentai, Erotica
    sfw: bool,
    /// Filter out: R+, Rx, Hentai, Erotica, Ecchi
    sfw_struct: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only reviews of the specified type
    entry_type: EntryType:(|_, v| format!("type={v}")),
}, {
    page: "Page must be less than 20k": (|p: &u32| *p > 20000),
    limit: "Can not request more than 100 entries at once": (|l: &u32| *l > 100),
});

query!(RandomAnime {
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Filter by media type. Multiple types can be selected.
    anime_type: Vec<AnimeType>:(|_, v| query_vec("type", v)),
    /// Only anime with this airing status
    status: AnimeStatus,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Only entries with this score or higher
    min_score: f64,
    /// Only entries with this score or lower
    max_score: f64,
    /// Calendar year of the aired date. Cannot be combined with [RandomAnimeQuery::min_year] or [RandomAnimeQuery::max_year]
    year: u32,
    /// Anime that aired this *provided* year or later
    min_year: u32,
    /// Anime that aired this *provided* year or earlier
    max_year: u32,
    /// Filter by source material. If multiple provided, will match with any of them.
    source: Vec<AnimeSource>:query_vec,
    /// Filter by age rating. Multiple ratings can be selected.
    rating: Vec<AgeRating>:query_vec,
    /// Filter by a main trailer ID or URL. Playback availability is not checked.
    has_trailer: bool,
    /// List of produces to filter by. Use the ID of the producer.
    producers: Vec<u32>:query_vec,
}, {
    min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    max_score: "Max Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    rating: "Rating can not be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
    source: "Can only have 25 types of sources": (|s: &[AnimeSource]| s.len() > 25),
}, {
    year:min_year:max_year,
});

query!(RandomManga {
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Filter by manga type, multiple can be selected
    manga_type: Vec<MangaType>:(|_, v| query_vec("type", v)),
    /// Only manga with this publishing status
    status: MangaStatus,
    /// List of genres to filter by. Use the ID of the genre.
    genres: Vec<u32>:query_vec,
    /// List of genres to exclude. Use the ID of the genre.
    genres_exclude: Vec<u32>:query_vec,
    /// Only entries with this score or higher
    min_score: f64,
    /// Only entries with this score or lower
    max_score: f64,
    /// Calendar year of the published start date. Cannot be combined with [RandomMangaQuery::min_year] or [RandomMangaQuery::max_year]
    year: u32,
    /// Manga that started publishing this *provided* year or later
    min_year: u32,
    /// Manga that started publishing this *provided* year or earlier
    max_year: u32,
    /// Filter by credited author or artist. If multiple are provided will match any of them. Requires MAL person IDs
    authors: Vec<String>: query_vec,
}, {
    min_score: "Min Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    max_score: "Max Score must be between 0 and 10 (inclusive)": (|l: &f64| *l >= 0.0 && *l <= 10.0),
    authors: "Cannot have more than 25 author entries": (|a: &[String]| a.len() > 25),
}, {
    year:min_year:max_year,
});

query!(RandomPerson {
 	/// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only entries with this favourite count or higher
    min_favorites: u32,
    /// Only entries with this favourite count or lower
    max_favorites: u32,
}, {});

query!(Recommendation {
    /// Filter adult content
    sfw: bool,
    /// Apply strict SFW filtering for adult content and Ecchi
    sfw_strict: bool:(|_, v| format!("sfw-strict={v}")),
    /// Only content that has been written by this MAL username. Matches the full username
    username: String,
}, {
    username: "Username cannot be more than 30 characters": (|u: &String| u.len() > 30),
});
