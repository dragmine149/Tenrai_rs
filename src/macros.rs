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

#[macro_export]
macro_rules! query {
    ($name:ident { $($var:ident:$var_type:ty $(:$fmt:expr)?),* }) => {
        pastey::paste! {
            #[derive(Debug, Builder)]
            pub struct [<$name:upper_camel Query>] {
            	$(
             		#[builder(setter(into, strip_option), default)]
                    $var: Option<$var_type>,
                )*
            }
            impl [<$name:upper_camel Query>] {
            	pub fn builder() -> [<$name:upper_camel QueryBuilder>] {
             		[<$name:upper_camel QueryBuilder>]::default()
             	}
            }
            impl std::fmt::Display for [<$name:upper_camel Query>] {
            	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
             		let mut query = vec![];
               		$(
                 		if let Some(v) = self.$var.as_ref() {
                 			query.push(format!("{}={}", stringify!($var), query!(@format v, $($fmt)?)));
                   		}
                 	)*
                  	if query.len() > 0 {
                   		write!(f, "?{}", query.join("&"))
                   	} else {
                    	write!(f, "")
                    }
             	}
            }
        }
    };

    (@format $field:expr, $fmt:expr) => {
    	$fmt($field)
    };
    (@format $field:expr,) => {
    	format!("{}", $field)
    }
}
