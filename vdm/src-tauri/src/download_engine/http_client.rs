use reqwest::Client;
use std::time::Duration;

pub struct HttpClient;

impl HttpClient {
    pub fn build() -> Result<Client, String> {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .tcp_keepalive(Duration::from_secs(60))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(32)
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))
    }
}
