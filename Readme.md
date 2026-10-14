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
<details>
  <summary>rate</summary>
  Implements a local client-based rate limiter. Do note this can't take into account IP-based or Key-based limits.
  Due to not being able to account for other clients, this is an opt-in feature.

  The local limit applied depends on if a server key is provided or not.
  - Without a server-key: One request every 500ms (2/s)
  - With a server-key: One request every 200ms (5/s)

  Wilst technically we can go up to 4/s w/o a server key. The limit is at 2/s to avoid hitting the 120/m limit.
  Server keys don't have this restriction as they can do 300/min which is in-line with the 5/s limit.

  NOTE: The rate feature does not guarantee that a limit will never be hit. Especially with multiple clients using the same IP/Server-key.
  It is here to reduce the chance of this one client hitting the limit.
</details>

# Documentation
Everything has been documentated in line with the [official API documentation](https://api.tenrai.org/documentation)
However in some cases, documentation will be slightly reworded to make more sense or to explain more that the official one lacks.
