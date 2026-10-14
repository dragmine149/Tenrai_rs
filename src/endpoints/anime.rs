use crate::{
    TenraiAPI,
    schemas::{
        BasicPageQuery, SfwQuery, TenraiPaginatorResponse, TenraiResponse,
        anime::{Anime, AnimeCharacter, AnimeEpisode, AnimeQuery, AnimeRecommendation, AnimeStaff},
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
    /// Get recommendations for an anime
    pub async fn anime_recommendations(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<AnimeRecommendation>> {
        Ok(self
            .request(format!("anime/{id}/recommendations{query}"))
            .await?
            .result()?)
    }
    /// Get character list and voice actors for an anime
    pub async fn anime_characters(
        &self,
        id: u64,
    ) -> crate::Result<TenraiResponse<Vec<AnimeCharacter>>> {
        Ok(self
            .request(format!("anime/{id}/characters"))
            .await?
            .result()?)
    }
    ///Get staff for an anime
    pub async fn anime_staff(&self, id: u64) -> crate::Result<TenraiResponse<Vec<AnimeStaff>>> {
        Ok(self.request(format!("anime/{id}/staff")).await?.result()?)
    }
    /// Get episodes for an anime
    pub async fn anime_episodes(
        &self,
        id: u64,
        query: BasicPageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<AnimeEpisode>> {
        Ok(self
            .request(format!("anime/{id}/episodes{query}"))
            .await?
            .result_paginator()?)
    }
    // /// Get a list of news articles related to the anime
    // pub async fn anime_news(&self, id: u64, query: BasicPageQuery, query2: SfwQuery) {}
}
