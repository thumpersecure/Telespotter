use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct GoogleApiResponse {
    items: Option<Vec<GoogleSearchItem>>,
}

#[derive(Debug, Deserialize)]
struct GoogleSearchItem {
    title: String,
    snippet: Option<String>,
}

/// Search using Google Custom Search API if credentials are available,
/// otherwise fall back to web scraping
#[allow(dead_code)]
pub async fn search(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    search_with_config(query, num_results, &SearchConfig::default()).await
}

/// Search with custom configuration
pub async fn search_with_config(query: &str, num_results: usize, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    // Check for API credentials in environment variables
    let api_key = env::var("GOOGLE_API_KEY").ok();
    let search_engine_id = env::var("GOOGLE_SEARCH_ENGINE_ID").ok();

    match (api_key, search_engine_id) {
        (Some(key), Some(cx)) => {
            // Use official API if credentials are available
            search_with_api_config(query, num_results, &key, &cx, config).await
        }
        _ => {
            // Fall back to web scraping
            search_with_scraping_config(query, num_results, config).await
        }
    }
}

/// Search using Google Custom Search API
#[allow(dead_code)]
async fn search_with_api(
    query: &str,
    num_results: usize,
    api_key: &str,
    cx: &str,
) -> Result<Vec<SearchResult>> {
    search_with_api_config(query, num_results, api_key, cx, &SearchConfig::default()).await
}

/// Search using Google Custom Search API with config
async fn search_with_api_config(
    query: &str,
    num_results: usize,
    api_key: &str,
    cx: &str,
    config: &SearchConfig,
) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);
    // Wrap query in quotes for exact phrase matching
    let quoted_query = format!("\"{}\"", query);
    let encoded_query = urlencoding::encode(&quoted_query);

    // Google Custom Search API endpoint
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num={}",
        api_key, cx, encoded_query, num_results.min(10) // API max is 10 per request
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Google API error: {}", response.status()));
    }

    let api_response: GoogleApiResponse = response.json().await?;

    let results = api_response
        .items
        .unwrap_or_default()
        .into_iter()
        .map(|item| {
            SearchResult::new(
                item.title,
                item.snippet.unwrap_or_default(),
                "Google".to_string(),
            )
        })
        .collect();

    Ok(results)
}

/// Search using web scraping (fallback method)
#[allow(dead_code)]
async fn search_with_scraping(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    search_with_scraping_config(query, num_results, &SearchConfig::default()).await
}

/// Search using web scraping with config
async fn search_with_scraping_config(query: &str, num_results: usize, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);
    // Wrap query in quotes for exact phrase matching
    let quoted_query = format!("\"{}\"", query);
    let encoded_query = urlencoding::encode(&quoted_query);
    let url = format!(
        "https://www.google.com/search?q={}&num={}",
        encoded_query, num_results
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Google scraping error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // Selector for main search result divs
    let result_selector = Selector::parse("div.g").unwrap();
    let title_selector = Selector::parse("h3").unwrap();
    let snippet_selectors = vec![
        Selector::parse("div.VwiC3b").unwrap(),
        Selector::parse("div.yXK7lf").unwrap(),
    ];

    for element in document.select(&result_selector) {
        let title = element
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let mut snippet = String::new();
        for selector in &snippet_selectors {
            if let Some(s) = element.select(selector).next() {
                snippet = s.text().collect::<String>();
                break;
            }
        }

        if !title.is_empty() || !snippet.is_empty() {
            results.push(SearchResult::new(
                title,
                snippet,
                "Google".to_string(),
            ));
        }
    }

    Ok(results)
}
