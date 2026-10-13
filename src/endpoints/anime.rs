use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        anime::{Anime, AnimeQuery},
    },
};

impl TenraiAPI {
    pub async fn anime_search(
        &self,
        query: AnimeQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        self.request(format!("anime{query}")).await
    }
    pub async fn anime_details(&self, id: u64) -> crate::Result<TenraiResponse<Anime>> {
        self.request(format!("anime/{id}")).await
    }
    pub async fn anime_full_details(&self, id: u64) -> crate::Result<TenraiResponse<Anime>> {
        self.request(format!("anime/{id}/full")).await
    }
}
