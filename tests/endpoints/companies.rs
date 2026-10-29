use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::ProducerQuery,
};
TestPaginator!(
    company_search,
    (async |client: &TenraiAPI, page| {
        client
            .company_search(
                ProducerQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    company_full_details,
    (async |client: &TenraiAPI, id| { client.company_full_details(id).await })
);
TestBuilder!(
    company_external,
    (async |client: &TenraiAPI, id| { client.company_external(id).await })
);
TestBuilder!(
    company_ids,
    (async |client: &TenraiAPI, _| { client.company_ids().await }),
    ignore = "Requires Server key"
);
