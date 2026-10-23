use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse, TenraiTotalResponse,
        article::Article,
        common::{
            External, Forum, IDList, Images, MoreInfo, Recommendation, Relation, StatsHistory,
        },
        filter::BasicInfo,
        manga::{Manga, MangaStatistics},
        people::Character,
        query::{
            AdaptedQuery, ArticleQuery, BasicQuery, ForumQuery, MagazineQuery, MangaIdsQuery,
            MangaQuery, PageSfwQuery, ReviewQuery, SfwQuery, StackQuery, StatsHistoryQuery,
        },
        reviews::Review,
        stacks::Stack,
    },
};

/// API Functions related to Manga
impl TenraiAPI {
    /// Search and filter manga
    pub async fn manga_search(
        &self,
        query: MangaQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("manga{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for a manga
    pub async fn manga_details(&self, id: u64) -> crate::Result<TenraiResponse<Manga>> {
        Ok(self.request(format!("manga/{id}")).await?.result()?)
    }
    /// Get full detailed data for a manga.
    pub async fn manga_full_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Manga>> {
        Ok(self
            .request(format!("manga/{id}/full{query}"))
            .await?
            .result()?)
    }
    /// Get recommendations for a manga
    pub async fn manga_recommendations(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<Recommendation>>> {
        Ok(self
            .request(format!("manga/{id}/recommendations{query}"))
            .await?
            .result()?)
    }
    /// Get manga characters
    pub async fn manga_characters(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Character>>> {
        Ok(self
            .request(format!("manga/{id}/characters"))
            .await?
            .result()?)
    }
    /// Get news about a manga
    pub async fn manga_news(
        &self,
        id: u64,
        query: PageSfwQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("manga/{id}/news{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get featured articles about a manga
    pub async fn manga_articles(
        &self,
        id: u64,
        query: ArticleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("manga/{id}/articles{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get pictures for a manga
    pub async fn manga_pictures(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Images>>> {
        Ok(self
            .request(format!("manga/{id}/pictures"))
            .await?
            .result()?)
    }
    /// Get statistics for a manga
    pub async fn manga_statistics(
        &self,
        id: u64,
    ) -> crate::Result<TenraiResponse<MangaStatistics>> {
        Ok(self
            .request(format!("manga/{id}/statistics"))
            .await?
            .result()?)
    }
    /// Get historical statistics for a manga
    ///
    /// See https://api.tenrai.org/documentation#tag/manga/GET/manga/{id}/statistics/history for more information
    pub async fn manga_statistics_historyical(
        &self,
        id: u64,
        query: StatsHistoryQuery,
    ) -> crate::Result<TenraiPaginatorResponse<StatsHistory<MangaStatistics>>> {
        Ok(self
            .request(format!("manga/{id}/statistics/history{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get more information about a manga
    pub async fn manga_more_info(&self, id: u64) -> crate::Result<TenraiResponse<MoreInfo>> {
        Ok(self
            .request(format!("manga/{id}/moreinfo"))
            .await?
            .result()?)
    }
    /// Get relations for a manga
    pub async fn manga_relations(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<Relation>>> {
        Ok(self
            .request(format!("manga/{id}/relations{query}"))
            .await?
            .result()?)
    }
    /// Get links to other sites about a manga
    pub async fn manga_external(&self, id: u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("manga/{id}/external"))
            .await?
            .result()?)
    }
    /// Get ,user reviews of a manga.
    pub async fn manga_reviews(
        &self,
        id: u64,
        query: ReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("manga/{id}/reviews/{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get forum topics related to the manga entry. Episode discussion threads only
    ///
    /// NOTE: You can use the official MAL API to fetch the threads themselves by using the thread id.
    pub async fn manga_forum(
        &self,
        id: u64,
        query: ForumQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Forum>> {
        Ok(self
            .request(format!("manga/{id}/forum{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get all public interest stacks for an manga
    pub async fn manga_stacks(
        &self,
        id: u64,
        query: StackQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Stack>> {
        Ok(self
            .request(format!("manga/{id}/stacks{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get manga waiting for MAL approval
    pub async fn manga_unapproved(
        &self,
        query: BasicQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("manga/unapproved{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get manga MAL denied
    pub async fn manga_denied(
        &self,
        query: BasicQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("manga/denied{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get manga that have been adapted into anime
    pub async fn manga_adapted(
        &self,
        query: AdaptedQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("manga/adapted{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get manga that has been removed from MAL
    pub async fn manga_deleted(
        &self,
        query: BasicQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("manga/deleted{query}"))
            .await?
            .result_paginator()?)
    }
    /// Retrieve all unique MAL manga IDs of approved entries that currently exist and are active.
    ///
    /// NOTE: This requires a server key.
    pub async fn manga_ids(
        &self,
        query: MangaIdsQuery,
    ) -> crate::Result<TenraiTotalResponse<IDList>> {
        self.check_server_key()?;
        Ok(self
            .request(format!("manga/ids{query}"))
            .await?
            .result_total()?)
    }
}

// This is wayy too small to be on it's own. And it fits nicely in with manga (as well yeah).
// If this expands, then we break it out.
/// API Functions related to Magazines
impl TenraiAPI {
    /// Search manga magazines
    pub async fn magazines(
        &self,
        query: MagazineQuery,
    ) -> crate::Result<TenraiResponse<Vec<BasicInfo>>> {
        Ok(self.request(format!("magazines{query}")).await?.result()?)
    }
}
