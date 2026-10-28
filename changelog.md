# Changelog
# Unreleased
- **Changed** All non-query parameters to be borrowed *`&`* instead of owned.

# 1.1.2
 - **Fixed** `AnimeSource` not having a ` ` (empty string) option. *(Defaults to `AnimeSource.Unknown`)*
 - **Fixed** `FullRecommendationInfo.content` breaking due to a `null` value.

# 1.1.1
*And this is why we have tests*
- **Fixed** `AnimeSource` not having a `radio` option.

# 1.1.0
Small changes, but a lot of them. Ran the same test a couple hundred times to make sure this update didn't have issues.
- **Added** `API Documentation: <link>` to every endpoint with a valid documentation.
- **Enum-ified** `Relation.relation`. 
- **Enum-ified** `Anime.source`. 
- **Fixed** `Relation.media_type` being null. 
- **Fixed** edge cases in `Anime.theme` Regex parsing *hopefully should have caught them all*
- **Fixed** `rate` feature just not working. *I messed up maths*
- **Fixed** a couple of unresolved links to crate members.

# 1.0.3
I realised i could actually do this now.
- **Added** local client-sided server key validation.

# 1.0.2
Minor update to clean things up
- **Added** github automated testing
- **Changed** `#[serde(deny_unknown_fields)]` is now an optional feature behind `strict`
- **Changed** how `changelog` is formatted
- **Fixed** changelog link from crates.io *(hopefully)*
- **Fixed** "edge" case where [`Articles.author_url`](https://docs.rs/tenrai/latest/tenrai/schemas/article/struct.Article.html#structfield.author_url) is `null`

# 1.0.1
Some markdown fixes

- **Added** Even more basic documentation for `impl`s. *because apparently thats a thing*
- **Fixed** Markdown rendering in [Readme](./Readme.md)

## 1.0.0
Initial release
