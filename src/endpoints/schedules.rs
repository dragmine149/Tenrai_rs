use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, anime::Anime, query::ScheduleQuery},
};

impl TenraiAPI {
    /// Get the weekly broadcast schedule of airing anime
    pub async fn schedules(
        &self,
        query: ScheduleQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Anime>> {
        Ok(self
            .request(format!("schedules{query}"))
            .await?
            .result_paginator()?)
    }
}
