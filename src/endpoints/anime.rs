use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        anime::{Anime, AnimeQuery, AnimeQueryBuilder},
    },
};

impl TenraiAPI {
    pub async fn anime_search(
        &self,
        query: AnimeQuery,
    ) -> anyhow::Result<TenraiPaginatorResponse<Anime>> {
        self.request(format!("anime{query}")).await
    }
    pub async fn anime_details(&self, id: u64) -> anyhow::Result<TenraiResponse<Anime>> {
        self.request(format!("anime/{id}")).await
    }
    pub async fn anime_full_details(&self, id: u64) -> anyhow::Result<TenraiResponse<Anime>> {
        self.request(format!("anime/{id}/full")).await
    }
}
