use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search FastPeopleSearch for phone number information
pub async fn search_with_config(phone: &str, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);

    // Format phone for FastPeopleSearch URL (with dashes)
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    let formatted = if digits.len() == 10 {
        format!("{}-{}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
    } else {
        digits.clone()
    };

    let url = format!("https://www.fastpeoplesearch.com/{}", formatted);

    let response = client
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("FastPeopleSearch error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // FastPeopleSearch detail sections
    let section_selectors = [
        ".detail-box",
        ".card",
        ".result-item",
        ".person-detail",
        "article",
    ];

    for selector_str in section_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector).take(5) {
                // Look for owner/name information
                let name_selectors = ["h2", "h3", ".owner-name", ".name", "a strong"];
                let mut name = String::new();

                for ns in name_selectors {
                    if let Ok(name_sel) = Selector::parse(ns) {
                        if let Some(name_elem) = element.select(&name_sel).next() {
                            let text = name_elem.text().collect::<String>().trim().to_string();
                            // Filter out generic headers
                            if !text.is_empty() && text.len() > 2 &&
                               !text.to_lowercase().contains("search") &&
                               !text.to_lowercase().contains("phone") {
                                name = text;
                                break;
                            }
                        }
                    }
                }

                // Look for address
                let addr_selectors = [".address", ".detail-box-address", "address", ".location-info"];
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

                // Look for age/details
                let age_selectors = [".age", ".detail-box-age", ".person-age"];
                let mut age_info = String::new();

                for age_s in age_selectors {
                    if let Ok(age_sel) = Selector::parse(age_s) {
                        if let Some(age_elem) = element.select(&age_sel).next() {
                            let text = age_elem.text().collect::<String>().trim().to_string();
                            if !text.is_empty() {
                                age_info = text;
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
                    if !age_info.is_empty() {
                        snippet_parts.push(format!("Age: {}", age_info));
                    }

                    results.push(SearchResult::new(
                        format!("FastPeopleSearch: {}", name),
                        if snippet_parts.is_empty() {
                            "Phone owner found".to_string()
                        } else {
                            snippet_parts.join(" | ")
                        },
                        "FastPeopleSearch".to_string(),
                    ));
                }
            }
        }
        if !results.is_empty() {
            break;
        }
    }

    // Fallback: check for owner info in the page
    if results.is_empty() {
        let owner_selector = Selector::parse("h1, h2, .phone-owner").ok();
        if let Some(selector) = owner_selector {
            for element in document.select(&selector).take(3) {
                let text = element.text().collect::<String>().trim().to_string();
                if text.len() > 3 &&
                   !text.to_lowercase().contains("search") &&
                   !text.to_lowercase().contains("free") &&
                   text.chars().filter(|c| c.is_alphabetic()).count() > 3 {
                    results.push(SearchResult::new(
                        "FastPeopleSearch".to_string(),
                        text.chars().take(200).collect(),
                        "FastPeopleSearch".to_string(),
                    ));
                    break;
                }
            }
        }
    }

    Ok(results)
}
