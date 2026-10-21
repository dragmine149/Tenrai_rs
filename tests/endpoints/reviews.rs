use crate::{TestPaginator, build_client};
use tenrai::{TenraiAPI, schemas::query::TopReviewQuery};

TestPaginator!(
    review_anime_recent,
    (async |client: &TenraiAPI, page| {
        client
            .reviews_anime_recent(
                TopReviewQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "Paginator"
);
TestPaginator!(
    review_manga_recent,
    (async |client: &TenraiAPI, page| {
        client
            .reviews_manga_recent(
                TopReviewQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "Paginator"
);
