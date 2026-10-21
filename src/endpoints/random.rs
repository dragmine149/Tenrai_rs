use crate::{
    TenraiAPI,
    schemas::{
        TenraiResponse, anime::Anime, characters::CharacterDetails, manga::Manga, people::Person,
        query::SfwQuery,
    },
};

impl TenraiAPI {
    /// Get a random anime
    pub async fn random_anime(&self, query: SfwQuery) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request(format!("random/anime{query}"))
            .await?
            .result()?)
    }
    /// Get a random manga
    pub async fn random_manga(&self, query: SfwQuery) -> crate::Result<TenraiResponse<Manga>> {
        Ok(self
            .request(format!("random/manga{query}"))
            .await?
            .result()?)
    }
    /// Get a random character
    pub async fn random_character(
        &self,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<CharacterDetails>> {
        Ok(self
            .request(format!("random/characters{query}"))
            .await?
            .result()?)
    }
    /// Get a random person
    pub async fn random_person(&self, query: SfwQuery) -> crate::Result<TenraiResponse<Person>> {
        Ok(self
            .request(format!("random/people{query}"))
            .await?
            .result()?)
    }
}
