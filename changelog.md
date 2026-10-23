# Changelog

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
