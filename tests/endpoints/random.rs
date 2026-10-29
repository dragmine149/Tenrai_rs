use crate::{TestBuilder, build_client};
use tenrai::{
    TenraiAPI,
    errors::{MachineErrorType, TenraiError},
    schemas::query::{RandomAnimeQuery, RandomMangaQuery, RandomPersonQuery},
};

TestBuilder!(
    random_anime,
    (async |client: &TenraiAPI, _| client.random_anime(RandomAnimeQuery::default()).await)
);
TestBuilder!(
    random_manga,
    (async |client: &TenraiAPI, _| client.random_manga(RandomMangaQuery::default()).await)
);
TestBuilder!(
    random_character,
    (async |client: &TenraiAPI, _| client.random_character(RandomPersonQuery::default()).await)
);
TestBuilder!(
    random_person,
    (async |client: &TenraiAPI, _| client.random_person(RandomPersonQuery::default()).await)
);
