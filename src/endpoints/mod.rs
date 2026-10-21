#[cfg(feature = "custom")]
use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, TenraiResponse},
};
#[cfg(feature = "custom")]
use serde::de::DeserializeOwned;

mod anime;
mod characters;
mod companies;
mod manga;
mod people;
mod schedules;
mod seasonal;

/// Custom functions related to the API. Gives the security of the requests while the freedom of the response.
///
/// Shouldn't be needed in most cases.
#[cfg(feature = "custom")]
impl TenraiAPI {
    pub async fn custom<T>(&self, endpoint: impl Into<String>) -> crate::Result<TenraiResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(self.request(endpoint.into()).await?.result()?)
    }
    pub async fn custom_paginator<T>(
        &self,
        endpoint: impl Into<String>,
    ) -> crate::Result<TenraiPaginatorResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(self.request(endpoint.into()).await?.result_paginator()?)
    }
}
