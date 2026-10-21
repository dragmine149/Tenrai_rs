use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, common::FullRecommendationInfo, query::FullPageSfwQuery},
};

impl TenraiAPI {
    /// Get the latest anime recommendations from users
    pub async fn recommendations_anime_recent(
        &self,
        query: FullPageSfwQuery,
    ) -> crate::Result<TenraiPaginatorResponse<FullRecommendationInfo>> {
        Ok(self
            .request(format!("recommendations/anime{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the latest manga recommendations from users
    pub async fn recommendations_manga_recent(
        &self,
        query: FullPageSfwQuery,
    ) -> crate::Result<TenraiPaginatorResponse<FullRecommendationInfo>> {
        Ok(self
            .request(format!("recommendations/manga{query}"))
            .await?
            .result_paginator()?)
    }
}
