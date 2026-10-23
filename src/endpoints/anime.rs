use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse, TenraiTotalResponse,
        anime::{Anime, AnimeEpisode, AnimeStatistics, AnimeVideo, AnimeVideoEpisode, ThemeSongs},
        article::Article,
        common::{
            External, Forum, IDList, Images, MoreInfo, Recommendation, Relation, StatsHistory,
        },
        people::{AnimeStaff, Character},
        query::{
            AnimeIdsQuery, AnimeQuery, ArticleQuery, BasicPageQuery, BasicQuery, ForumQuery,
            PageSfwQuery, ReviewQuery, SfwQuery, StackQuery, StatsHistoryQuery,
        },
        reviews::Review,
        stacks::Stack,
    },
};

/// API functions related to anime.
impl TenraiAPI {
    /// Search for an anime with filters.
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
    pub async fn anime_details(&self, id: u64) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self.request(format!("anime/{id}")).await?.result()?)
    }
    /// Get full detailed data for an anime
    pub async fn anime_full_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Anime>> {
        Ok(self
            .request(format!("anime/{id}/full{query}"))
            .await?
            .result()?)
    }
    /// Get recommendations for an anime
    pub async fn anime_recommendations(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<Recommendation>>> {
        Ok(self
            .request(format!("anime/{id}/recommendations{query}"))
            .await?
            .result()?)
    }
    /// Get character list and voice actors for an anime
    pub async fn anime_characters(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Character>>> {
        Ok(self
            .request(format!("anime/{id}/characters"))
            .await?
            .result()?)
    }
    ///Get staff for an anime
    pub async fn anime_staff(&self, id: u64) -> crate::Result<TenraiResponse<Vec<AnimeStaff>>> {
        Ok(self.request(format!("anime/{id}/staff")).await?.result()?)
    }
    /// Get episodes for an anime
    pub async fn anime_episodes(
        &self,
        id: u64,
        query: BasicPageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<AnimeEpisode>> {
        Ok(self
            .request(format!("anime/{id}/episodes{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get a list of news articles related to the anime
    pub async fn anime_news(
        &self,
        id: u64,
        query: PageSfwQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Article>> {
        Ok(self
            .request(format!("anime/{id}/news{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get featured articles related to the anime.
    pub async fn anime_articles(
        &self,
        id: u64,
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
    pub async fn anime_forum(
        &self,
        id: u64,
        query: ForumQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Forum>> {
        Ok(self
            .request(format!("anime/{id}/forum{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details for a specific anime episode
    pub async fn anime_episode(
        &self,
        id: u64,
        episode_id: u64,
    ) -> crate::Result<TenraiResponse<AnimeEpisode>> {
        Ok(self
            .request(format!("anime/{id}/episodes/{episode_id}"))
            .await?
            .result()?)
    }
    /// Get videos for an anime
    pub async fn anime_videos(&self, id: u64) -> crate::Result<TenraiResponse<AnimeVideo>> {
        Ok(self.request(format!("anime/{id}/videos")).await?.result()?)
    }
    /// Get episode videos for an anime
    pub async fn anime_video_episodes(
        &self,
        id: u64,
        query: BasicPageQuery,
    ) -> crate::Result<TenraiPaginatorResponse<AnimeVideoEpisode>> {
        Ok(self
            .request(format!("anime/{id}/videos/episodes{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get pictures for an anime
    pub async fn anime_pictures(&self, id: u64) -> crate::Result<TenraiResponse<Vec<Images>>> {
        Ok(self
            .request(format!("anime/{id}/pictures"))
            .await?
            .result()?)
    }
    /// Get statistics for an anime
    pub async fn anime_statistics(
        &self,
        id: u64,
    ) -> crate::Result<TenraiResponse<AnimeStatistics>> {
        Ok(self
            .request(format!("anime/{id}/statistics"))
            .await?
            .result()?)
    }
    /// Get historical statistics for an anime
    ///
    /// SEE: <https://api.tenrai.org/documentation#tag/anime/GET/anime/{id}/statistics/history> for more information.
    pub async fn anime_statistics_historical(
        &self,
        id: u64,
        query: StatsHistoryQuery,
    ) -> crate::Result<TenraiPaginatorResponse<StatsHistory<AnimeStatistics>>> {
        Ok(self
            .request(format!("anime/{id}/statistics/history{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get more information for an anime
    pub async fn anime_more_info(&self, id: u64) -> crate::Result<TenraiResponse<MoreInfo>> {
        Ok(self
            .request(format!("anime/{id}/moreinfo"))
            .await?
            .result()?)
    }
    /// Get relations for an anime
    pub async fn anime_relations(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Vec<Relation>>> {
        Ok(self
            .request(format!("anime/{id}/relations{query}"))
            .await?
            .result()?)
    }
    /// Get opening and ending themes for an anime
    pub async fn anime_themes(&self, id: u64) -> crate::Result<TenraiResponse<ThemeSongs>> {
        Ok(self.request(format!("anime/{id}/themes")).await?.result()?)
    }
    /// Get external links for an anime
    pub async fn anime_external(&self, id: u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("anime/{id}/external"))
            .await?
            .result()?)
    }
    /// Get streaming links for an anime
    pub async fn anime_streaming(&self, id: u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("anime/{id}/streaming"))
            .await?
            .result()?)
    }
    /// Get reviews for an anime
    pub async fn anime_reviews(
        &self,
        id: u64,
        query: ReviewQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Review>> {
        Ok(self
            .request(format!("anime/{id}/reviews{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get all public interest stacks for an anime
    pub async fn anime_stacks(
        &self,
        id: u64,
        query: StackQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Stack>> {
        Ok(self
            .request(format!("anime/{id}/stacks{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get anime waiting for MAL approval
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
