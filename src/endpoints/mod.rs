#[cfg(feature = "custom")]
use crate::{
    TenraiAPI,
    schemas::{TenraiPaginatorResponse, TenraiRaw, TenraiRawPaginator, TenraiResponse},
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
        Ok(self
            .request::<TenraiRaw<T>>(endpoint.into())
            .await?
            .result()?)
    }
    pub async fn custom_paginator<T>(
        &self,
        endpoint: impl Into<String>,
    ) -> crate::Result<TenraiPaginatorResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .request::<TenraiRawPaginator<T>>(endpoint.into())
            .await?
            .result()?)
    }
}
