use crate::{TestBuilder, build_client};
use tenrai::{
    TenraiAPI,
    schemas::query::GenreQuery,
};

TestBuilder!(
    genres_anime,
    (async |client: &TenraiAPI, _| { client.genres_anime(GenreQuery::default()).await })
);
TestBuilder!(
    genres_manga,
    (async |client: &TenraiAPI, _| { client.genres_manga(GenreQuery::default()).await })
);
