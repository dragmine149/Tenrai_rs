use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{FullArticleQuery, NewsQuery, SfwQuery},
};

TestPaginator!(
    news,
    (async |client: &TenraiAPI, page| {
        client
            .news(
                NewsQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestPaginator!(
    articles,
    (async |client: &TenraiAPI, page| {
        client
            .articles(
                FullArticleQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    news_tags,
    (async |client: &TenraiAPI, _| { client.news_tags().await })
);
TestBuilder!(
    article_tags,
    (async |client: &TenraiAPI, _| { client.article_tags().await })
);

TestBuilder!(
    news_details,
    (async |client: &TenraiAPI, page: &u64| {
        let id = client
            .news(
                NewsQuery::builder()
                    .page((*page).clamp(1, 16000))
                    .limit(1_u64)
                    .build()
                    .unwrap(),
            )
            .await
            .expect("Failed to unwarp news list")
            .data
            .first()
            .expect("Page is empty?")
            .mal_id;
        println!("New ID: {id}");
        client.news_details(&id, SfwQuery::default()).await
    })
);
TestBuilder!(
    article_details,
    (async |client: &TenraiAPI, id| { client.article_details(id, SfwQuery::default()).await })
);
