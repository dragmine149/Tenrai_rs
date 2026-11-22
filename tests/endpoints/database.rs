use tenrai::{
    TenraiAPI,
    schemas::{
        common::IDsQueryInclude,
        query::{AnimeIdsQuery, MangaIdsQuery},
    },
};

use crate::{TestBuilder, build_client};

TestBuilder!(
    anime_ids,
    (async |client: &TenraiAPI, _| client.anime_ids(AnimeIdsQuery::default()).await),
    ignore = "Requires Server key"
);
TestBuilder!(
    anime_ids_information,
    (async |client: &TenraiAPI, _| client
        .anime_ids(
            AnimeIdsQuery::builder()
                .include([IDsQueryInclude::Title, IDsQueryInclude::Type])
                .build()
                .unwrap()
        )
        .await),
    ignore = "Requires Server key"
);

TestBuilder!(
    character_ids,
    (async |client: &TenraiAPI, _| client.character_ids().await),
    ignore = "Requires Server key"
);

TestBuilder!(
    company_ids,
    (async |client: &TenraiAPI, _| { client.company_ids().await }),
    ignore = "Requires Server key"
);

TestBuilder!(
    manga_ids,
    (async |client: &TenraiAPI, _| { client.manga_ids(MangaIdsQuery::default()).await }),
    ignore = "Requires Server key"
);
TestBuilder!(
    manga_ids_information,
    (async |client: &TenraiAPI, _| {
        client
            .manga_ids(
                MangaIdsQuery::builder()
                    .include([IDsQueryInclude::Title, IDsQueryInclude::Type])
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "Requires Server key"
);

TestBuilder!(
    people_ids,
    (async |client: &TenraiAPI, _| { client.people_ids().await }),
    ignore = "Requires Server key"
);
