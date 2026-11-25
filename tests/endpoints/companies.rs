use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{CompanyQuery, PageQueryHelper},
};
TestPaginator!(
    company_search,
    (async |client: &TenraiAPI, page| {
        client
            .company_search(
                CompanyQuery::builder()
                    .page(page)
                    .limit(100_u32)
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
