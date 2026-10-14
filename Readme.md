# Tenrai API
A rust wrapper for the [Tenrai API](https://tenrai.org/)

# Versioning
Major -> Same as Tenrai API version
Minor / Patch -> Whatever i fell like.

# Features
<details>
  <summary>auto_retry (default)</summary>
  Automatically retry upon certain errors. This will delay the response but make it more likely an OK response is returned.

  Currently, the following status codes are auto-retried:
  - 429 (rate-limited)
</details>
<details>
  <summary>custom</summary>
  The custom feature should rarely be required. It exposes a function allowing direct access to the request function without any verification checks.

  The only checks performed are those when making the request itself.
</details>
<details>
  <summary>log</summary>
  Use the [log crate](https://crates.io/crates/log) to print additional output. 

  This is used in cases like auto-retry, to output more information that isn't directly returned by the function.
  You probably won't get much use out of this other than debugging related.
</details>

# Documentation
Everything has been documentated in line with the [official API documentation](https://api.tenrai.org/documentation)
However in some cases, documentation will be slightly reworded to make more sense or to explain more that the official one lacks.
