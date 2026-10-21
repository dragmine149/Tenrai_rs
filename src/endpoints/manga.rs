use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, TenraiResponse, manga::Manga, query::MangaQuery},
};

impl TenraiAPI {
    pub async fn manga_search(
        &self,
        query: MangaQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("manga{query}"))
            .await?
            .result_paginator()?)
    }
    pub async fn manga_details(&self, id: u64) -> crate::Result<TenraiResponse<Manga>> {
        Ok(self.request(format!("manga/{id}")).await?.result()?)
    }
}
