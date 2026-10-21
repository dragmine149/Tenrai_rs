use crate::{TestBuilder, build_client};
use tenrai::{
    TenraiAPI,
    errors::{MachineErrorType, TenraiError},
    schemas::query::SfwQuery,
};

TestBuilder!(
    random_anime,
    (async |client: &TenraiAPI, _| client.random_anime(SfwQuery::default()).await)
);
TestBuilder!(
    random_manga,
    (async |client: &TenraiAPI, _| client.random_manga(SfwQuery::default()).await)
);
TestBuilder!(
    random_character,
    (async |client: &TenraiAPI, _| client.random_character(SfwQuery::default()).await)
);
TestBuilder!(
    random_person,
    (async |client: &TenraiAPI, _| client.random_person(SfwQuery::default()).await)
);
