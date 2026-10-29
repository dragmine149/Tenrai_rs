use crate::{TestBuilder, build_client};
use tenrai::{
    TenraiAPI,
    schemas::{common::Season, query::SeasonalQuery},
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
            &rand::random_range(1990_u64..2027_u64),
            &Season::Winter,
            SeasonalQuery::default()
        )
        .await)
);
