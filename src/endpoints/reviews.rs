use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, query::TopReviewQuery, reviews::Review},
};

/// API Functions related to Reviews
impl TenraiAPI {
    /// Get the latest anime reviews.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/reviews/GET/reviews/anime>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/reviews/GET/reviews/manga>
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
