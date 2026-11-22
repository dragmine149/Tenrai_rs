use crate::{
    TenraiAPI,
    schemas::{
        TenraiResponse, TenraiTotalResponse,
        common::IDList,
        query::{AnimeIdsQuery, MangaIdsQuery},
    },
};

/// API functions related to database queries: <https://api.tenrai.org/documentation#tag/database>
impl TenraiAPI {
    /// Retrieve all unique MAL anime IDs of approved entries that currently exist and are active.
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/database/GET/anime/ids>
    #[doc(alias = "db_anime_ids")]
    pub async fn anime_ids(
        &self,
        query: AnimeIdsQuery,
    ) -> crate::Result<TenraiTotalResponse<IDList>> {
        self.check_server_key()?;
        Ok(self
            .request(format!("anime/ids{query}"))
            .await?
            .result_total()?)
    }
    /// Retrieve all unique MAL manga IDs of approved entries that currently exist and are active.
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/database/GET/manga/ids>
    #[doc(alias = "db_manga_ids")]
    pub async fn manga_ids(
        &self,
        query: MangaIdsQuery,
    ) -> crate::Result<TenraiTotalResponse<IDList>> {
        self.check_server_key()?;
        Ok(self
            .request(format!("manga/ids{query}"))
            .await?
            .result_total()?)
    }
    /// Get the ID of every character in one request
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <api.tenrai.org/documentation#tag/database/GET/characters/ids>
    #[doc(alias = "db_character_ids")]
    pub async fn character_ids(&self) -> crate::Result<TenraiResponse<Vec<u32>>> {
        self.check_server_key()?;
        Ok(self.request("characters/ids".to_string()).await?.result()?)
    }
    /// Get the ID of every person in one request
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/database/GET/people/ids>
    #[doc(alias = "db_people_ids")]
    #[doc(alias = "person_ids")]
    #[doc(alias = "db_person_ids")]
    pub async fn people_ids(&self) -> crate::Result<TenraiResponse<Vec<u32>>> {
        self.check_server_key()?;
        Ok(self.request("people/ids".to_string()).await?.result()?)
    }
    /// Get the ID of every company in one request
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/database/GET/producers/ids>
    #[doc(alias = "producers_ids")]
    #[doc(alias = "db_producers_ids")]
    #[doc(alias = "db_company_ids")]
    pub async fn company_ids(&self) -> crate::Result<TenraiResponse<Vec<u32>>> {
        self.check_server_key()?;
        Ok(self.request("producers/ids".to_string()).await?.result()?)
    }
}
