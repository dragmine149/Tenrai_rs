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

/// API Functions related to People
impl TenraiAPI {
    /// Search people, such as VAs, Staff and Authors
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}>
    pub async fn people_details(&self, id: u64) -> crate::Result<TenraiResponse<Person>> {
        Ok(self.request(format!("people/{id}")).await?.result()?)
    }

    /// Get Full details for a person
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}/full>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}/anime>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}/manga>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}/voices>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}/pictures>
    pub async fn people_pictures(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Images>>> {
        Ok(self
            .request(format!("people/{id}/pictures"))
            .await?
            .result()?)
    }
    /// Get featured articles about a person
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/{id}/articles>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/people/GET/people/ids>
    pub async fn people_ids(&self) -> crate::Result<TenraiResponse<Vec<u64>>> {
        self.check_server_key()?;
        Ok(self.request("people/ids".to_string()).await?.result()?)
    }
}
