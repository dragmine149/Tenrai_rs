use crate::{TestBuilder, build_client};
use tenrai::{
    TenraiAPI,
    schemas::{anime::AnimeStatus, common::Season, filter::SeasonOrder, query::SeasonalQuery},
};

TestBuilder!(
    seasonal_anime_current,
    (async |client: &TenraiAPI, _| client
        .seasonal_anime_current(SeasonalQuery::default())
        .await)
);
TestBuilder!(
    seasonal_anime_previous,
    (async |client: &TenraiAPI, _| client
        .seasonal_anime_previous(SeasonalQuery::default())
        .await)
);
TestBuilder!(
    seasonal_anime_upcoming,
    (async |client: &TenraiAPI, _| client
        .seasonal_anime_upcoming(SeasonalQuery::default())
        .await)
);
TestBuilder!(
    seasonal_all,
    (async |client: &TenraiAPI, _| client.seasonal_all().await)
);

TestBuilder!(
    seasonal_winter_anime,
    (async |client: &TenraiAPI, _| client
        .seasonal_anime(
            &rand::random_range(1990_u32..2027_u32),
            &Season::Winter,
            SeasonalQuery::default()
        )
        .await)
);

TestBuilder!(
    seasonal_order_30d,
    (async |client: &TenraiAPI, _| client
        .seasonal_anime_current(
            SeasonalQuery::builder()
                .order_by(SeasonOrder::ScoreChange30d)
                .build()
                .unwrap()
        )
        .await)
);

#[tokio::test]
async fn test_seasonal_upcoming_status() {
    let client = build_client();
    client
        .seasonal_anime_upcoming(
            SeasonalQuery::builder()
                .status(vec![AnimeStatus::Airing])
                .build()
                .unwrap(),
        )
        .await
        .expect_err("This should not be allowed");
    client
        .seasonal_anime_upcoming(
            SeasonalQuery::builder()
                .status_exclude(vec![AnimeStatus::Airing])
                .build()
                .unwrap(),
        )
        .await
        .expect_err("This should not be allowed");
}
