use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        article::Article,
        characters::{AnimeCharacterConnection, CharacterDetails, MangaCharacterConnection},
        common::Images,
        people::CharacterVA,
        query::{ArticleQuery, CharacterQuery, SfwQuery},
    },
};

impl TenraiAPI {
    /// Search for characters
    pub async fn character_search(
        &self,
        query: CharacterQuery,
    ) -> crate::Result<TenraiPaginatorResponse<CharacterDetails>> {
        Ok(self
            .request(format!("characters{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for a character
    pub async fn character_details(
        &self,
        id: u64,
    ) -> crate::Result<TenraiResponse<CharacterDetails>> {
        Ok(self.request(format!("characters/{id}")).await?.result()?)
    }
    /// Get full details for a character
    pub async fn character_full_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<CharacterDetails>> {
        Ok(self
            .request(format!("characters/{id}/full{query}"))
            .await?
            .result()?)
    }
    /// Get the anime the character appears in
    pub async fn character_anime_appearance(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<AnimeCharacterConnection>>> {
        Ok(self
            .request(format!("characters/{id}/anime{query}"))
            .await?
            .result()?)
    }
    /// Get the manga a character appears in
    pub async fn character_manga_appearance(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<MangaCharacterConnection>>> {
        Ok(self
            .request(format!("characters/{id}/manga{query}"))
            .await?
            .result()?)
    }
    /// Get a character voice actors
    pub async fn character_voice_actors(
        &self,
        id: u64,
    ) -> crate::Result<TenraiResponse<Vec<CharacterVA>>> {
        Ok(self
            .request(format!("characters/{id}/voices"))
            .await?
            .result()?)
    }
    /// Get the pictures for a character
    pub async fn character_pictures(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Images>>> {
        Ok(self
            .request(format!("characters/{id}/pictures"))
            .await?
            .result()?)
    }
    /// Get featured articles about a character
    pub async fn character_articles(
        &self,
        id: u64,
        query: ArticleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("characters/{id}/articles{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the ID of every character in one request
    ///
    /// NOTE: This requires a server key.
    /// TODO: Implement client-sided server-key checker
    pub async fn character_ids(&self) -> crate::Result<TenraiResponse<Vec<u64>>> {
        Ok(self.request(format!("characters/ids")).await?.result()?)
    }
}
