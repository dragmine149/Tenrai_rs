use tenrai::{TenraiAPI, schemas::query::MangaQuery};

use crate::{TestPaginator, build_client};

// #[tokio::test]
// #[ignore = "This sends upwards of 300 requests hence takes like a minute or two."]
// async fn manga_search() {
//     let client = build_client();
//     let mut page = 1_u64;
//     loop {
//         println!("{page}");
//         let result = client
//             .manga_search(
//                 MangaQuery::builder()
//                     .page(page)
//                     .limit(100_u64)
//                     .build()
//                     .unwrap(),
//             )
//             .await
//             .expect("This should not fail");
//         page += 1;
//         if !result.pagination.has_next_page {
//             break;
//         }
//     }
// }

TestPaginator!(
    manga_search,
    (async |client: &TenraiAPI, page| {
        client
            .manga_search(
                MangaQuery::builder()
                    .page(page)
                    .limit(100_u64)
                    .build()
                    .unwrap(),
            )
            .await
    }),
    ignore = "This takes 1 minutes to complete on it's own (public tier)"
);
