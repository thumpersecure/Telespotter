use crate::search::{create_client, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

pub async fn search(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
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
