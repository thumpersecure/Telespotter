use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for search requests
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub timeout_secs: u64,
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            timeout_secs: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub snippet: String,
    pub source: String,
}

impl SearchResult {
    pub fn new(title: String, snippet: String, source: String) -> Self {
        SearchResult {
            title,
            snippet,
            source,
        }
    }
}

#[allow(dead_code)]
pub trait SearchEngine {
    async fn search(query: &str, num_results: usize) -> anyhow::Result<Vec<SearchResult>>;
}

/// Create HTTP client with default timeout
#[allow(dead_code)]
pub fn create_client() -> reqwest::Client {
    create_client_with_timeout(10)
}

/// Create HTTP client with custom timeout
pub fn create_client_with_timeout(timeout_secs: u64) -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(timeout_secs))
        .pool_max_idle_per_host(5)
        .pool_idle_timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

/// Create HTTP client from config
pub fn create_client_from_config(config: &SearchConfig) -> reqwest::Client {
    create_client_with_timeout(config.timeout_secs)
}
