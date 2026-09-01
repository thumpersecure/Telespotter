use crate::search::{is_blocked_page, BlockedError, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search DuckDuckGo with default configuration
#[allow(dead_code)]
pub async fn search(query: &str, num_results: usize, client: &reqwest::Client) -> Result<Vec<SearchResult>> {
    search_with_config(query, num_results, client).await
}

/// Search DuckDuckGo with a shared client.
/// Uses the lite endpoint (lite.duckduckgo.com/lite/), which is far more
/// scrape-stable than the html.duckduckgo.com layout.
pub async fn search_with_config(query: &str, num_results: usize, client: &reqwest::Client) -> Result<Vec<SearchResult>> {
    // Wrap query in quotes for exact phrase matching
    let quoted_query = format!("\"{}\"", query);
    let encoded_query = urlencoding::encode(&quoted_query);
    let url = format!("https://lite.duckduckgo.com/lite/?q={}", encoded_query);

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("DuckDuckGo search error: {}", response.status()));
    }

    let html = response.text().await?;

    if is_blocked_page(&html) {
        return Err(BlockedError { engine: "DuckDuckGo".to_string() }.into());
    }

    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // The lite layout renders results in a table: titles are <a class="result-link">
    // and snippets are <td class="result-snippet">. Fall back to older classes too.
    let title_selector = Selector::parse("a.result-link, a.result__a").unwrap();
    let snippet_selector = Selector::parse("td.result-snippet, a.result__snippet").unwrap();

    let titles: Vec<String> = document
        .select(&title_selector)
        .map(|e| e.text().collect::<String>().trim().to_string())
        .collect();
    let snippets: Vec<String> = document
        .select(&snippet_selector)
        .map(|e| e.text().collect::<String>().trim().to_string())
        .collect();

    for (i, title) in titles.iter().enumerate() {
        let snippet = snippets.get(i).cloned().unwrap_or_default();
        if !title.is_empty() || !snippet.is_empty() {
            results.push(SearchResult::new(
                title.clone(),
                snippet,
                "DuckDuckGo".to_string(),
            ));
        }
        if results.len() >= num_results {
            break;
        }
    }

    Ok(results)
}
