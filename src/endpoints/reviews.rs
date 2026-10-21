use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, query::TopReviewQuery, reviews::Review},
};

impl TenraiAPI {
    /// Get the latest anime reviews.
    pub async fn reviews_anime_recent(
        &self,
        query: TopReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("reviews/anime{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the latest manga reviews.
    pub async fn reviews_manga_recent(
        &self,
        query: TopReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("reviews/manga{query}"))
            .await?
            .result_paginator()?)
    }
}
