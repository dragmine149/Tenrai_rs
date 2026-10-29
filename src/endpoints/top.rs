use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse,
        anime::{Anime, AnimeEpisode},
        characters::CharacterDetails,
        common::TopRecommendation,
        manga::Manga,
        people::Person,
        query::{
            PageQuery, TopAnimeQuery, TopEpisodeQuery, TopMangaQuery, TopReviewQuery,
            TypeSfwPageQuery,
        },
        reviews::Review,
    },
};

/// API Functions related to Ranking
impl TenraiAPI {
    /// Get the highest ranking anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/anime>
    pub async fn top_anime(
        &self,
        query: TopAnimeQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("top/anime{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the highest ranking Manga
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/manga>
    pub async fn top_manga(
        &self,
        query: TopMangaQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Manga>> {
        Ok(self
            .request(format!("top/manga{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get the most favorited people
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/people>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/characters>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/reviews>
    pub async fn top_reviews(
        &self,
        query: TopReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("top/reviews{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get recommendations the most users agree on. Only pairs recommended by at least 2 users are listed.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/recommendations>
    pub async fn top_recommendations(
        &self,
        query: TypeSfwPageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<TopRecommendation>> {
        Ok(self
            .request(format!("top/recommendations{query}"))
            .await?
            .result_paginator()?)
    }

    /// Get the highest-rated anime episodes.
    ///
    /// Default list is weighted, episodes with few votes rank lower than their score would place them.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/top/GET/top/episodes>
    pub async fn top_episodes(
        &self,
        query: TopEpisodeQuery,
    ) -> crate::Result<TenraiPaginatorResponse<AnimeEpisode>> {
        Ok(self
            .request(format!("top/episodes{query}"))
            .await?
            .result_paginator()?)
    }
}
