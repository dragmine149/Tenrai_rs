use rand::{
    RngExt,
    distr::{Alphabetic, Distribution},
};
use tenrai::{
    TenraiAPI,
    schemas::{
        anime::EpisodeOrder,
        common::{IDsQueryInclude, Nulls},
        query::{
            AnimeIdsQuery, AnimeQuery, ArticleQuery, BasicPageQuery, BasicQuery, EpisodeQuery,
            ExtendedDetailsQuery, ForumQuery, NewsArticleQuery, PageQueryHelper,
            RecommendationQuery, ReviewQuery, SfwQuery, StackQuery, StatsHistoryQuery,
        },
    },
};

use crate::{TestBuilder, build_client};

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
TestBuilder!(
    anime_ids,
    (async |client: &TenraiAPI, _| client.anime_ids(AnimeIdsQuery::default()).await),
    ignore = "Requires Server key"
);
TestBuilder!(
    anime_ids_information,
    (async |client: &TenraiAPI, _| client
        .anime_ids(
            AnimeIdsQuery::builder()
                .include([IDsQueryInclude::Title, IDsQueryInclude::Type])
                .build()
                .unwrap()
        )
        .await),
    ignore = "Requires Server key"
);

#[tokio::test]
async fn test_anime_query() {
    let client = build_client();
    let base_data = client
        .anime_search(AnimeQuery::default())
        .await
        .expect("This should not fail")
        .pagination
        .items
        .unwrap()
        .total;
    let mut rng = rand::rng();

    // AnimeQuery::builder().added_after.unwrap_or_default()

    let query = client
        .anime_search(
            AnimeQuery::builder()
                .query(
                    (0..7)
                        .map(|_| rng.sample(Alphabetic) as char)
                        .collect::<String>(),
                )
                .build()
                .unwrap(),
        )
        .await
        .expect("This should not fail")
        .pagination
        .items
        .unwrap()
        .total;
    assert_ne!(base_data, query);
    let letter = client
        .anime_search(
            AnimeQuery::builder()
                .letter(rng.sample(Alphabetic) as char)
                .build()
                .unwrap(),
        )
        .await
        .expect("This should not fail")
        .pagination
        .items
        .unwrap()
        .total;
    assert_ne!(base_data, letter);
}
