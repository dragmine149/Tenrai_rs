use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{SfwQuery, StackQuery},
};

TestPaginator!(
    stacks,
    (async |client: &TenraiAPI, page| {
        client
            .stacks(
                StackQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    stacks_details,
    (async |client: &TenraiAPI, id| { client.stacks_details(id, SfwQuery::default()).await })
);
