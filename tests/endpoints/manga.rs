use crate::{TestBuilder, TestPaginator, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::{
        AdaptedQuery, ArticleQuery, BasicQuery, ExtendedDetailsQuery, ForumQuery, MagazineQuery,
        MangaQuery, NewsArticleQuery, PageQueryHelper, RecommendationQuery, ReviewQuery, SfwQuery,
        StackQuery, StatsHistoryQuery,
    },
};

TestPaginator!(
    manga_search,
    (async |client: &TenraiAPI, page| {
        client
            .manga_search(
                MangaQuery::builder()
                    .page(page)
                    .limit(100_u32)
                    .build()
                    .unwrap(),
            )
            .await
    })
);
TestBuilder!(
    manga_full_details,
    (async |client: &TenraiAPI, id| {
        client
            .manga_full_details(id, ExtendedDetailsQuery::default())
            .await
    })
);
TestBuilder!(
    manga_recommendation,
    (async |client: &TenraiAPI, id| {
        client
            .manga_recommendations(id, RecommendationQuery::default())
            .await
    })
);
TestBuilder!(
    manga_characters,
    (async |client: &TenraiAPI, id| { client.manga_characters(id).await })
);
TestBuilder!(
    manga_news,
    (async |client: &TenraiAPI, id| { client.manga_news(id, NewsArticleQuery::default()).await })
);
TestBuilder!(
    manga_articles,
    (async |client: &TenraiAPI, id| { client.manga_articles(id, ArticleQuery::default()).await })
);
TestBuilder!(
    manga_pictures,
    (async |client: &TenraiAPI, id| { client.manga_pictures(id).await })
);
TestBuilder!(
    manga_statistics,
    (async |client: &TenraiAPI, id| { client.manga_statistics(id).await })
);
TestBuilder!(
    manga_statistics_historical,
    (async |client: &TenraiAPI, id| {
        client
            .manga_statistics_historyical(id, StatsHistoryQuery::default())
            .await
    })
);
TestBuilder!(
    manga_more_info,
    (async |client: &TenraiAPI, id| { client.manga_more_info(id).await })
);
TestBuilder!(
    manga_relations,
    (async |client: &TenraiAPI, id| { client.manga_relations(id, SfwQuery::default()).await })
);
TestBuilder!(
    manga_external,
    (async |client: &TenraiAPI, id| { client.manga_external(id).await })
);
TestBuilder!(
    manga_reviews,
    (async |client: &TenraiAPI, id| { client.manga_reviews(id, ReviewQuery::default()).await })
);
TestBuilder!(
    manga_forum,
    (async |client: &TenraiAPI, id| { client.anime_forum(id, ForumQuery::default()).await })
);
TestBuilder!(
    manga_stacks,
    (async |client: &TenraiAPI, id| { client.manga_stacks(id, StackQuery::default()).await })
);
TestBuilder!(
    manga_unapproved,
    (async |client: &TenraiAPI, _| { client.manga_unapproved(BasicQuery::default()).await })
);
TestBuilder!(
    manga_denied,
    (async |client: &TenraiAPI, _| { client.manga_denied(BasicQuery::default()).await })
);
TestBuilder!(
    manga_adapted,
    (async |client: &TenraiAPI, _| { client.manga_adapted(AdaptedQuery::default()).await })
);
TestBuilder!(
    manga_deleted,
    (async |client: &TenraiAPI, _| { client.manga_deleted(BasicQuery::default()).await })
);
TestBuilder!(
    magazines,
    (async |client: &TenraiAPI, _| { client.magazines(MagazineQuery::default()).await })
);
