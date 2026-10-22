use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    errors::{MachineErrorType, TenraiError},
    schemas::query::{ArticleQuery, PeopleQuery, SfwQuery},
};

TestPaginator!(
    people_search,
    (async |client: &TenraiAPI, page| {
        client
            .people_search(
                PeopleQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    people_full_details,
    (async |client: &TenraiAPI, id| { client.people_full_details(id, SfwQuery::default()).await })
);
TestBuilder!(
    people_anime_roles,
    (async |client: &TenraiAPI, id| { client.people_anime_roles(id, SfwQuery::default()).await })
);
TestBuilder!(
    people_manga_roles,
    (async |client: &TenraiAPI, id| { client.people_manga_roles(id, SfwQuery::default()).await })
);
TestBuilder!(
    people_voices,
    (async |client: &TenraiAPI, id| { client.people_voices(id, SfwQuery::default()).await })
);
TestBuilder!(
    people_pictures,
    (async |client: &TenraiAPI, id| { client.people_pictures(id).await })
);
TestBuilder!(
    people_articles,
    (async |client: &TenraiAPI, id| { client.people_articles(id, ArticleQuery::default()).await })
);
TestBuilder!(
    people_ids,
    (async |client: &TenraiAPI, _| { client.people_ids().await }),
    ignore = "This requires a server key"
);
