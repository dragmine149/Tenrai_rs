use tenrai::schemas::query::ScheduleQuery;

use crate::build_client;

#[tokio::test]
async fn schedules() {
    let client = build_client();
    client
        .schedules(ScheduleQuery::default())
        .await
        .expect("Failed to get API");
}
