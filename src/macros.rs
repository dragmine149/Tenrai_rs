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

/// Helper macro for making errors.
///
/// # Usage
/// ```ignore (macros are private)
/// error!(Cache {
///     Cache: std::io::Error,
///     Reqwest: reqwest::Error,
///     Serde: serde_json::Error,
/// });
/// ```
/// Basically just define a struct of the errors.
#[macro_export]
macro_rules! error {
    ($name: tt {
    	$(
    		$(#[$field_meta:meta])?
     		$error:ident:$error_type:ty,
     	)+
    }) => {
        pastey::paste! {
        	#[derive(Debug)]
            pub enum [<$name:upper_camel Error>] {
            	$(
             		$(#[$field_meta])?
             		[<$error:upper_camel>]($error_type),
             	)*
            }
            impl std::error::Error for [<$name:upper_camel Error>] {}
            impl std::fmt::Display for [<$name:upper_camel Error>] {
            	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
             		writeln!(
               			f, "{}", match self {
                  			$(
                     			Self::[<$error:upper_camel>](e) => e.to_string(),
                     		)+
                  		}
               		)
             	}
            }
            $(
            	impl From<$error_type> for [<$name:upper_camel Error>] {
             		fn from(value: $error_type) -> Self {
               			Self::[<$error:upper_camel>](value)
               		}
             	}
            )*
        }
    };
}
