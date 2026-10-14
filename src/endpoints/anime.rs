use crate::{
    TenraiAPI,
    schemas::{
        SfwQuery, TenraiPaginatorResponse, TenraiResponse,
        anime::{Anime, AnimeQuery},
    },
};

impl TenraiAPI {
    /// Search for an anime with filters.
    pub async fn anime_search(
        &self,
        query: AnimeQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("anime{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for an anime
    pub async fn anime_details(&self, id: u64) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self.request(format!("anime/{id}")).await?.result()?)
    }
    /// Get full detailed data for an anime
    ///
    /// NOTE: [SfwQuery] will only filter out external relationsions. Not the entire entry itself.
    pub async fn anime_full_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request(format!("anime/{id}/full{query}"))
            .await?
            .result()?)
    }
}
