use crate::{
    TenariAPI,
    schemas::{TenraiResponse, anime::Anime},
};

impl TenariAPI {
    pub fn anime_search(&self) {}
    pub async fn anime_details(&self, id: u64) -> anyhow::Result<TenraiResponse<Anime>> {
        self.request(format!("anime/{id}")).await
    }
}
