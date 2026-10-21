use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse,
        anime::Anime,
        common::{Season, SeasonLookup},
        query::SeasonalQuery,
    },
};

impl TenraiAPI {
    /// Get the anime of a given season
    pub async fn seasonal_anime(
        &self,
        year: u64,
        season: Season,
        query: SeasonalQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        if year <= 1000 || year >= 2999 {
            return Err(crate::errors::TenraiError::Custom(
                "Year must be between 1000 and 2999 (inclusive)",
            ));
        }
        Ok(self
            .request(format!("seasons/{year}/{season}{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the current season's anime
    pub async fn seasonal_anime_current(
        &self,
        query: SeasonalQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("seasons/now{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the previous season's anime
    pub async fn seasonal_anime_previous(
        &self,
        query: SeasonalQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("seasons/previous{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get announced anime that hasn't aired
    pub async fn seasonal_anime_upcoming(
        &self,
        query: SeasonalQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("seasons/upcoming{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get every season that can be looked up
    pub async fn seasonal_all(&self) -> crate::Result<TenraiPaginatorResponse<SeasonLookup>> {
        Ok(self.request(format!("seasons")).await?.result_paginator()?)
    }
}
