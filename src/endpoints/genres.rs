use crate::{
    TenraiAPI,
    schemas::{TenraiResponse, filter::BasicInfo, query::GenreQuery},
};

/// API Functions related to Genres
impl TenraiAPI {
    /// List all anime genres, themes and demographics
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/genres/GET/genres/anime>
    pub async fn genres_anime(
        &self,
        query: GenreQuery,
    ) -> crate::Result<TenraiResponse<Vec<BasicInfo>>> {
        Ok(self
            .request(format!("genres/anime{query}"))
            .await?
            .result()?)
    }
    /// List all manga genres, themes and demographics
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/genres/GET/genres/manga>
    pub async fn genres_manga(
        &self,
        query: GenreQuery,
    ) -> crate::Result<TenraiResponse<Vec<BasicInfo>>> {
        Ok(self
            .request(format!("genres/manga{query}"))
            .await?
            .result()?)
    }
}
