#[macro_export]
macro_rules! generic_type {
    ($name: tt) => {
        pastey::paste! {
           #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
            pub struct [<$name:upper_camel Type>] {
                pub mal_id: u64,
                pub name: String,
                #[serde(rename = "type")]
                pub [<$name:lower_camel _type>]: String,
                pub url: String,
            }
        }
    };
}
