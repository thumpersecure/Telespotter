use crate::search::{create_client_from_config, SearchConfig, SearchResult};
use anyhow::Result;
use scraper::{Html, Selector};

/// Search ThatsThem for phone number information
pub async fn search_with_config(phone: &str, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let client = create_client_from_config(config);

    // Format phone for ThatsThem URL
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    let formatted = if digits.len() == 10 {
        format!("{}-{}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
    } else if digits.len() == 11 && digits.starts_with('1') {
        format!("{}-{}-{}", &digits[1..4], &digits[4..7], &digits[7..11])
    } else {
        digits.clone()
    };

    let url = format!("https://thatsthem.com/phone/{}", formatted);

    let response = client
        .get(&url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("ThatsThem error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    // ThatsThem result containers
    let container_selectors = [
        ".ThatsThem-record",
        ".result-record",
        ".record-card",
        ".person-record",
        ".search-result",
    ];

    for selector_str in container_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector).take(5) {
                // Extract name
                let name_selectors = [".ThatsThem-name", ".name", "h2", "h3", ".record-name", "a.name"];
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

                // Extract address
                let addr_selectors = [".ThatsThem-address", ".address", ".location", ".record-address"];
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

                // Extract age
                let age_selectors = [".ThatsThem-age", ".age", ".record-age"];
                let mut age = String::new();

                for age_s in age_selectors {
                    if let Ok(age_sel) = Selector::parse(age_s) {
                        if let Some(age_elem) = element.select(&age_sel).next() {
                            let text = age_elem.text().collect::<String>().trim().to_string();
                            if !text.is_empty() {
                                age = text;
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
                    if !age.is_empty() {
                        snippet_parts.push(format!("Age: {}", age));
                    }

                    results.push(SearchResult::new(
                        format!("ThatsThem: {}", name),
                        if snippet_parts.is_empty() {
                            "Phone record found".to_string()
                        } else {
                            snippet_parts.join(" | ")
                        },
                        "ThatsThem".to_string(),
                    ));
                }
            }
        }
        if !results.is_empty() {
            break;
        }
    }

    // Fallback: look for any result indication
    if results.is_empty() {
        let body_selector = Selector::parse("main, .content, .results, body").unwrap();
        if let Some(body) = document.select(&body_selector).next() {
            let body_text = body.text().collect::<String>();

            // Check for indication of a match
            if body_text.contains("Lives at") || body_text.contains("years old") ||
               body_text.contains("Associated") || body_text.contains("Address") {

                // Try to extract key info
                let h2_selector = Selector::parse("h2, h3, .name").ok();
                if let Some(selector) = h2_selector {
                    for element in document.select(&selector).take(3) {
                        let text = element.text().collect::<String>().trim().to_string();
                        if text.len() > 3 && text.chars().filter(|c| c.is_alphabetic()).count() > 3 {
                            results.push(SearchResult::new(
                                "ThatsThem Result".to_string(),
                                text.chars().take(200).collect(),
                                "ThatsThem".to_string(),
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(results)
}
