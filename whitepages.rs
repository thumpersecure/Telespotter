use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search Whitepages for phone number information
pub async fn search_with_config(phone: &str, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);

    // Format phone for Whitepages URL (digits only)
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    let url = format!("https://www.whitepages.com/phone/{}", digits);

    let response = client
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Whitepages search error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // Try to extract person info from Whitepages
    // Whitepages structure varies, trying multiple selectors
    let name_selectors = [
        "h2.name",
        "span.name",
        ".person-name",
        "[data-testid='person-name']",
        "h1.title",
        ".full-name",
    ];

    let address_selectors = [
        ".address",
        ".current-address",
        "[data-testid='address']",
        ".location",
        ".addr",
    ];

    let mut found_name = String::new();
    let mut found_address = String::new();

    for selector_str in name_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text = element.text().collect::<String>().trim().to_string();
                if !text.is_empty() && text.len() > 2 {
                    found_name = text;
                    break;
                }
            }
        }
    }

    for selector_str in address_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text = element.text().collect::<String>().trim().to_string();
                if !text.is_empty() && text.len() > 3 {
                    found_address = text;
                    break;
                }
            }
        }
    }

    // Also look for any text content that might contain relevant info
    let body_selector = Selector::parse("body").unwrap();
    if let Some(body) = document.select(&body_selector).next() {
        let body_text = body.text().collect::<String>();

        // Check if there's meaningful content indicating a match
        if body_text.contains("Owner") || body_text.contains("Resident") ||
           body_text.contains("Location") || body_text.contains("Address") {

            if !found_name.is_empty() || !found_address.is_empty() {
                let title = if !found_name.is_empty() {
                    format!("Whitepages: {}", found_name)
                } else {
                    "Whitepages Result".to_string()
                };

                let snippet = if !found_address.is_empty() {
                    found_address
                } else if !found_name.is_empty() {
                    format!("Phone registered to: {}", found_name)
                } else {
                    "Phone record found".to_string()
                };

                results.push(SearchResult::new(
                    title,
                    snippet,
                    "Whitepages".to_string(),
                ));
            }
        }
    }

    // Parse any listing cards
    let card_selector = Selector::parse(".listing, .result-card, .person-card").ok();
    if let Some(selector) = card_selector {
        for element in document.select(&selector).take(5) {
            let text = element.text().collect::<String>();
            let clean_text: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
            if clean_text.len() > 10 {
                results.push(SearchResult::new(
                    "Whitepages Listing".to_string(),
                    clean_text.chars().take(200).collect(),
                    "Whitepages".to_string(),
                ));
            }
        }
    }

    Ok(results)
}
