mod common;
mod endpoints;

use tenrai::TenraiAPI;

/// Build a client. This is used in every test.
///
/// If you wish to test server key aspects, include the `server-key` as an environment variable.
pub(crate) fn build_client() -> TenraiAPI {
    let mut builder = TenraiAPI::builder();
    builder.cache(false);
    builder.cache(true);
    if let Some(sk) = option_env!("server-key") {
        builder.server_key(sk);
    }
    builder.build().unwrap()
}

/// Macro for making tests with.
///
/// `$func_name` is the function to test. Due to limitations, the function will need to be repeated in `$func` callback.
/// `$id` is an optional field but used in place of the randomiser.
/// `$attr` is an additional optional field, used for things like `ignore = "..."`
#[macro_export]
macro_rules! TestBuilder {
    ($func_name:ident, $func:expr) => {
        $crate::TestBuilder!(@impl $func_name, $func, [l], []);
    };
    ($func_name:ident, $func:expr, $attr:meta) => {
        $crate::TestBuilder!(@impl $func_name, $func, [l], [$attr]);
    };
    ($func_name:ident, $func:expr, $id:expr) => {
        $crate::TestBuilder!(@impl $func_name, $func, [$id], []);
    };
    ($func_name:ident, $func:expr, $id:expr, $attr:meta) => {
        $crate::TestBuilder!(@impl $func_name, $func, [$id], [$attr]);
    };

    (@impl $func_name:ident, $func:expr, [l], [$($attr:meta)*]) => {
        #[tokio::test]
        $(#[$attr])*
        async fn $func_name() {
            let client = build_client();
            loop {
                let random = rand::random_range(1..70_000);
                println!("ID: {random}");
                let res = $func(&client, random).await;
                if let Err(ref e) = res
                    && let TenraiError::Tenrai(e) = e
                {
                    if e.error_type == MachineErrorType::ResourceNotFoundException {
                        continue;
                    }
                }
                res.expect("Failed to call API");
                break;
            }
        }
    };
    (@impl $func_name:ident, $func:expr, [$id:expr], [$($attr:meta)*]) => {
        #[tokio::test]
        $(#[$attr])*
        async fn $func_name() {
            let client = build_client();
            $func(&client, $id).await.expect("Failed to call API");
        }
    };
}

/// Macro for making tests specified to testing the paginator endpoints.
///
/// These tests can take many minutes to run as although we're doing 100 entries per page, we are still limited.
#[macro_export]
macro_rules! TestPaginator {
    ($func_name: ident, $func:expr) => {
        pastey::paste! {
           #[tokio::test]
           #[ignore = "Paginator test, can take many minutes."]
           async fn [<$func_name _paginator>]() {
               let client = build_client();
               let mut page = 1_u64;
               for _ in 0..120 {
                   let res = $func(&client, page).await.expect("This should not fail");
                   if page == 1 {
                       page = rand::random_range(1..res.pagination.last_visible_page);
                       println!("Chosen page: {page}");
                   }
                   page += 1;
                   if !res.pagination.has_next_page {
                       break;
                   }
               }
           }
        }
    };
}
