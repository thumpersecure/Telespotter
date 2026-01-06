use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search Bing with default configuration
#[allow(dead_code)]
pub async fn search(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    search_with_config(query, num_results, &SearchConfig::default()).await
}

/// Search Bing with custom configuration
pub async fn search_with_config(query: &str, num_results: usize, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);
    let encoded_query = urlencoding::encode(query);
    let url = format!(
        "https://www.bing.com/search?q={}&count={}",
        encoded_query, num_results
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Bing search error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // Selector for Bing search results
    let result_selector = Selector::parse("li.b_algo").unwrap();
    let title_selector = Selector::parse("h2").unwrap();
    let snippet_selector = Selector::parse("p").unwrap();

    for element in document.select(&result_selector) {
        let title = element
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let snippet = element
            .select(&snippet_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        if !title.is_empty() || !snippet.is_empty() {
            results.push(SearchResult::new(
                title,
                snippet,
                "Bing".to_string(),
            ));
        }
    }

    Ok(results)
}
