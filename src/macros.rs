/// A macro to make a generic type. There are many different types of the same information, so this consolidates them all.
///
/// This also helps us with the `type` parameter as `type` is a keyword in rust.
#[macro_export]
macro_rules! generic_type {
    ($name: tt) => {
        pastey::paste! {
           #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
           #[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
            pub struct [<$name:upper_camel Type>] {
                /// The unique MAL ID of the catalog item
                pub mal_id: u64,
                /// The name of the catalog item
                pub name: String,
                #[serde(rename = "type")]
                /// The type of the catalog item
                pub [<$name:lower_camel _type>]: String,
                /// The MAL website link for this catalog item
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
/// ## format_func
/// A format function is given the stringified name of the field, plus the value to format. As such it can be used for both renaming and edge-cases.
///
/// For example
/// ```ignore
/// // renaming
/// query: String:(|_, v| format!("q={v}")),
///
/// // parsing
/// anime_type: Vec<AnimeType>:query_vec,
///
/// pub(crate) fn query_vec<T: Display>(var_name: &str, vec: &[T]) -> String {
///     format!(
///         "{var_name}={}",
///         vec.iter()
///             .map(|s| format!("{s}"))
///             .collect::<Vec<String>>()
///             .join(",")
///     )
/// }
/// ```
/// By default, if one is not provided, `format!("{var_name}={value}")` is used.
///
/// # Default
/// Note that we do not implement any method to deal with default variables. The server should take care of that.
#[macro_export]
macro_rules! query {
    ($(#[$query_meta:meta])? $name:ident {
    	$(
     		$(#[$field_meta:meta])?
     		$var:ident:$var_type:ty $(:$fmt:expr)?,
     	)*
    }, {
    	$(
     		$builder_var:ident: $builder_var_msg:literal: $builder_var_func:expr,
     	)*
    }
    $(,{
    	$(
     		$combined_var:ident $(:$disallowed:ident)+,
     	)+
    })*) => {
        pastey::paste! {
            #[derive(Debug, Default, Builder)]
            #[builder(build_fn(validate = "Self::validate"))]
            $(#[$query_meta])?
            pub struct [<$name:upper_camel Query>] {
            	$(
             		$(#[$field_meta])?
             		#[builder(setter(into, strip_option), default)]
                    pub $var: Option<$var_type>,
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
                 	$(
                  		$(
                    		if self.$combined_var.is_some() {
                      			$(
	                      			if self.$disallowed.is_some() {
	                      				return Err(format!("{} cannot be combined with {}", stringify!($combined_var), stringify!($disallowed)));
	                         		}
                         		)+
                      		}
                    	)+
                  	)*
                 	Ok(())
             	}
            }
            impl std::fmt::Display for [<$name:upper_camel Query>] {
            	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
             		let mut query = vec![];
               		$(
                 		if let Some(v) = self.$var.as_ref() {
                 			query.push(query!(@format $var, v, $($fmt)?));
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

    (@format $var:ident, $field:expr, $fmt:expr) => {
    	$fmt(stringify!($var), $field)
    };
    (@format $var:ident, $field:expr,) => {
    	format!("{}={}", stringify!($var), $field)
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
    ($(#[$doc_meta:meta])? $name:ident {
    	$(
    		$(#[$field_meta:meta])?
     		$error:ident:$error_type:ty,
     	)+
    }) => {
        pastey::paste! {
        	#[derive(Debug)]
         	$(#[$doc_meta])?
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

/// Connects a person (ficitonal or real) to a type of media
#[macro_export]
macro_rules! person_connection {
    ($media:ident, $type: ident, $connection_name: ident) => {
        pastey::paste! {
            #[doc = "Connects a(n) `" $type "` to a(n) `" $media "` and what `" $connection_name "` they are"]
            #[derive(Debug, Serialize, Deserialize)]
            #[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
            pub struct [<$media:upper_camel $type:upper_camel Connection>] {
            	#[doc = "How this person connections to the `" $media "`"]
                $connection_name: String,
            	#[doc = "Basic information on the `" $media "` they are in."]
                $media: Entry,
            }
        }
    };
}
