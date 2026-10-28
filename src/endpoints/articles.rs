use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        article::Article,
        common::Tag,
        query::{FullArticleQuery, NewsQuery, SfwQuery},
    },
};

/// API Functions related to news articles
impl TenraiAPI {
    /// Get news articles
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/news/GET/news>
    pub async fn news(&self, query: NewsQuery) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("news{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get all news article tags
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/news/GET/news/tags>
    pub async fn news_tags(&self) -> crate::Result<TenraiResponse<Vec<Tag>>> {
        Ok(self.request("news/tags".to_string()).await?.result()?)
    }
    /// Get an news article by its id
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/news/GET/news/{id}>
    pub async fn news_details(
        &self,
        id: &u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Article>> {
        Ok(self.request(format!("news/{id}{query}")).await?.result()?)
    }
}

/// API Functions related to articles in general
impl TenraiAPI {
    /// Get featured articles
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/articles/GET/articles>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/articles/GET/articles/tags>
    pub async fn article_tags(&self) -> crate::Result<TenraiResponse<Vec<Tag>>> {
        Ok(self.request("articles/tags".to_string()).await?.result()?)
    }
    /// Get an article by its id
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/articles/GET/articles/{id}>
    pub async fn article_details(
        &self,
        id: &u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Article>> {
        Ok(self
            .request(format!("articles/{id}{query}"))
            .await?
            .result()?)
    }
}
