use crate::search::{create_client, SearchResult};
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
pub async fn search(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    // Check for API credentials in environment variables
    let api_key = env::var("GOOGLE_API_KEY").ok();
    let search_engine_id = env::var("GOOGLE_SEARCH_ENGINE_ID").ok();

    match (api_key, search_engine_id) {
        (Some(key), Some(cx)) => {
            // Use official API if credentials are available
            search_with_api(query, num_results, &key, &cx).await
        }
        _ => {
            // Fall back to web scraping
            search_with_scraping(query, num_results).await
        }
    }
}

/// Search using Google Custom Search API
async fn search_with_api(
    query: &str,
    num_results: usize,
    api_key: &str,
    cx: &str,
) -> Result<Vec<SearchResult>> {
    let client = create_client();
    let encoded_query = urlencoding::encode(query);
    
    // Google Custom Search API endpoint
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num={}",
        api_key, cx, encoded_query, num_results.min(10) // API max is 10 per request
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        eprintln!("Google API error: {}", response.status());
        return Ok(Vec::new());
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
async fn search_with_scraping(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    let client = create_client();
    let encoded_query = urlencoding::encode(query);
    let url = format!(
        "https://www.google.com/search?q={}&num={}",
        encoded_query, num_results
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Ok(Vec::new());
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
