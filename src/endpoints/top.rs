use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse,
        anime::Anime,
        characters::CharacterDetails,
        manga::Manga,
        people::Person,
        query::{PageQuery, TopQuery, TopReviewQuery},
        reviews::Review,
    },
};

/// API Functions related to Ranking
impl TenraiAPI {
    /// Get the highest ranking anime
    pub async fn top_anime(
        &self,
        query: TopQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("top/anime{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the highest ranking Manga
    pub async fn top_manga(
        &self,
        query: TopQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("top/manga{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the most favorited people
    pub async fn top_people(
        &self,
        query: PageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Person>> {
        Ok(self
            .request(format!("top/people{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the most favorited characters
    pub async fn top_character(
        &self,
        query: PageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<CharacterDetails>> {
        Ok(self
            .request(format!("top/characters{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the most helpful reviews
    pub async fn top_reviews(
        &self,
        query: TopReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("top/reviews{query}"))
            .await?
            .result_paginator()?)
    }
}
