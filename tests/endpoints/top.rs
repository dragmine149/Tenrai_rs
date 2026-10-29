use crate::{TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{
        PageQuery, TopAnimeQuery, TopEpisodeQuery, TopMangaQuery, TopReviewQuery, TypeSfwPageQuery,
    },
};
TestPaginator!(
    top_anime,
    (async |client: &TenraiAPI, page| {
        client
            .top_anime(
                TopAnimeQuery::builder()
                    .page(page)
                    .limit(100_u64)
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
                    .limit(100_u64)
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
                PageQuery::builder()
                    .page(page)
                    .limit(100_u64)
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
            .top_character(
                PageQuery::builder()
                    .page(page)
                    .limit(100_u64)
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
                    .limit(100_u64)
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
                    .limit(100_u64)
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
                TypeSfwPageQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
