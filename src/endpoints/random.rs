use crate::{
    TenraiAPI,
    schemas::{
        TenraiResponse,
        anime::Anime,
        characters::CharacterDetails,
        manga::Manga,
        people::Person,
        query::{RandomAnimeQuery, RandomMangaQuery, RandomPersonQuery, SfwQuery},
    },
};

/// API Functions related to getting random data
impl TenraiAPI {
    /// Get a random anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/random/GET/random/anime>
    pub async fn random_anime(
        &self,
        query: RandomAnimeQuery,
    ) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request(format!("random/anime{query}"))
            .await?
            .result()?)
    }
    /// Get a random manga
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/random/GET/random/manga>
    pub async fn random_manga(
        &self,
        query: RandomMangaQuery,
    ) -> crate::Result<TenraiResponse<Manga>> {
        Ok(self
            .request(format!("random/manga{query}"))
            .await?
            .result()?)
    }
    /// Get a random character
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/random/GET/random/characters>
    pub async fn random_character(
        &self,
        query: RandomPersonQuery,
    ) -> crate::Result<TenraiResponse<CharacterDetails>> {
        Ok(self
            .request(format!("random/characters{query}"))
            .await?
            .result()?)
    }
    /// Get a random person
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/random/GET/random/people>
    pub async fn random_person(
        &self,
        query: RandomPersonQuery,
    ) -> crate::Result<TenraiResponse<Person>> {
        Ok(self
            .request(format!("random/people{query}"))
            .await?
            .result()?)
    }
}
