use reqwest::{Client, header::HeaderMap};

pub struct TenariAPI {
    server_key: Option<String>,
    client: Client,
}

impl TenariAPI {
    /// Create a new instance of the API.
    ///
    /// The server key is used for higher rate limits if you have supported on patreon.
    /// See more https://api.tenrai.org/documentation#description/server-keys
    pub fn new(server_key: Option<String>) -> Self {
        Self {
            server_key,
            client: Client::builder()
                .user_agent(format!(
                    "Tenrai Rust Client - {}",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .unwrap(),
        }
    }

    /// build the required headers.
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/json".parse().unwrap());
        if let Some(key) = self.server_key.as_ref() {
            headers.insert("X-Server-Key", key.parse().unwrap());
        }
        headers
    }
}
