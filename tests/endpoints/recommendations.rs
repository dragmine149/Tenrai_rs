use crate::{TestPaginator, build_client};
use tenrai::{TenraiAPI, schemas::query::FullPageSfwQuery};

TestPaginator!(
    recommendations_anime_recent,
    (async |client: &TenraiAPI, page| {
        client
            .recommendations_anime_recent(
                FullPageSfwQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestPaginator!(
    recommendations_manga_recent,
    (async |client: &TenraiAPI, page| {
        client
            .recommendations_manga_recent(
                FullPageSfwQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
