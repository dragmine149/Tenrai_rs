use crate::{TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::{
        common::Date,
        query::{
            PageQueryHelper, TopAnimeQuery, TopCharacterQuery, TopEpisodeQuery, TopMangaQuery,
            TopPeopleQuery, TopRecommendationsQuery, TopReviewQuery, TopVideoQuery,
        },
    },
};
TestPaginator!(
    top_anime,
    (async |client: &TenraiAPI, page| {
        client
            .top_anime(
                TopAnimeQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

TestPaginator!(
    top_manga,
    (async |client: &TenraiAPI, page| {
        client
            .top_manga(
                TopMangaQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

TestPaginator!(
    top_people,
    (async |client: &TenraiAPI, page| {
        client
            .top_people(
                TopPeopleQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

TestPaginator!(
    top_character,
    (async |client: &TenraiAPI, page| {
        client
            .top_characters(
                TopCharacterQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

TestPaginator!(
    top_reviews,
    (async |client: &TenraiAPI, page| {
        client
            .top_reviews(
                TopReviewQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

TestPaginator!(
    top_recommendations,
    (async |client: &TenraiAPI, page| {
        client
            .top_recommendations(
                TopRecommendationsQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

TestPaginator!(
    top_episodes,
    (async |client: &TenraiAPI, page| {
        client
            .top_episodes(
                TopEpisodeQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);

#[test]
fn test_top_episode_regex() {
    TopEpisodeQuery::builder()
        .aired_after(Date::new(2026, 06, None).to_string())
        .build()
        .expect_err("This should fail");
    TopEpisodeQuery::builder()
        .aired_after(Date::new(2026, 06, None).to_full_string())
        .build()
        .expect("This should not fail");
}

TestPaginator!(
    top_videos,
    (async |client: &TenraiAPI, page| {
        client
            .top_videos(
                TopVideoQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
