mod common;
mod endpoints;

use std::time::Duration;

use tenrai::TenraiAPI;

/// Build a client. This is used in every test.
///
/// If you wish to test server key aspects, include the `server-key` as an environment variable.
pub(crate) fn build_client() -> TenraiAPI {
    let mut builder = TenraiAPI::builder();
    builder.cache(Duration::default());
    // builder.cache(true);
    if let Some(sk) = option_env!("server-key") {
        builder.server_key(sk);
    }
    builder.build().unwrap()
}

/// - Returns `true` if a valid error occurred which we need to re-request for.
/// - Returns `false` elsewise.
pub(crate) async fn error<T>(result: &tenrai::Result<T>) -> bool {
    #[allow(
        clippy::collapsible_match,
        reason = "There could be reasons why we emit the other errors."
    )]
    match result {
        Ok(_) => false,
        Err(e) => match e {
            tenrai::errors::TenraiError::Tenrai(tenrai_apierror) => {
                match tenrai_apierror.error_type {
                    // Reattempt in case we hit an entry which doesn't exist.
                    tenrai::errors::MachineErrorType::ResourceNotFoundException => true,
                    _ => false,
                }
            }
            // This is because cloudflare was giving us some issues with long running tests.
            tenrai::errors::TenraiError::Reqwest(re) => {
                let sleep = re.is_timeout() || re.is_connect();
                if sleep {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                sleep
            }
            _ => false,
        },
    }
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
            for _ in 0..5 {
                let random = rand::random_range(1_u32..100_000_u32);
                println!("ID: {random}");
                let res = $func(&client, &random).await;
                if $crate::error(&res).await {
                	println!("Hit error {:?}", res.err());
                	continue;
                } else {
                	res.expect("Test failed somehow...");
                 	break;
                }
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
                let mut page = 1_u32;
                for _ in 0..15 {
                    println!("Page: {page}");
                    let res = $func(&client, page).await;
                    let res = {
                        if $crate::error(&res).await {
                            res.unwrap()
                        } else {
                            res.expect("This should not fail")
                        }
                    };
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

#[macro_export]
macro_rules! TestQuery {
    (
    	$func_name: ident, $query: ident,
    	$func: expr,
     	$count: expr,
      	{
       		$(
         		$name:ident:$query_builder: expr,
         	)+
       	}
    ) => {
        pastey::paste! {
           #[tokio::test]
            async fn [<test_ $func_name _query>]() {
            	env_logger::init();
                let client = build_client();
                let mut rng = rand::rng();
                let base_data = $count(
                	$func(&client, $query::default())
                    .await
                    .expect("This should not fail")
                );
                $(
                	let mut builder = $query::builder();
                 	$query_builder(&mut builder, &mut rng);
                	let $name = $count(
                 		$func(&client, builder.build().unwrap())
                   		.await
                     	.expect("This should not fail")
                	);
                 	assert_ne!(base_data, $name, concat!("Panic in ", stringify!($name)));
                )+
            }
        }
    };
}
