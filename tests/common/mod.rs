#[cfg(test)]
mod anime;

/// Test suite to test parts of the API which aren't directly related to making successful api calls.
#[cfg(test)]
mod main_tests {
    use crate::common::build_client;

    #[tokio::test]
    async fn test_error() {
        let client = build_client();
        let result = client.anime_details(0).await;
        println!("{:?}", result);
        assert!(matches!(
            result.err().unwrap(),
            tenari::errors::Error::Tenrai(_)
        ))
    }
}

use tenari::TenraiAPI;

pub(crate) fn build_client() -> TenraiAPI {
    TenraiAPI::builder().cache(false).build().unwrap()
}
