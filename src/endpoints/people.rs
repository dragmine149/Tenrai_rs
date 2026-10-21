use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        article::Article,
        common::Images,
        people::{AnimePersonConnection, MangaPersonConnection, Person, VoiceConnection},
        query::{ArticleQuery, PeopleQuery, SfwQuery},
    },
};

impl TenraiAPI {
    /// Search people, such as VAs, Staff and Authors
    pub async fn people_search(
        &self,
        query: PeopleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Person>> {
        Ok(self
            .request(format!("people{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for a person
    pub async fn people_details(&self, id: u64) -> crate::Result<TenraiResponse<Person>> {
        Ok(self.request(format!("people/{id}")).await?.result()?)
    }

    /// Get Full details for a person
    pub async fn people_full_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Person>> {
        Ok(self
            .request(format!("people/{id}/full{query}"))
            .await?
            .result()?)
    }
    /// Get the anime a person worked on
    pub async fn people_anime_roles(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<AnimePersonConnection>>> {
        Ok(self
            .request(format!("people/{id}/anime{query}"))
            .await?
            .result()?)
    }
    /// Get the manga a person worked on
    pub async fn people_manga_roles(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<MangaPersonConnection>>> {
        Ok(self
            .request(format!("people/{id}/manga{query}"))
            .await?
            .result()?)
    }
    /// Get a person voice acting roles.
    pub async fn people_voices(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<VoiceConnection>>> {
        Ok(self
            .request(format!("people/{id}/voices{query}"))
            .await?
            .result()?)
    }
    /// Get pictures of a person
    pub async fn people_pictures(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Images>>> {
        Ok(self
            .request(format!("people/{id}/pictures"))
            .await?
            .result()?)
    }
    /// Get featured articles about a person
    pub async fn people_articles(
        &self,
        id: u64,
        query: ArticleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("people/{id}/articles{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the ID of every person in one request
    ///
    /// NOTE: This requires a server key.
    /// TODO: Implement client-sided server-key checker
    pub async fn people_ids(&self) -> crate::Result<TenraiResponse<Vec<u64>>> {
        Ok(self.request(format!("people/ids")).await?.result()?)
    }
}
