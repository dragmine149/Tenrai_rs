use tenrai::{
    TenraiAPI,
    errors::{MachineErrorType, TenraiError},
    schemas::{
        anime::AnimeIDsQueryInclude,
        query::{
            AnimeIdsQuery, AnimeQuery, AnimeReviewQuery, ArticleQuery, BasicAnimeQuery,
            BasicPageQuery, ForumQuery, PageSfwQuery, SfwQuery, StackQuery, StatsHistoryQuery,
        },
    },
};

use crate::{TestBuilder, build_client};

#[tokio::test]
#[ignore = "This sends upwards of 300 requests hence takes like a minute or two."]
async fn anime_search() {
    let client = build_client();
    let mut page = 1_u64;
    loop {
        println!("{page}");
        let result = client
            .anime_search(
                AnimeQuery::builder()
                    .page(page)
                    .limit(100_u64)
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
    (async |client: &TenraiAPI, rand| client.anime_full_details(rand, SfwQuery::default()).await)
);
TestBuilder!(
    anime_recommendation,
    (async |client: &TenraiAPI, id| client.anime_recommendations(id, SfwQuery::default()).await)
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
    (async |client: &TenraiAPI, id| client.anime_episodes(id, BasicPageQuery::default()).await)
);
TestBuilder!(
    anime_news,
    (async |client: &TenraiAPI, id| client.anime_news(id, PageSfwQuery::default()).await)
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
    (async |client: &TenraiAPI, id| client.anime_episode(id, 1).await)
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
    (async |client: &TenraiAPI, id| client.anime_reviews(id, AnimeReviewQuery::default()).await)
);
TestBuilder!(
    anime_stacks,
    (async |client: &TenraiAPI, id| client.anime_stacks(id, StackQuery::default()).await)
);
TestBuilder!(
    anime_unapproved,
    (async |client: &TenraiAPI, _| client.anime_unapproved(BasicAnimeQuery::default()).await)
);
TestBuilder!(
    anime_denied,
    (async |client: &TenraiAPI, _| client.anime_denied(BasicAnimeQuery::default()).await)
);
TestBuilder!(
    anime_deleted,
    (async |client: &TenraiAPI, _| client.anime_deleted(BasicAnimeQuery::default()).await)
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
                .include([AnimeIDsQueryInclude::Title, AnimeIDsQueryInclude::AnimeType])
                .build()
                .unwrap()
        )
        .await),
    ignore = "Requires Server key"
);

// #[test]
// fn local_parsing() {
//     serde_json::from_str::<tenrai::schemas::TenraiResponse<tenrai::schemas::anime::Anime>>(
//         include_str!("/home/dragmine/.cache/tenrai/12006696859294398967.txt"),
//     )
//     .expect("Failed to parse json");
// }
