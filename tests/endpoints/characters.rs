use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{ArticleQuery, CharacterQuery, SfwQuery},
};

TestPaginator!(
    character_search,
    (async |client: &TenraiAPI, page| {
        client
            .character_search(
                CharacterQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    character_full_details,
    (async |client: &TenraiAPI, id| { client.character_details(id).await })
);
TestBuilder!(
    character_anime_appearance,
    (async |client: &TenraiAPI, id| {
        client
            .character_anime_appearance(id, SfwQuery::default())
            .await
    })
);
TestBuilder!(
    character_manga_appearance,
    (async |client: &TenraiAPI, id| {
        client
            .character_manga_appearance(id, SfwQuery::default())
            .await
    })
);
TestBuilder!(
    character_voice_actors,
    (async |client: &TenraiAPI, id| { client.character_voice_actors(id).await })
);
TestBuilder!(
    character_pictures,
    (async |client: &TenraiAPI, id| { client.character_pictures(id).await })
);
TestBuilder!(
    character_articles,
    (async |client: &TenraiAPI, id| {
        client.character_articles(id, ArticleQuery::default()).await
    })
);
TestBuilder!(
    character_ids,
    (async |client: &TenraiAPI, _| client.character_ids().await),
    ignore = "Requires Server key"
);
