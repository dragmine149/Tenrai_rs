# Tests
The point of these tests is to make sure the parser is working correctly. Whilst i have full trust in [serde](https://serde.rs), there could be a numberious amount of issues with our types.

Each test will ping their specified endpoint for a random entry and parse the data. Outside of that, no additional checks are preformed.

## Random Entry
As much as tests are made to be deterministic, the random entry will allow for quicker finding of where errors occur as it could be the smallest of things in a very obscure place.

This does mean we can't guarantee 100% that nothing will fail, but for most use cases you shouldn't have to worry about it.

Additionally, when running tests. Expect them to take ~1+ minute every 24 tests. This is due to how many retries need to be done because of invalid entries.

## Queries
We are not testing the backend of the database, hence any endpoints with queries will be provided the query which gives the most data.

Queries themselves will have basic tests to make sure formatting works as expected due to the nature of how we have to send them. But as all queries are macros, only a few will be tested.
