use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, manga::Manga, query::MangaQuery},
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
}
