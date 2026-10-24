use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, anime::Anime, query::ScheduleQuery},
};

/// API Functions related to Schedules
impl TenraiAPI {
    /// Get the weekly broadcast schedule of airing anime
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/schedules/GET/schedules>
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
