use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        article::Article,
        common::Tag,
        query::{FullArticleQuery, NewsQuery, SfwQuery},
    },
};

impl TenraiAPI {
    /// Get news articles
    pub async fn news(&self, query: NewsQuery) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("news{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get all news article tags
    pub async fn news_tags(&self) -> crate::Result<TenraiResponse<Vec<Tag>>> {
        Ok(self.request(format!("news/tags")).await?.result()?)
    }
    /// Get an news article by its id
    pub async fn news_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Article>> {
        Ok(self.request(format!("news/{id}{query}")).await?.result()?)
    }
}

impl TenraiAPI {
    /// Get featured articles
    pub async fn articles(
        &self,
        query: FullArticleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("articles{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get all article tags
    pub async fn article_tags(&self) -> crate::Result<TenraiResponse<Tag>> {
        Ok(self.request(format!("articles/tags")).await?.result()?)
    }
    /// Get an article by its id
    pub async fn article_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Article>> {
        Ok(self
            .request(format!("articles/{id}{query}"))
            .await?
            .result()?)
    }
}
