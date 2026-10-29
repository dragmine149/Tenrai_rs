use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse, TenraiTotalResponse,
        anime::{Anime, AnimeEpisode, AnimeStatistics, AnimeVideo, AnimeVideoEpisode, ThemeSongs},
        article::{Article, NewsArticle},
        common::{
            External, Forum, IDList, Images, MoreInfo, Recommendation, RelationDetails,
            StatsHistory,
        },
        people::{AnimeStaff, Character},
        query::{
            AnimeIdsQuery, AnimeQuery, ArticleQuery, BasicPageQuery, BasicQuery, ForumQuery,
            NewsArticleQuery, RecommendationQuery, ReviewQuery, SfwQuery, StackQuery,
            StatsHistoryQuery,
        },
        reviews::Review,
        stacks::Stack,
    },
};

/// API functions related to anime.
impl TenraiAPI {
    /// Search for an anime with filters.
    ///
    /// API Documentation: <https://api.tenrai.org/documentation#tag/anime/GET/anime>
    pub async fn anime_search(
        &self,
        query: AnimeQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("anime{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for an anime
    ///
    /// API Documentation: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}>
    pub async fn anime_details(&self, id: &u64) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self.request(format!("anime/{id}")).await?.result()?)
    }
    /// Get full detailed data for an anime
    ///
    /// API Documentation: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/full>
    pub async fn anime_full_details(
        &self,
        id: &u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request(format!("anime/{id}/full{query}"))
            .await?
            .result()?)
    }
    /// Get recommendations for an anime
    ///
    /// API Documentation: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/recommendations>
    pub async fn anime_recommendations(
        &self,
        id: &u64,
        query: RecommendationQuery,
    ) -> crate::Result<TenraiResponse<Vec<Recommendation>>> {
        Ok(self
            .request(format!("anime/{id}/recommendations{query}"))
            .await?
            .result()?)
    }
    /// Get character list and voice actors for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/characters>
    pub async fn anime_characters(
        &self,
        id: &u64,
    ) -> crate::Result<TenraiResponse<Vec<Character>>> {
        Ok(self
            .request(format!("anime/{id}/characters"))
            .await?
            .result()?)
    }
    /// Get staff for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/staff>
    pub async fn anime_staff(&self, id: &u64) -> crate::Result<TenraiResponse<Vec<AnimeStaff>>> {
        Ok(self.request(format!("anime/{id}/staff")).await?.result()?)
    }
    /// Get episodes for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/episodes>
    pub async fn anime_episodes(
        &self,
        id: &u64,
        query: BasicPageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<AnimeEpisode>> {
        Ok(self
            .request(format!("anime/{id}/episodes{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get a list of news articles related to the anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/news>
    pub async fn anime_news(
        &self,
        id: &u64,
        query: NewsArticleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<NewsArticle>> {
        Ok(self
            .request(format!("anime/{id}/news{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get featured articles related to the anime.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/articles>
    pub async fn anime_articles(
        &self,
        id: &u64,
        query: ArticleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("anime/{id}/articles{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get forum topics related to the anime entry. Episode discussion threads only!
    ///
    /// NOTE: You can use the official MAL API to fetch the threads themselves by using the thread id.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/forum>
    pub async fn anime_forum(
        &self,
        id: &u64,
        query: ForumQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Forum>> {
        Ok(self
            .request(format!("anime/{id}/forum{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for a specific anime episode
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/episodes/{episode_id}>
    pub async fn anime_episode(
        &self,
        id: &u64,
        episode_id: &u64,
    ) -> crate::Result<TenraiResponse<AnimeEpisode>> {
        Ok(self
            .request(format!("anime/{id}/episodes/{episode_id}"))
            .await?
            .result()?)
    }
    /// Get videos for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/videos>
    pub async fn anime_videos(&self, id: &u64) -> crate::Result<TenraiResponse<AnimeVideo>> {
        Ok(self.request(format!("anime/{id}/videos")).await?.result()?)
    }
    /// Get episode videos for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/videos/episodes>
    pub async fn anime_video_episodes(
        &self,
        id: &u64,
        query: BasicPageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<AnimeVideoEpisode>> {
        Ok(self
            .request(format!("anime/{id}/videos/episodes{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get pictures for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/pictures>
    pub async fn anime_pictures(&self, id: &u64) -> crate::Result<TenraiResponse<Vec<Images>>> {
        Ok(self
            .request(format!("anime/{id}/pictures"))
            .await?
            .result()?)
    }
    /// Get statistics for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/statistics>
    pub async fn anime_statistics(
        &self,
        id: &u64,
    ) -> crate::Result<TenraiResponse<AnimeStatistics>> {
        Ok(self
            .request(format!("anime/{id}/statistics"))
            .await?
            .result()?)
    }
    /// Get historical statistics for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/statistics/history>
    pub async fn anime_statistics_historical(
        &self,
        id: &u64,
        query: StatsHistoryQuery,
    ) -> crate::Result<TenraiPaginatorResponse<StatsHistory<AnimeStatistics>>> {
        Ok(self
            .request(format!("anime/{id}/statistics/history{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get more information for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/moreinfo>
    pub async fn anime_more_info(&self, id: &u64) -> crate::Result<TenraiResponse<MoreInfo>> {
        Ok(self
            .request(format!("anime/{id}/moreinfo"))
            .await?
            .result()?)
    }
    /// Get relations for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/relations>
    pub async fn anime_relations(
        &self,
        id: &u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<RelationDetails>>> {
        Ok(self
            .request(format!("anime/{id}/relations{query}"))
            .await?
            .result()?)
    }
    /// Get opening and ending themes for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/themes>
    pub async fn anime_themes(&self, id: &u64) -> crate::Result<TenraiResponse<ThemeSongs>> {
        Ok(self.request(format!("anime/{id}/themes")).await?.result()?)
    }
    /// Get external links for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/external>
    pub async fn anime_external(&self, id: &u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("anime/{id}/external"))
            .await?
            .result()?)
    }
    /// Get streaming links for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/streaming>
    pub async fn anime_streaming(&self, id: &u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("anime/{id}/streaming"))
            .await?
            .result()?)
    }
    /// Get reviews for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/reviews>
    pub async fn anime_reviews(
        &self,
        id: &u64,
        query: ReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("anime/{id}/reviews{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get all public interest stacks for an anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/stacks>
    pub async fn anime_stacks(
        &self,
        id: &u64,
        query: StackQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Stack>> {
        Ok(self
            .request(format!("anime/{id}/stacks{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get anime waiting for MAL approval
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/unapproved>
    pub async fn anime_unapproved(
        &self,
        query: BasicQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("anime/unapproved{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get anime MAL denied
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/denied>
    pub async fn anime_denied(
        &self,
        query: BasicQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("anime/denied{query}"))
            .await?
            .result_paginator()?)
    }
    /// Browse deleted anime entries
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/deleted>
    pub async fn anime_deleted(
        &self,
        query: BasicQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("anime/deleted{query}"))
            .await?
            .result_paginator()?)
    }
    /// Retrieve all unique MAL anime IDs of approved entries that currently exist and are active.
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/anime/GET/anime/ids>
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
}
