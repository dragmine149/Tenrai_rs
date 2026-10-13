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
            tenrai::errors::Error::Tenrai(_)
        ))
    }

    #[tokio::test]
    #[ignore = "There should not be any major need to test this"]
    async fn test_rate_limit() {
        let client = build_client();
        let mut futures = vec![];
        // limit is 20 as we should be able to hit the /second limit rather than the /min limit.
        for _ in 0..20 {
            futures.push(client.anime_details(1));
        }
        for future in futures {
            future
                .await
                .expect("Nothing should have failed just because we rate-limited");
        }
    }
}

use tenrai::TenraiAPI;

pub(crate) fn build_client() -> TenraiAPI {
    TenraiAPI::builder().cache(false).build().unwrap()
}
