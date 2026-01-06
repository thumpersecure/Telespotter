use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search USPhoneBook for phone number information
pub async fn search_with_config(phone: &str, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);

    // Format phone for USPhoneBook URL
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    let formatted = if digits.len() == 10 {
        format!("{}-{}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
    } else if digits.len() == 11 && digits.starts_with('1') {
        format!("{}-{}-{}", &digits[1..4], &digits[4..7], &digits[7..11])
    } else {
        digits.clone()
    };

    let url = format!("https://www.usphonebook.com/{}", formatted);

    let response = client
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("USPhoneBook error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // USPhoneBook person cards/records
    let card_selectors = [
        ".person-card",
        ".result-card",
        ".phone-record",
        ".card",
        "article",
        ".listing",
    ];

    for selector_str in card_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector).take(5) {
                // Extract name
                let name_selectors = [".name", "h2", "h3", ".card-title", ".person-name", "a h2", "strong"];
                let mut name = String::new();

                for ns in name_selectors {
                    if let Ok(name_sel) = Selector::parse(ns) {
                        if let Some(name_elem) = element.select(&name_sel).next() {
                            let text = name_elem.text().collect::<String>().trim().to_string();
                            // Filter out generic text
                            if !text.is_empty() && text.len() > 2 &&
                               !text.to_lowercase().contains("search") &&
                               !text.to_lowercase().contains("phone") &&
                               !text.to_lowercase().contains("book") {
                                name = text;
                                break;
                            }
                        }
                    }
                }

                // Extract address
                let addr_selectors = [".address", ".location", "address", ".card-address", ".current-address"];
                let mut address = String::new();

                for as_ in addr_selectors {
                    if let Ok(addr_sel) = Selector::parse(as_) {
                        if let Some(addr_elem) = element.select(&addr_sel).next() {
                            let text = addr_elem.text().collect::<String>().trim().to_string();
                            if text.len() > 5 {
                                address = text.split_whitespace().collect::<Vec<_>>().join(" ");
                                break;
                            }
                        }
                    }
                }

                // Extract carrier/type info
                let type_selectors = [".phone-type", ".carrier", ".line-type"];
                let mut phone_type = String::new();

                for ts in type_selectors {
                    if let Ok(type_sel) = Selector::parse(ts) {
                        if let Some(type_elem) = element.select(&type_sel).next() {
                            let text = type_elem.text().collect::<String>().trim().to_string();
                            if !text.is_empty() {
                                phone_type = text;
                                break;
                            }
                        }
                    }
                }

                if !name.is_empty() {
                    let mut snippet_parts = Vec::new();
                    if !address.is_empty() {
                        snippet_parts.push(address);
                    }
                    if !phone_type.is_empty() {
                        snippet_parts.push(format!("Type: {}", phone_type));
                    }

                    results.push(SearchResult::new(
                        format!("USPhoneBook: {}", name),
                        if snippet_parts.is_empty() {
                            "Phone owner found".to_string()
                        } else {
                            snippet_parts.join(" | ")
                        },
                        "USPhoneBook".to_string(),
                    ));
                }
            }
        }
        if !results.is_empty() {
            break;
        }
    }

    // Fallback: Parse page for any useful content
    if results.is_empty() {
        // Look for the main content area
        let main_selectors = ["main", "#main", ".main-content", ".content"];

        for ms in main_selectors {
            if let Ok(main_sel) = Selector::parse(ms) {
                if let Some(main) = document.select(&main_sel).next() {
                    // Look for headings that might be names
                    let h_selector = Selector::parse("h1, h2, h3").ok();
                    if let Some(selector) = h_selector {
                        for element in main.select(&selector).take(3) {
                            let text = element.text().collect::<String>().trim().to_string();
                            // Filter to likely name content
                            if text.len() > 3 && text.len() < 100 &&
                               text.chars().filter(|c| c.is_alphabetic()).count() > 3 &&
                               !text.to_lowercase().contains("phone") &&
                               !text.to_lowercase().contains("search") &&
                               !text.to_lowercase().contains("free") {
                                results.push(SearchResult::new(
                                    "USPhoneBook".to_string(),
                                    text,
                                    "USPhoneBook".to_string(),
                                ));
                                break;
                            }
                        }
                    }
                    if !results.is_empty() {
                        break;
                    }
                }
            }
        }
    }

    // Check for "no results" or empty state
    let body_text = document.select(&Selector::parse("body").unwrap())
        .next()
        .map(|b| b.text().collect::<String>())
        .unwrap_or_default();

    if body_text.to_lowercase().contains("no results") ||
       body_text.to_lowercase().contains("not found") ||
       body_text.to_lowercase().contains("no records") {
        // No results found, return empty
        return Ok(Vec::new());
    }

    Ok(results)
}
