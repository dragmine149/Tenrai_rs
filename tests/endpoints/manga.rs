use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    errors::{MachineErrorType, TenraiError},
    schemas::query::MangaQuery,
};

TestPaginator!(
    manga_search,
    (async |client: &TenraiAPI, page| {
        client
            .manga_search(
                MangaQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "This takes 1 minutes to complete on it's own (public tier)"
);
TestBuilder!(
    manga_details,
    (async |client: &TenraiAPI, id| { client.manga_details(id).await })
);
