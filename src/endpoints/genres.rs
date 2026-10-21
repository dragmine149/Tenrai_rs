use crate::{
    TenraiAPI,
    schemas::{TenraiResponse, filter::GenreInfo, query::GenreQuery},
};

impl TenraiAPI {
    /// List all anime genres, themes and demographics
    pub async fn genres_anime(
        &self,
        query: GenreQuery,
    ) -> crate::Result<TenraiResponse<Vec<GenreInfo>>> {
        Ok(self
            .request(format!("genres/anime{query}"))
            .await?
            .result()?)
    }
    /// List all manga genres, themes and demographics
    pub async fn genres_manga(
        &self,
        query: GenreQuery,
    ) -> crate::Result<TenraiResponse<Vec<GenreInfo>>> {
        Ok(self
            .request(format!("genres/manga{query}"))
            .await?
            .result()?)
    }
}
