use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiRaw, TenraiRawPaginator, TenraiResponse,
        anime::{Anime, AnimeQuery},
    },
};

impl TenraiAPI {
    pub async fn anime_search(
        &self,
        query: AnimeQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request::<TenraiRawPaginator<Anime>>(format!("anime{query}"))
            .await?
            .result()?)
    }
    pub async fn anime_details(&self, id: u64) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request::<TenraiRaw<Anime>>(format!("anime/{id}"))
            .await?
            .result()?)
    }
    pub async fn anime_full_details(&self, id: u64) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request::<TenraiRaw<Anime>>(format!("anime/{id}/full"))
            .await?
            .result()?)
    }
}
