use crate::{TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{PageQuery, TopQuery, TopReviewQuery},
};
TestPaginator!(
    top_anime,
    (async |client: &TenraiAPI, page| {
        client
            .top_anime(
                TopQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "This takes 1+ mins to complete (public tier)"
);

TestPaginator!(
    top_manga,
    (async |client: &TenraiAPI, page| {
        client
            .top_manga(
                TopQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "This takes 1+ mins to complete (public tier)"
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
    }),
    ignore = "This takes 1+ mins to complete (public tier)"
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
    }),
    ignore = "This takes 1+ mins to complete (public tier)"
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
    }),
    ignore = "This takes 1+ mins to complete (public tier)"
);
