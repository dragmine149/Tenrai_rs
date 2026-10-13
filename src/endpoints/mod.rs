#[cfg(feature = "custom")]
use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, TenraiResponse},
};
#[cfg(feature = "custom")]
use serde::de::DeserializeOwned;

mod anime;

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
