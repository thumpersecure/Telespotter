use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// List of common user agents for rotation
const USER_AGENTS: &[&str] = &[
    // Chrome on Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36",
    // Chrome on macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    // Firefox on Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
    // Firefox on macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
    // Safari on macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    // Edge on Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
    // Chrome on Linux
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    // Firefox on Linux
    "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
];

/// Get a random user agent
pub fn get_random_user_agent() -> &'static str {
    USER_AGENTS.choose(&mut rand::thread_rng()).unwrap_or(&USER_AGENTS[0])
}

/// Get the default user agent
pub fn get_default_user_agent() -> &'static str {
    USER_AGENTS[0]
}

/// Configuration for search requests
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub timeout_secs: u64,
    pub random_user_agent: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            timeout_secs: 10,
            random_user_agent: false,
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
    create_client_with_timeout(10, false)
}

/// Create HTTP client with custom timeout and optional random user agent
pub fn create_client_with_timeout(timeout_secs: u64, random_ua: bool) -> reqwest::Client {
    let user_agent = if random_ua {
        get_random_user_agent()
    } else {
        get_default_user_agent()
    };

    reqwest::Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(timeout_secs))
        .pool_max_idle_per_host(5)
        .pool_idle_timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

/// Create HTTP client from config
pub fn create_client_from_config(config: &SearchConfig) -> reqwest::Client {
    create_client_with_timeout(config.timeout_secs, config.random_user_agent)
}
