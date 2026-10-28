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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers>
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
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/{id}>
    pub async fn company_details(&self, id: &u64) -> crate::Result<TenraiResponse<Company>> {
        Ok(self.request(format!("producers/{id}")).await?.result()?)
    }
    /// Get full details about a company
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/{id}/full>
    pub async fn company_full_details(&self, id: &u64) -> crate::Result<TenraiResponse<Company>> {
        Ok(self
            .request(format!("producers/{id}/full"))
            .await?
            .result()?)
    }
    /// Get external links  to companies sites
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/{id}/external>
    pub async fn company_external(&self, id: &u64) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("producers/{id}/external"))
            .await?
            .result()?)
    }
    /// Get the ID of every company in one request
    ///
    /// NOTE: This requires a server key.
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/ids>
    pub async fn company_ids(&self) -> crate::Result<TenraiResponse<Vec<u64>>> {
        self.check_server_key()?;
        Ok(self.request("producers/ids".to_string()).await?.result()?)
    }
}
