use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse,
        query::{SfwQuery, StackQuery},
        stacks::Stack,
    },
};

/// API Functions related to Intrest Stacks
impl TenraiAPI {
    ///Search interest stacks. Interest stacks are themed lists of anime or manga made by users.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/interest-stacks/GET/stacks>
    pub async fn stacks(&self, query: StackQuery) -> crate::Result<TenraiPaginatorResponse<Stack>> {
        Ok(self
            .request(format!("stacks{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get an interest stack by its ID.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/interest-stacks/GET/stacks/{id}>
    pub async fn stacks_details(
        &self,
        id: u64,
        query: SfwQuery,
    ) -> crate::Result<TenraiResponse<Stack>> {
        Ok(self
            .request(format!("stacks/{id}{query}"))
            .await?
            .result()?)
    }
}
