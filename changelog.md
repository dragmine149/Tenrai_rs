# Changelog
Format is `{Crate}(API)` where both are their represetive versions. If `(API)` does not exist, then it's the same API version as the previous crate version.

## 1.3.0 (v1.0.{ver}) **UNRELEASED**
Skipped many a version because of the query update. Hopefully smoother sailing from here.
This is also a big update, kinda on the scale of [v1.0.0](<changelog#1.0.0 (v1.0.24)>)

- **Updated API** to `v1.0.{insert ver here}`
	- See [API Changelog](https://api.tenrai.org/changelog) for more details.
- **Updated** a bunch of documentation.
	- **Added** testing for documentation.
- **Updated** dependencies.
	- Nothing actually needed updating, however i have reduced the pinning back to `X.Y` instead of `X.Y.Z` for all of them.
	- Also added comments so those reading it know which dependencies are for what.
- **Changed** types from `u64` to `u32` to better align with the API.
	- Still use `f64` though as those are rust defaults. (*we just check for 2dp in most cases*)
	- Note: You can still do stuff `u32::MAX` as the API only accepts up to `i32::MAX`. *i use `u32::MAX` as i don't want to deal with negatives*
	- *There was like 339+ replacements...*
- **Added** a mixture of `PartialEq, Eq, PartialOrd, Ord` to every enum to further align with [c-common-traits](https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits)
	- *Also removed some from structs whilst i work out what to do*
- `Date`:
	- **Added** `new`, `From<&str>`, `From<String>` methods
	- **Added** `to_full_string` method as an alternative for `fmt` methods.
	- **Changed** `Date` types to be more fitting of their true representation. (*you don't need a u32 for a u4*)
- **Added** `From<bool>` for `OnlyBool` (*you're not getting a reverse bc of `Only`*)
- URL now gets **logged** via `log::debug` on every request. (*see log feature*)
- **Added** `producer_*` aliases to `company_*` endpoint methods.
- **Publicisied** `TenraiAPI.server_key` (*no reason to have it private*)
- **Added** `#[must_use]` to `*QueryBuilder::builder` methods.
- **Added** `From<reqwest::Client> for TenraiAPI`
- **Added** *developer focused* `test` feature.
- **Added** `db_*` aliases to `_ids` functions.
	- *Also moved them to a separate `impl`*
- **Renamed:**
	- `ProducerQuery*` to `CompanyQuery*` (and aliased)
	- `ProducerOrder` to `CompanyOrder` (and aliased)
	- `top_character` to `top_characters`
	- `TypeSfwPageQuery` to `TopRecommendationsQuery` (not aliased as it makes more sense this way.)
- **Added** `new` function to `*QueryBuilder`, which is just a reference to the `default` function.
- **Changed** `RegionalRestrictions` from a struct into an enum.

### Query related stuff
- **Updated** How handle of queries is done internally.
	- Nothing changes much on the frontend, apart from having to additionally import some new traits
- **Updated** a bunch of queries to have new arguments in line with the new API updates.
- **Added** `From<*Query> for *QueryBuilder` 
- **Added** Ability for queries to require other query parameters. *Yes the API does this, but eh.*
- **Fixed** a bunch of issues with the old query system whilst updating it.
	- `usernames` being set to limit of 300 chars instead of 30
	- *And probably many more*
- **Changed** All query fields to be private. In return, getter methods are provided.
- Added basics query tests.
	- *these are really limited, however what they do test should guarantee it works. Full query tests are impossible.*

### More specific struct changes that aren't generalised
- **Privatised** `Anime.explicit_genres` due to being always empty.
- **Renamed** `Anime.theme` to `Anime.theme_songs` to make the difference between that and `Anime.themes` more obvious.
	- *It still reads from `theme` though*

## 1.2.0 (v1.0.25)
First API update and some other small fixes.

I am thinking of changing how the query is handled, but don't know what will happen to it yet. Query args will be revalidated *and tested* then. For now, please open an issue for any issues with them.
- **Updated** to `v1.0.25`, See [API Changelog](https://api.tenrai.org/changelog) for more details.
	- **Added** `top_episodes`, `top_recommendations`
	- **Updated** Various queries and return structs to reflect the new changes. *once again, see the API changelog for more details*
		- **Updated** `random_*`/`*_recommendations` endpoints to use more detailed queries instead of `SfwQuery`
	- **Removed** `TopQuery` in favour of `TopMangaQuery` and `TopAnimeQuery`
- **Added** local checking for when two query variables cannot be used together. 
- **Exposed** Caching `age` so values can be used other than the default of `1 day`
- **FIXED** `News` and `Articles` not being the same thing. 
- **Fixed** Changelog formatting *(i was using `#` instead of `##`)*
- **Fixed** `sfw-strict` having an extra `?`. *idk if this actually affected stuff, but its fixed now*
- **Fixed** Various incorrect `query` documentation due to copy-paste. *Theres probably still issues somewhere*

## 1.1.3
- **Changed** All non-query parameters to be borrowed *`&`* instead of owned.
	- *Unsure yet to change the query parameters as well.*

## 1.1.2
 - **Fixed** `AnimeSource` not having a ` ` (empty string) option. *(Defaults to `AnimeSource.Unknown`)*
 - **Fixed** `FullRecommendationInfo.content` breaking due to a `null` value.

## 1.1.1
*And this is why we have tests*
- **Fixed** `AnimeSource` not having a `radio` option.

## 1.1.0
Small changes, but a lot of them. Ran the same test a couple hundred times to make sure this update didn't have issues.
- **Added** `API Documentation: <link>` to every endpoint with a valid documentation.
- **Enum-ified** `Relation.relation`. 
- **Enum-ified** `Anime.source`. 
- **Fixed** `Relation.media_type` being null. 
- **Fixed** edge cases in `Anime.theme` Regex parsing *hopefully should have caught them all*
- **Fixed** `rate` feature just not working. *I messed up maths*
- **Fixed** a couple of unresolved links to crate members.

## 1.0.3
I realised i could actually do this now.
- **Added** local client-sided server key validation.

## 1.0.2
Minor update to clean things up
- **Added** github automated testing
- **Changed** `#[serde(deny_unknown_fields)]` is now an optional feature behind `strict`
- **Changed** how `changelog` is formatted
- **Fixed** changelog link from crates.io *(hopefully)*
- **Fixed** "edge" case where [`Articles.author_url`](https://docs.rs/tenrai/latest/tenrai/schemas/article/struct.Article.html#structfield.author_url) is `null`

## 1.0.1
Some markdown fixes

- **Added** Even more basic documentation for `impl`s. *because apparently thats a thing*
- **Fixed** Markdown rendering in [Readme](./Readme.md)

## 1.0.0 (v1.0.24)
Initial release
