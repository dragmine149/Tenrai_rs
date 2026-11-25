use crate::{
    TenraiAPI,
    schemas::{
        TenraiPaginatorResponse, TenraiResponse, common::External, companies::Company,
        query::CompanyQuery,
    },
};

/// API Functions related to news Companies
impl TenraiAPI {
    /// Search for a company on MAL
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers>
    #[doc(alias = "producers_search")]
    pub async fn company_search(
        &self,
        query: CompanyQuery,
    ) -> crate::Result<TenraiPaginatorResponse<Company>> {
        Ok(self
            .request(format!("producers{query}"))
            .await?
            .result_paginator()?)
    }
    /// Get details about a company
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/{id}>
    #[doc(alias = "producers_details")]
    pub async fn company_details(&self, id: &u32) -> crate::Result<TenraiResponse<Company>> {
        Ok(self.request(format!("producers/{id}")).await?.result()?)
    }
    /// Get full details about a company
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/{id}/full>
    #[doc(alias = "producers_full_details")]
    pub async fn company_full_details(&self, id: &u32) -> crate::Result<TenraiResponse<Company>> {
        Ok(self
            .request(format!("producers/{id}/full"))
            .await?
            .result()?)
    }
    /// Get external links  to companies sites
    ///
    /// API DOCUMENTATION: <https://api.tenrai.org/documentation#tag/producers/GET/producers/{id}/external>
    #[doc(alias = "producers_external")]
    pub async fn company_external(&self, id: &u32) -> crate::Result<TenraiResponse<Vec<External>>> {
        Ok(self
            .request(format!("producers/{id}/external"))
            .await?
            .result()?)
    }
}
