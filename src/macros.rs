/// A macro to make a generic type. There are many different types of the same information, so this consolidates them all.
///
/// This also helps us with the `type` parameter as `type` is a keyword in rust.
#[macro_export]
macro_rules! generic_type {
    ($name: tt) => {
        pastey::paste! {
           #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
           #[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
            pub struct [<$name:upper_camel Type>] {
                /// The unique MAL ID of the catalog item
                pub mal_id: u32,
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
/// query!(name { DataStruct }, { ValidationStruct }, { ExclusiveStruct })
/// ```
/// ## DataStruct
/// For the most part, this is same as usual...
/// ```ignore
/// /// (optional) some comment
/// some_field:some_type
/// ```
/// However, it has been expanded to support the following
/// ```ignore
/// /// (optional) some comment
/// *custom:some_field:some_type:format_func
/// ```
/// ### `*custom`
/// This marks the field as a custom field, and assumes the field is a query instead of a typical field.
/// The benefits of this means you can use the [query_extension] macro and formatting the field won't be as bad as `?some_field=?some_field=some_value`
///
/// ### `format_func`
/// This allows for custom formatting that can't be done via `Display` without having to make a struct just for implementing `Display`.
/// For example:
/// ```ignore
/// query: String:(|_, v| format!("q={v}")),
/// // ofc, probably split it out.
/// genres: Vec<u32>:(|n, v| format!("{n}={}", v.iter().map(|s| format!("{s}")).collect::<Vec<String>>().join(","))),
/// // because this is possible (where `query_vec` is just the name of a function that can be accessed)
/// genres: Vec<u32>:query_vec,
/// ```
/// The inputs are:
/// - variable name (String)
/// - variable value (type of variable as defined)
///
/// ## ValidationStruct
/// This struct adds extra conditions when using the `Builder`. A build will fail if not all conditions are met. Conditions are formatted as such
/// ```ignore
/// some_var:some_reason:some_condition
/// // example
/// query: "Query length is too long": (|q: &String| q.len() > 300),
/// page: "Page must be less than 20k": (|p: &u32| *p > 20000),
/// rating: "Rating can not be None": (|r: &[AgeRating]| r.contains(&AgeRating::None)),
/// ```
/// - `some_var` must be the same as one of the members of the `DataStruct`
/// - `some_reason` must be a string literal of the error. This is returned in as `E` in `Result`
/// - `some_func` is the function to test. You get passed the value, and the type is a reference to the type specified.
///
/// Additionally, after the above, it is possible to add the following `/key/var:condition` This is a shorthand for the above, as such can't be as dynamic.
/// `key` is a specific key of one of the following
/// - v:  Checks to see if any item in `Vec<T>` is `< var`
/// - vl: Checks to see if the length of `Vec<T>` is > var`
/// - i:  Checks to see if the value is > var
/// - il: Checks to see if the value is < var
///	For any of the above condition, if the condition returns true an error is caused.
///
/// `var` is the variable in question, whilst `condition` depends on the key.
///
/// ## ExclusiveStruct
/// Finally, as an extension to `ValidationStruct`. This struct compares and makes sure `some_val_a` can not be enabled when `some_val_b` is also enabled.
/// ```ignore
/// {
///     some_field_a:some_field_b:some_field_c:some_field_d,
///     some_field_d:some_field_e,
///     some_field_f:some_field_g
/// }
/// ```
/// This says, that as long as one of `a/b/c/d` is enabled. The rest of them cannot be. It also allows stuff like `a/e` to be enabled whilst `a/d` and `d/e` are disallowed. (enabling `d` would prevent `a` and `e`)
///
/// Due to being an extension, it's completely optional and can just not be provided. If provided though, it must have at least one exclusive value *just to make it worth while providing it*
///
/// # Derives
/// All types must implement `Debug, Display, Default, Clone`. *rust gets annoyed otherwise*
#[macro_export]
macro_rules! query {
    ($(#[$query_meta:meta])? $name:ident {
    	$(
     		$(#[$field_meta:meta])?
       		$(*$custom:ident:)?
     		$var:ident:$var_type:ty $(:$fmt:expr)?,
     	)*
    }, {
    	$($builder_var:ident: $builder_var_msg:literal: $builder_var_func:expr,)*
      	$(/$tt:tt/$builder_var2:ident:$builder_condition:literal,)*
    }
    $(,{
    	$($combined_var:ident $(:$disallowed:ident)+,)*
     	$(/$required_var:ident $(:$requires:ident)+,)*
    })*) => {
        pastey::paste! {
        	// deals with the main struct. This is what contains the data after everything.
            #[derive(Debug, Default, Builder, Clone)]
            #[builder(build_fn(validate = "Self::validate"))]
            $(#[$query_meta])?
            pub struct [<$name:upper_camel Query>] {
            	$(
             		$(#[$field_meta])?
             		#[builder(setter(into, strip_option $(,$custom)?), default)]
                    $var: Option<$var_type>,
                )*
            }
            #[doc = "Helper functions for interacting with " [<$name:upper_camel Query>]]
            impl [<$name:upper_camel Query>] {
            	/// Create a new builder object
             	#[must_use = "Finish the builder with `.build()`"]
            	pub fn builder() -> [<$name:upper_camel QueryBuilder>] {
             		[<$name:upper_camel QueryBuilder>]::default()
             	}
              	$(
                 	$(#[$field_meta])?
               		pub fn $var(&self) -> &Option<$var_type> {
                 		return &self.$var;
                 	}
               	)*
            }
            /// Allow converting from query to builder
            ///
            /// Note: If the value in the query struct is [None], then it will get translated to [None] here instead of `Some(None)`
            impl From<[<$name:upper_camel Query>]> for [<$name:upper_camel QueryBuilder>] {
            	fn from(value: [<$name:upper_camel Query>]) -> Self {
             		Self {
               			$(
                  			$var: value.$var.is_some().then(|| value.$var),
                  		)*
               		}
             	}
            }

            /// Custom validation function.
            impl [<$name:upper_camel QueryBuilder>] {
            	fn validate(&self) -> Result<(), String> {
             		// Check to see if the ValidationStruct is fine
             		$(
               			if let Some(ref v) = self.$builder_var && let Some(v) = v && $builder_var_func(v) {
                     		return Err($builder_var_msg.to_string());
                     	}
               		)*
                 	// Custom validation which reduces a lot of reputition.
                 	$(
                  		if let Some(ref v) = self.$builder_var2 && let Some(v) = v
                    		&& query!(@condition /$tt/ v, $builder_condition)
                      	{
                    		return Err(query!(@condition_reason /$tt/ $builder_var2, $builder_condition))
                    	}
                  	)*
                 	// Then check the exclusive struct
                 	$(
                  		$(
                    		if self.$combined_var.is_some() {
                      			$(
	                      			if self.$disallowed.is_some() {
	                      				return Err(format!("{} cannot be combined with {}", stringify!($combined_var), stringify!($disallowed)));
	                         		}
                         		)+
                      		}
                    	)*
                  	)*
                   	// and finnally the required.
                    $(
                    	$(
                     		if self.$required_var.is_some() {
                       			$(
                          			if self.$requires.is_none() {
                             			return Err(format!("{} requires {}", stringify!($required_var), stringify!($requires)));
                             		}
                          		)+
                       		}
                     	)*
                    )*
                    $(
                    	$(
                    		query!(@validate $custom [<$var:upper_camel QueryHelper>])(&self).map_err(|e| e.to_string())?;
                     	)?
                    )*
                 	Ok(())
             	}
            }
            /// Converts the struct into `?some_var=a&some_foo=b&some_bar=c`.
            impl std::fmt::Display for [<$name:upper_camel Query>] {
            	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
             		let mut query = vec![];
               		$(
                 		if let Some(v) = self.$var.as_ref() {
                 			query.push(query!(@format $($custom,)? $var, v, $($fmt)?));
                   		}
                 	)*

                  	// this is useful because then we don't have to worry about getting rid of trailing `&`s
                  	if query.len() > 0 {
                   		write!(f, "?{}", query.join("&"))
                   	} else {
                    	write!(f, "")
                    }
             	}
            }

            // see query_extension
            #[doc="Helper trait for adding " $name " to another query\n\nThis is a sealed trait and cannot be implemented."]
            #[sealed::sealed]
            pub trait [<$name:upper_camel QueryHelper>] {
            	$(
             		$(#[$field_meta])?
             		fn $var<VALUE>(&mut self, value: VALUE) -> &mut Self
               		where
                 		VALUE: Into<$var_type>;
             	)*

              	/// Test to see if the custom builder is fine or not.
                ///
                /// The default implementation is to return OK so that nothing breaks.
              	fn validate_helper(&self) -> Result<(), [<$name:upper_camel QueryBuilderError>]> { Ok(()) }
            }
        }
    };

    // format extensions of `Display`, depending on the input.
    (@format $var:ident, $field:expr, $fmt:expr) => {
    	$fmt(stringify!($var), $field)
    };
    (@format $var:ident, $field:expr,) => {
    	format!("{}={}", stringify!($var), $field)
    };
    (@format custom, $var:ident, $field:expr,) => {
    	(|| {
     		let original = format!("{}", $field);
      		original.strip_prefix("?").map(|s| s.to_string()).unwrap_or(original)
     	})()
    };

    // if any of these conditions are true. Then it causes an error.
    (@condition /v/ $value:expr, $condition:expr) => {
    	(|| $value.iter().any(|v| *v < $condition))()
    };
    (@condition /vl/ $value:expr, $condition:expr) => {
    	(|| $value.len() > $condition)()
    };
    (@condition /i/ $value:expr, $condition:expr) => {
    	(|| *$value > $condition)()
    };
    (@condition /il/ $value:expr, $condition:expr) => {
    	(|| *$value < $condition)()
    };
    (@condition_reason /v/ $v:ident, $condition:expr) => {
    	format!("All values of \"{}\" must be greater than \"{}\"", stringify!($v), $condition)
    };
    (@condition_reason /vl/ $v:ident, $condition:expr) => {
    	format!("Can only have a max of {} {}(s)", $condition, stringify!($v))
    };
    (@condition_reason /i/ $v:ident, $condition:expr) => {
    	format!("{} must be less than {}", stringify!($v), $condition)
    };
    (@condition_reason /il/ $v:ident, $condition:expr) => {
    	format!("{} must be greater than {}", stringify!($v), $condition)
    };

    (@validate custom $helper:ident) => {
    	<Self as $helper>::validate_helper
    };
}

/// Extension macro to [query] allowing for things that [query] can't do on its own.
///
/// This involves implementing the helper trait where specified.
///
/// # Usage
/// ```
/// query_extension!(Anime, Sfw, {
///     sfw: bool,
///     sfw_strict: bool,
///	});
///	```
/// where `Anime` is the `Query` struct and `Sfw` is the `Helper` trait.
/// Due to limitations in rust, you will have to provided all the fields and types again. However, the amount saved is worth it.
///
/// Additionally, the following helper traits have their own branch, hence the whole `{ /* ... */ }` is useless.
/// - Sfw
#[macro_export]
macro_rules! query_extension {
	($query: ident, Sfw) => {
		query_extension!($query, Sfw, {
    		sfw: bool,
      		sfw_strict: bool,
		});
	};
	($query: ident, Ids) => {
		query_extension!($query, Ids, {
			min_id: u32,
			max_id: u32,
			ids: Vec<u32>,
			exclude_ids: Vec<u32>,
		});
	};
	($query:ident, Score) => {
		query_extension!($query, Score, {
			score:f64,
			min_score:f64,
			max_score:f64,
			min_scored_by:u32,
			max_scored_by:u32,
		});
	};
	($query:ident, Year) => {
		query_extension!($query, Year, {
			year: Vec<u32>,
			min_year: u32,
			max_year: u32,
		});
	};
	($query:ident, Page) => {
		query_extension!($query, Page, {
			page: u32,
			limit: u32,
		});
	};
	($query:ident, BasicPage) => {
		query_extension!($query, BasicPage, {
			page: u32,
		});
	};

    ($query: ident, $helper: ident, {$($value:ident:$var_type:ty,)*}) => {
        pastey::paste! {
        	// impl sealed as it's a sealed trait.
        	#[sealed::sealed]
         	impl [<$helper:upper_camel QueryHelper>] for [<$query:upper_camel QueryBuilder>] {
          		$(
            		// for all functions, reimplement it.
            		fn $value<VALUE>(&mut self, value: VALUE) -> &mut Self
              		where
                		VALUE: Into<$var_type>
                  	{
                   		// if it doesn't exist, make it.
                  		self.[<$helper:snake>]
                    		.get_or_insert_with(|| Some([<$helper:upper_camel Query>]::default()))
                      		.get_or_insert_default()
                        	// then edit it.
                        	.$value = Some(value.into());
                        self
                  	}
            	)*

             	fn validate_helper(&self) -> Result<(), [<$helper:upper_camel QueryBuilderError>]> {
             		[<$helper:upper_camel QueryBuilder>]::from(self.[<$helper:snake>]
              			.clone()
                		.unwrap_or_default()
                  		.unwrap_or_default()
              		).build()?;
                	Ok(())
              	}
          	}
        }
    };
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
