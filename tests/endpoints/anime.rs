use rand::{RngExt, distr::Alphabetic, rngs::ThreadRng};
use tenrai::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse,
        anime::{Anime, AnimeEpisode, AnimeSource, AnimeStatus, AnimeType, EpisodeOrder},
        article::NewsArticle,
        common::{AgeRating, Date, Language, Nulls, OnlyBool, Season},
        people::CharacterRole,
        query::{
            AnimeQuery, AnimeQueryBuilder, ArticleQuery, BasicPageQuery, BasicQuery, EpisodeQuery,
            EpisodeQueryBuilder, ExtendedDetailsQuery, ForumQuery, IdsQueryHelper,
            NewsArticleQuery, NewsArticleQueryBuilder, PageQueryHelper, RecommendationQuery,
            ReviewQuery, ReviewQueryBuilder, ScoreQueryHelper, SfwQuery, SfwQueryHelper,
            StackQuery, StackQueryBuilder, StatsHistoryQuery, YearQueryHelper,
        },
        reviews::{Review, ReviewReactionEnum, Sentiment},
        stacks::Stack,
    },
};

use crate::{TestBuilder, TestQuery, build_client};

#[tokio::test]
#[ignore = "This sends upwards of 300 requests hence takes like a minute or two."]
async fn anime_search() {
    let client = build_client();
    let mut page = 1_u32;
    loop {
        println!("{page}");
        let result = client
            .anime_search(
                AnimeQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
            .expect("This should not fail");
        page += 1;
        if !result.pagination.has_next_page {
            break;
        }
    }
}

TestBuilder!(
    anime_full_details,
    (async |client: &TenraiAPI, rand| {
        client
            .anime_full_details(
                rand,
                ExtendedDetailsQuery::builder()
                    .ids(
                        rand::random_iter()
                            .take(5)
                            .map(|s: i32| s.abs() as u32)
                            .collect::<Vec<u32>>(),
                    )
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    anime_recommendation,
    (async |client: &TenraiAPI, id| client
        .anime_recommendations(id, RecommendationQuery::default())
        .await)
);
TestBuilder!(
    anime_characters,
    (async |client: &TenraiAPI, id| client.anime_characters(id).await)
);
TestBuilder!(
    anime_staff,
    (async |client: &TenraiAPI, id| client.anime_staff(id).await)
);
TestBuilder!(
    anime_episodes,
    (async |client: &TenraiAPI, id| client.anime_episodes(id, EpisodeQuery::default()).await)
);
TestBuilder!(
    anime_episodes_nertff,
    (async |client: &TenraiAPI, id| client
        .anime_episodes(
            id,
            EpisodeQuery::builder()
                .order_by(EpisodeOrder::MalId)
                .nulls(Nulls::Exclude)
                .recap(true)
                .filler(false)
                .build()
                .unwrap()
        )
        .await)
);
TestBuilder!(
    anime_news,
    (async |client: &TenraiAPI, id| client.anime_news(id, NewsArticleQuery::default()).await)
);
TestBuilder!(
    anime_articles,
    (async |client: &TenraiAPI, id| client.anime_articles(id, ArticleQuery::default()).await)
);
TestBuilder!(
    anime_forum,
    (async |client: &TenraiAPI, id| client.anime_forum(id, ForumQuery::default()).await)
);
TestBuilder!(
    anime_episode,
    (async |client: &TenraiAPI, id| client.anime_episode(id, &1).await)
);
TestBuilder!(
    anime_video,
    (async |client: &TenraiAPI, id| client.anime_videos(id).await)
);
TestBuilder!(
    anime_episode_video,
    (async |client: &TenraiAPI, id| client
        .anime_video_episodes(id, BasicPageQuery::default())
        .await)
);
TestBuilder!(
    anime_pictures,
    (async |client: &TenraiAPI, id| client.anime_pictures(id).await)
);
TestBuilder!(
    anime_statistics,
    (async |client: &TenraiAPI, id| client.anime_statistics(id).await)
);
TestBuilder!(
    anime_statistics_historic,
    (async |client: &TenraiAPI, id| client
        .anime_statistics_historical(id, StatsHistoryQuery::default())
        .await)
);
TestBuilder!(
    anime_more_info,
    (async |client: &TenraiAPI, id| client.anime_more_info(id).await)
);
TestBuilder!(
    anime_relations,
    (async |client: &TenraiAPI, id| client.anime_relations(id, SfwQuery::default()).await)
);
TestBuilder!(
    anime_themes,
    (async |client: &TenraiAPI, id| client.anime_themes(id).await)
);
TestBuilder!(
    anime_external,
    (async |client: &TenraiAPI, id| client.anime_external(id).await)
);
TestBuilder!(
    anime_streaming,
    (async |client: &TenraiAPI, id| client.anime_streaming(id).await)
);
TestBuilder!(
    anime_reviews,
    (async |client: &TenraiAPI, id| client.anime_reviews(id, ReviewQuery::default()).await)
);
TestBuilder!(
    anime_stacks,
    (async |client: &TenraiAPI, id| client.anime_stacks(id, StackQuery::default()).await)
);
TestBuilder!(
    anime_unapproved,
    (async |client: &TenraiAPI, _| client.anime_unapproved(BasicQuery::default()).await)
);
TestBuilder!(
    anime_denied,
    (async |client: &TenraiAPI, _| client.anime_denied(BasicQuery::default()).await)
);
TestBuilder!(
    anime_deleted,
    (async |client: &TenraiAPI, _| client.anime_deleted(BasicQuery::default()).await)
);
// TODO: Randomise these more.
TestQuery!(anime_search, AnimeQuery,
    (async |client: &TenraiAPI, query: AnimeQuery| client.anime_search(query).await),
    (|res: TenraiPaginatorResponse<Anime>| res.pagination.items.unwrap().total),
    {
        query:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.query((0..7).map(|_| rand.sample(Alphabetic) as char).collect::<String>());}),
        letter:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.letter(rand.sample(Alphabetic) as char);}),
        anime_type:(|builder: &mut AnimeQueryBuilder, _| {builder.anime_type(vec![AnimeType::Tv]);}),
        anime_type_exclude:(|builder: &mut AnimeQueryBuilder, _| {builder.anime_type_exclude(vec![AnimeType::Ova]);}),
        status:(|builder: &mut AnimeQueryBuilder, _| {builder.status(vec![AnimeStatus::Airing]);}),
        status_exclude:(|builder: &mut AnimeQueryBuilder, _| {builder.status_exclude(vec![AnimeStatus::Complete]);}),
        rating:(|builder: &mut AnimeQueryBuilder, _| {builder.rating(vec![AgeRating::Rx]);}),
        rating_exclude:(|builder: &mut AnimeQueryBuilder, _| {builder.rating_exclude(vec![AgeRating::Pg]);}),
        source:(|builder: &mut AnimeQueryBuilder, _| {builder.source(vec![AnimeSource::Original]);}),
        source_exclude:(|builder: &mut AnimeQueryBuilder, _| {builder.source_exclude(vec![AnimeSource::LightNovel]);}),
        has_trailer: (|builder: &mut AnimeQueryBuilder, _| {builder.has_trailer(true);}),
        streaming_service: (|builder: &mut AnimeQueryBuilder, _| {builder.streaming_service(vec!{"Cr".to_string()});}),
        has_relations: (|builder: &mut AnimeQueryBuilder, _| {builder.has_relations(OnlyBool::Only);}),
        genres:(|builder: &mut AnimeQueryBuilder, _| {builder.genres(vec![2]);}),
        genres_exclude:(|builder: &mut AnimeQueryBuilder, _| {builder.genres_exclude(vec![4]);}),
        staff:(|builder: &mut AnimeQueryBuilder, _| {builder.staff(vec![4]);}),
        characters:(|builder: &mut AnimeQueryBuilder, _| {builder.characters(vec![4]);}),
        character_role:(|builder: &mut AnimeQueryBuilder, _| {builder.character_role(CharacterRole::Main).characters(vec![155904, 211945, 117225]);}),
        voice_actors:(|builder: &mut AnimeQueryBuilder, _| {builder.voice_actors(vec![6, 3]);}),
        voice_language:(|builder: &mut AnimeQueryBuilder, _| {builder.voice_language(vec![Language::English]);}),
        min_id:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_id(rand.random::<u16>());}),
        max_id:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_id(rand.random::<u16>());}),
        ids:(|builder: &mut AnimeQueryBuilder, _| {builder.ids(vec![2309]);}),
        exclude_ids:(|builder: &mut AnimeQueryBuilder, _| {builder.exclude_ids(vec![5439]);}),
        score:(|builder: &mut AnimeQueryBuilder, _| {builder.score(1.23);}),
        min_score:(|builder: &mut AnimeQueryBuilder, _| {builder.min_score(1.0);}),
        max_score:(|builder: &mut AnimeQueryBuilder, _| {builder.max_score(2.34);}),
        min_scored_by:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_scored_by(rand.random::<u16>());}),
        max_scored_by:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_scored_by(rand.random::<u16>());}),
        min_reviews:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_reviews(rand.random::<u8>());}),
        max_reviews:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_reviews(rand.random::<u8>());}),
        min_members:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_members(rand.random::<u16>());}),
        max_members:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_members(rand.random::<u16>());}),
        min_favorites:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_favorites(rand.random::<u16>());}),
        max_favorites:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_favorites(rand.random::<u16>());}),
        min_episodes:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_episodes(rand.random::<u8>());}),
        max_episodes:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_episodes(rand.random::<u8>());}),
        year:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.year(vec![rand.random_range(1900_u32..2100_u32)]);}),
        min_year:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.min_year(rand.random_range(1900_u32..2100_u32));}),
        max_year:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.max_year(rand.random_range(1900_u32..2100_u32));}),
        season:(|builder: &mut AnimeQueryBuilder, _| {builder.season(vec![Season::Spring]);}),
        season_year:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.season_year(vec![rand.random_range(1900_u32..2100_u32)]);}),
        start_date:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.start_date(Date::new(rand.random_range(1900_u16..2100_u16), rand.random_range(1_u8..12_u8), None));}),
        end_date:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.end_date(Date::new(rand.random_range(1900_u16..2100_u16), rand.random_range(1_u8..12_u8), None));}),
        added_after:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.added_after(Date::new(rand.random_range(2010_u16..2100_u16), rand.random_range(1_u8..12_u8), None));}),
        added_before:(|builder: &mut AnimeQueryBuilder, rand: &mut ThreadRng| {builder.added_before(Date::new(rand.random_range(1900_u16..2025_u16), rand.random_range(1_u8..12_u8), None));}),
        sfw:(|builder: &mut AnimeQueryBuilder, _| {builder.sfw(true);}),
        sfw_strict:(|builder: &mut AnimeQueryBuilder, _| {builder.sfw_strict(true);}),
        unapproved:(|builder: &mut AnimeQueryBuilder, _| {builder.unapproved(true);}),
        // The rest don't do anything useful here.
    }
);

TestQuery!(
    anime_episodes, EpisodeQuery,
    (async |client: &TenraiAPI, query: EpisodeQuery| client.anime_episodes(&6547_u32, query).await),
    (|res: TenraiPaginatorResponse<AnimeEpisode>| res.data.len()),
    {
        filler:(|builder: &mut EpisodeQueryBuilder, _| {builder.filler(true);}),
        recap:(|builder: &mut EpisodeQueryBuilder, _| {builder.recap(true);}),
    }
);

TestQuery!(
    anime_news, NewsArticleQuery,
    (async |client: &TenraiAPI, query: NewsArticleQuery| client.anime_news(&6547_u32, query).await),
    (|res: TenraiPaginatorResponse<NewsArticle>| res.data.len()),
    {
        author_username:(|builder: &mut NewsArticleQueryBuilder, _| {builder.author_username("Naruleach");}),
        published_after:(|builder: &mut NewsArticleQueryBuilder, _| {builder.published_after("2011-01-01");}),
        published_before:(|builder: &mut NewsArticleQueryBuilder, _| {builder.published_before("2011-01-01");}),
    }
);

TestQuery!(
    anime_reviews, ReviewQuery,
    (async |client: &TenraiAPI, query: ReviewQuery| client.anime_reviews(&6547_u32, query).await),
    (|res: TenraiPaginatorResponse<Review>| res.pagination.last_visible_page),
    {
        username:(|builder: &mut ReviewQueryBuilder, _| {builder.username("coldcoal");}),
        sentiment:(|builder: &mut ReviewQueryBuilder, _| {builder.sentiment(vec![Sentiment::Recommended]);}),
        reaction: (|builder: &mut ReviewQueryBuilder, _| {builder.reaction(vec![ReviewReactionEnum::Funny]);}),
        min_reaction_count: (|builder: &mut ReviewQueryBuilder, rng: &mut ThreadRng| {builder.reaction(vec![ReviewReactionEnum::Funny]).min_reaction_count(rng.random_range(10_u32..200_u32));}),
        min_reactions: (|builder: &mut ReviewQueryBuilder, rng: &mut ThreadRng| {builder.min_reactions(rng.random_range(1000_u32..1500_u32));}),
        max_reactions: (|builder: &mut ReviewQueryBuilder, rng: &mut ThreadRng| {builder.max_reactions(rng.random_range(0_u32..300_u32));}),
        preliminary:(|builder: &mut ReviewQueryBuilder, _| {builder.preliminary(true);}),
        spoilers:(|builder: &mut ReviewQueryBuilder, _| {builder.spoilers(true);}),
        published_after:(|builder: &mut ReviewQueryBuilder, _| {builder.published_after("2020-05-13");}),
        published_before:(|builder: &mut ReviewQueryBuilder, _| {builder.published_before("2010-05-13");}),
    }
);

TestQuery!(
    anime_stacks,
    StackQuery,
    (async |client: &TenraiAPI, query: StackQuery| client.anime_stacks(&6547_u32, query).await),
    (|res: TenraiPaginatorResponse<Stack>| res.pagination.items.unwrap().count),
    {
        author_username:(|builder: &mut StackQueryBuilder, _| {builder.author_username("Plaxin");}),
        spoilers:(|builder:&mut StackQueryBuilder, _| {builder.spoilers(true);}),
        challenges:(|builder:&mut StackQueryBuilder, _| {builder.challenges(true);}),
        official:(|builder:&mut StackQueryBuilder, _| {builder.official(true);}),
        has_notes:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.has_notes(rng.random::<bool>());}),
        min_noted_entries:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.min_noted_entries(rng.random_range(0_u32..50_u32));}),
        has_author_scores:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.has_author_scores(rng.random::<bool>());}),
        min_scored_entries:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.min_scored_entries(rng.random_range(0_u32..50_u32));}),
        min_entries:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.min_entries(rng.random_range(0_u32..50_u32));}),
        max_entries:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.max_entries(rng.random_range(0_u32..50_u32));}),
        min_restacks:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.min_restacks(rng.random_range(0_u32..700_u32));}),
        max_restacks:(|builder:&mut StackQueryBuilder, rng: &mut ThreadRng| {builder.max_restacks(rng.random_range(0_u32..700_u32));}),
        created_after:(|builder: &mut StackQueryBuilder, _| {builder.created_after("2020-05-13");}),
        created_before:(|builder: &mut StackQueryBuilder, _| {builder.created_before("2010-05-13");}),
        max_per_author:(|builder: &mut StackQueryBuilder, rng: &mut ThreadRng| {builder.max_per_author(rng.random_range(2_u32..100_u32));}),
    }
);
