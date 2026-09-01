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
pub fn create_client() -> anyhow::Result<reqwest::Client> {
    create_client_with_timeout(10, false)
}

/// Create HTTP client with custom timeout and optional random user agent
pub fn create_client_with_timeout(timeout_secs: u64, random_ua: bool) -> anyhow::Result<reqwest::Client> {
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
        .map_err(|e| anyhow::anyhow!("Failed to build HTTP client: {}", e))
}

/// Create HTTP client from config
pub fn create_client_from_config(config: &SearchConfig) -> anyhow::Result<reqwest::Client> {
    create_client_with_timeout(config.timeout_secs, config.random_user_agent)
}

/// Detect common block / CAPTCHA / consent-wall pages that return HTTP 200 but
/// contain no real results. Returns true if the body looks like a block page.
pub fn is_blocked_page(body: &str) -> bool {
    let lower = body.to_lowercase();
    const INDICATORS: &[&str] = &[
        "captcha",
        "unusual traffic",
        "detected unusual activity",
        "consent.google",
        "/sorry/",
        "are you a robot",
        "verify you are a human",
        "please verify you're a human",
        "our systems have detected",
        "automated queries",
        "enablejs",
        // DuckDuckGo anti-bot challenge (served with HTTP 202 to hosting IPs)
        "anomaly.js",
        "anomaly-modal",
        "challenge-form",
    ];
    INDICATORS.iter().any(|i| lower.contains(i))
}

/// Error returned when an engine appears to have served a block/CAPTCHA page.
#[derive(Debug)]
pub struct BlockedError {
    pub engine: String,
}

impl std::fmt::Display for BlockedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} appears to have blocked the request (CAPTCHA / consent / unusual-traffic page)", self.engine)
    }
}

impl std::error::Error for BlockedError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_blocked_page_detects_indicators() {
        assert!(is_blocked_page("Please complete the CAPTCHA to continue"));
        assert!(is_blocked_page("Our systems have detected unusual traffic"));
        assert!(is_blocked_page("<form id=\"challenge-form\" action=\"//duckduckgo.com/anomaly.js\">"));
    }

    #[test]
    fn test_is_blocked_page_allows_normal_content() {
        assert!(!is_blocked_page("<div class=\"g\"><h3>John Doe - 555-1234</h3></div>"));
        assert!(!is_blocked_page("No results found for that query"));
    }

    #[test]
    fn test_create_client_builds_ok() {
        assert!(create_client_with_timeout(10, false).is_ok());
        assert!(create_client_with_timeout(5, true).is_ok());
    }
}
