# Changelog
Format is `{Crate}(API)` where both are their represetive versions. If `(API)` does not exist, then it's the same API version as the previous crate version.

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
