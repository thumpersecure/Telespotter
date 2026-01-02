use serde::{Deserialize, Serialize};

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

pub trait SearchEngine {
    async fn search(query: &str, num_results: usize) -> anyhow::Result<Vec<SearchResult>>;
}

pub fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}
