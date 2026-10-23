use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse, common::External, companies::Company,
        query::ProducerQuery,
    },
};

/// API Functions related to news Companies
impl TenraiAPI {
    /// Search for a company on MAL
    pub async fn company_search(
        &self,
        query: ProducerQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Company>> {
        Ok(self
            .request(format!("producers{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details about a company
    pub async fn company_details(&self, id: u64) -> crate::Result<TenraiResponse<Company>> {
        Ok(self.request(format!("producers/{id}")).await?.result()?)
    }
    /// Get full details about a company
    pub async fn company_full_details(&self, id: u64) -> crate::Result<TenraiResponse<Company>> {
        Ok(self
            .request(format!("producers/{id}/full"))
            .await?
            .result()?)
    }
    /// Get external links  to companies sites
    pub async fn company_external(&self, id: u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("producers/{id}/external"))
            .await?
            .result()?)
    }
    /// Get the ID of every company in one request
    ///
    /// NOTE: This requires a server key.
    pub async fn company_ids(&self) -> crate::Result<TenraiResponse<Vec<u64>>> {
        self.check_server_key()?;
        Ok(self.request("producers/ids".to_string()).await?.result()?)
    }
}
