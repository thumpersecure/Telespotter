use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search TruePeopleSearch for phone number information
pub async fn search_with_config(phone: &str, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);

    // Format phone for TruePeopleSearch URL
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    let url = format!("https://www.truepeoplesearch.com/resultphone?phoneno={}", digits);

    let response = client
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("TruePeopleSearch error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // TruePeopleSearch result cards
    let card_selectors = [
        ".card-summary",
        ".card",
        ".result-card",
        ".person-card",
        "[data-detail-link]",
    ];

    for selector_str in card_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector).take(5) {
                // Look for name within the card
                let name_selectors = [".h4", "h4", ".name", ".card-title", "a.h4"];
                let mut name = String::new();

                for ns in name_selectors {
                    if let Ok(name_sel) = Selector::parse(ns) {
                        if let Some(name_elem) = element.select(&name_sel).next() {
                            let text = name_elem.text().collect::<String>().trim().to_string();
                            if !text.is_empty() && text.len() > 2 {
                                name = text;
                                break;
                            }
                        }
                    }
                }

                // Look for address/location
                let addr_selectors = [".address", ".location", ".content-value", "span"];
                let mut address = String::new();

                for as_ in addr_selectors {
                    if let Ok(addr_sel) = Selector::parse(as_) {
                        if let Some(addr_elem) = element.select(&addr_sel).next() {
                            let text = addr_elem.text().collect::<String>().trim().to_string();
                            if text.len() > 5 && (text.contains(',') || text.chars().any(|c| c.is_numeric())) {
                                address = text;
                                break;
                            }
                        }
                    }
                }

                if !name.is_empty() {
                    results.push(SearchResult::new(
                        format!("TruePeopleSearch: {}", name),
                        if address.is_empty() { "Phone match found".to_string() } else { address },
                        "TruePeopleSearch".to_string(),
                    ));
                }
            }
        }
        if !results.is_empty() {
            break;
        }
    }

    // Fallback: parse the page content
    if results.is_empty() {
        let body_selector = Selector::parse("main, #main, .content, body").unwrap();
        if let Some(body) = document.select(&body_selector).next() {
            let body_text = body.text().collect::<String>();

            // Check for indication of results
            if body_text.contains("Age") || body_text.contains("Lives in") ||
               body_text.contains("Address") || body_text.contains("Related") {
                let clean_text: String = body_text.split_whitespace().collect::<Vec<_>>().join(" ");
                if clean_text.len() > 20 {
                    results.push(SearchResult::new(
                        "TruePeopleSearch Result".to_string(),
                        clean_text.chars().take(250).collect(),
                        "TruePeopleSearch".to_string(),
                    ));
                }
            }
        }
    }

    Ok(results)
}
