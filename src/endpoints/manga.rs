use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        manga::Manga,
        query::{MangaQuery, SfwQuery},
    },
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
}
