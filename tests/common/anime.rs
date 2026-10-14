// #[cfg(test)]
// mod anime_tests {
use crate::common::build_client;
use tenrai::schemas::{
    AgeRating, TenraiResponse,
    anime::{Anime, AnimeQuery, AnimeType},
};

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
#[test]
fn test_long_query() {
    let query = AnimeQuery::builder()
        .query(include_str!("../data/long_text.txt"))
        .build()
        .expect_err("Should panic for being too long");
    assert_eq!(query.to_string(), "Query length is too long");
}

// #[tokio::test]
// async fn search_anime() {
//     let client = build_client();
//     let data = client
//         .anime_search(AnimeQuery::builder().q("e").build().unwrap())
//         .await
//         .expect("Failed to send api")
//         .data;
// }
#[tokio::test]
async fn anime_details() {
    let client = build_client();
    let data = client
        .anime_details(1)
        .await
        .expect("Failed to send api")
        .data;
    // println!("{:?}", data);
    let doc_test_data =
        serde_json::from_str::<TenraiResponse<Anime>>(include_str!("../data/anime_details.json"))
            .expect("Failed to parse local test data")
            .data;

    assert_eq!(data.mal_id, doc_test_data.mal_id);
    assert_eq!(data.url, doc_test_data.url);
    assert_eq!(data.images, doc_test_data.images);
    assert_eq!(data.trailer.youtube_id, doc_test_data.trailer.youtube_id);
    assert_eq!(data.trailer.url, doc_test_data.trailer.url,);
    assert_eq!(data.trailer.embed_url, doc_test_data.trailer.embed_url);
    assert_eq!(data.trailer.images, doc_test_data.trailer.images);
    assert_eq!(data.trailer.title, doc_test_data.trailer.title);
    // views, likes, etc are harder to track as they are dynamic.
    // as long as they exist in some capacity. I'm happy.
    assert!(data.trailer.views.is_some());
    assert!(data.trailer.likes.is_some());
    assert!(data.trailer.dislikes.is_some());
    assert!(data.trailer.comment_count.is_some());
    assert_eq!(
        data.trailer.published_at,
        doc_test_data.trailer.published_at
    );
    assert_eq!(data.trailer.duration, doc_test_data.trailer.duration);
    assert_eq!(
        data.trailer.privacy_status,
        doc_test_data.trailer.privacy_status
    );
    assert_eq!(
        data.trailer.region_restriction,
        doc_test_data.trailer.region_restriction
    );
    assert_eq!(data.trailer.embeddable, doc_test_data.trailer.embeddable);
    assert_eq!(data.approved, doc_test_data.approved);
    assert_eq!(data.titles, doc_test_data.titles);
    assert_eq!(data.title, doc_test_data.title);
    assert_eq!(data.title_english, doc_test_data.title_english);
    assert_eq!(data.title_japanese, doc_test_data.title_japanese);
    assert_eq!(data.title_synonyms, doc_test_data.title_synonyms);
    assert_eq!(data.anime_type, doc_test_data.anime_type);
    assert_eq!(data.source, doc_test_data.source);
    assert_eq!(data.episodes, doc_test_data.episodes);
    assert_eq!(data.status, doc_test_data.status);
    assert_eq!(data.airing, doc_test_data.airing);
    assert_eq!(data.aired, doc_test_data.aired);
    assert_eq!(data.duration, doc_test_data.duration);
    assert_eq!(data.rating, doc_test_data.rating);
    assert!(data.score.is_some());
    assert!(data.scored_by.is_some());
    assert!(data.rank.is_some());
    assert!(data.popularity.is_some());
    assert!(data.members.is_some());
    assert!(data.favorites.is_some());
    assert_eq!(data.synopsis, doc_test_data.synopsis);
    assert_eq!(data.background, doc_test_data.background);
    assert_eq!(data.season, doc_test_data.season);
    assert_eq!(data.year, doc_test_data.year);
    assert_eq!(data.broadcast, doc_test_data.broadcast);
    assert_eq!(data.producers, doc_test_data.producers);
    assert_eq!(data.licensors, doc_test_data.licensors);
    assert_eq!(data.studios, doc_test_data.studios);
    assert_eq!(data.genres, doc_test_data.genres);
    assert_eq!(data.explicit_genres, doc_test_data.explicit_genres);
    assert_eq!(data.themes, doc_test_data.themes);
    assert_eq!(data.demographics, doc_test_data.demographics);
}
// }

#[tokio::test]
#[ignore = "This sends upwards of 300 requests hence takes like a minute or two."]
async fn check_parsing() {
    let client = build_client();
    let mut page = 1_u64;
    loop {
        println!("{page}");
        let result = client
            .anime_search(
                AnimeQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
            .expect("This should not fail");
        page += 1;
        if !result.pagination.has_next_page {
            break;
        }
    }
}
