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

/// Helper macro for making query objects.
///
/// Makes a full struct/builder/validation combo from little input.
///
/// # Usage
/// ```ignore
/// query!(name { DataStruct }, { ValidationStruct })
/// ```
/// - `DataStruct` is done like a normal struct, with optional `:{format_func}` for cases where `T` doesn't have `to_string()`
/// - `ValidationStruct` is done in the format of `{var}: {reason}: {func}`.
///
/// # Example
/// ```ignore
///query!(Anime {
///     q: String,
///     page: u64,
///     limit: u64,
///     /// Filter by media type. Multiple types can be selected.
///     anime_type: Vec<AnimeType>:query_vec,
///     status: Vec<AnimeStatus>:query_vec,
///     rating: Vec<AnimeRating>:query_vec,
///     sfw: String,
///     sfw_strict: String,
///     unapproved: String,
///     score: u64,
///     min_score: u64,
///     max_score: u64,
///     genres: String,
///     genres_exclude: String,
///     order_by: String,
///     sort: String,
///     letter: char,
///     producers: String,
///     start_date: String,
///     end_date: String,
/// }, {
///     q: "Query length is too long": (|q: &String| q.len() > 300),
///     page: "Page must be less than 20k": (|p: &u64| *p < 20000),
///     limit: "Can not request more than 100 entries at once": (|l: &u64| l > 100),
/// });
/// ```
#[macro_export]
macro_rules! query {
    ($name:ident {
    	$(
     		$(#[$field_meta:meta])?
     		$var:ident:$var_type:ty $(:$fmt:expr)?,
     	)*
    }, {
    	$(
     		$builder_var:ident: $builder_var_msg:literal: $builder_var_func:expr,
     	)*
    }) => {
        pastey::paste! {
            #[derive(Debug, Builder)]
            #[builder(build_fn(validate = "Self::validate"))]
            pub struct [<$name:upper_camel Query>] {
            	$(
             		$(#[$field_meta])?
             		#[builder(setter(into, strip_option), default)]
                    $var: Option<$var_type>,
                )*
            }
            impl [<$name:upper_camel Query>] {
            	pub fn builder() -> [<$name:upper_camel QueryBuilder>] {
             		[<$name:upper_camel QueryBuilder>]::default()
             	}
            }
            impl [<$name:upper_camel QueryBuilder>] {
            	fn validate(&self) -> Result<(), String> {
             		$(
               			if let Some(ref v) = self.$builder_var && let Some(v) = v && $builder_var_func(v) {
                     		return Err($builder_var_msg.to_string())
                     	}
               		)*
                 	Ok(())
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
