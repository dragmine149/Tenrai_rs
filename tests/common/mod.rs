//! Tests related to various utils aspects. Kinda not meant to be testing one specific endpoint.

use crate::build_client;
use tenrai::schemas::{anime::AnimeType, common::AgeRating, query::AnimeQuery};

/// Test to see if the error response works.
/// *as no test should error*
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

/// Test to see how our rate limiter copes.
///
/// Requires feature `auto_retry`
#[tokio::test]
#[ignore = "There should not be any major need to test this"]
#[cfg(feature = "auto_retry")]
async fn test_auto_retry() {
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

#[tokio::test]
#[ignore = "There should not be any major need to test this"]
#[cfg(feature = "rate")]
async fn test_rate_limiter() {
    use std::time::Duration;
    use tokio::time::Instant;

    let client = build_client();
    let duration = if client.has_server_key() {
        Duration::from_millis(200)
    } else {
        Duration::from_millis(500)
    };
    let upper = duration + Duration::from_millis(100);
    let lower = duration - Duration::from_millis(100);

    let mut futures = vec![];
    // limit is 20 as we should be able to hit the /second limit rather than the /min limit.
    for _ in 0..20 {
        futures.push(client.anime_details(1));
    }
    let mut last = Instant::now();
    let mut index = 0;
    for future in futures {
        future.await.expect("The request should be fine...");
        if index > 1 {
            let diff = Instant::now().duration_since(last);
            println!("{:?}", diff);
            if diff >= upper {
                panic!("Request took too long to complete ({diff:?})")
            }
            if diff <= lower {
                panic!("Request completed too quickly ({diff:?})")
            }
        }
        index += 1;
        last = Instant::now();
    }
}

/// Test to see how the query generation works as some things are parsed differently.
#[test]
fn test_query_gen() {
    let query = AnimeQuery::builder()
        .anime_type([AnimeType::Tv, AnimeType::Movie, AnimeType::TvSpecial])
        .query("Some Anime")
        .rating([AgeRating::Pg13])
        .build()
        .expect("Failed to build");
    assert_eq!(
        format!("{query}"),
        "?q=Some Anime&type=tv,movie,tv_special&rating=pg13"
    )
}

/// Test to see how long queries are dealt with.
#[test]
fn test_long_query() {
    let query = AnimeQuery::builder()
        .query(include_str!("../data/long_text.txt"))
        .build()
        .expect_err("Should panic for being too long");
    assert_eq!(query.to_string(), "Query length is too long");
}

/// Test designed for quickly checking an endpoint as trying to find the cause can be confusing.
#[test]
fn local_parsing() {
    serde_json::from_str::<
        tenrai::schemas::TenraiPaginatorResponse<tenrai::schemas::article::Article>,
    >(include_str!("../data/temp.json"))
    .expect("Failed to parse json");
}
