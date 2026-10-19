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
