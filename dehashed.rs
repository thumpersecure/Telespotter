use crate::search::SearchResult;
use anyhow::Result;
use serde::Deserialize;
use std::env;

/// Response shape from the Dehashed v2 search API.
/// Only the fields we surface are modeled; unknown fields are ignored.
#[derive(Debug, Deserialize)]
struct DehashedResponse {
    #[serde(default)]
    entries: Option<Vec<DehashedEntry>>,
    #[serde(default)]
    total: Option<u64>,
}

#[derive(Debug, Deserialize, Default)]
struct DehashedEntry {
    #[serde(default)]
    name: Option<serde_json::Value>,
    #[serde(default)]
    email: Option<serde_json::Value>,
    #[serde(default)]
    username: Option<serde_json::Value>,
    #[serde(default)]
    address: Option<serde_json::Value>,
    #[serde(default)]
    phone: Option<serde_json::Value>,
    #[serde(default)]
    database_name: Option<String>,
}

/// Coerce a Dehashed field (which may be a string or an array of strings) into
/// a single display string.
fn field_to_string(v: &Option<serde_json::Value>) -> String {
    match v {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|x| x.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        _ => String::new(),
    }
}

/// Search Dehashed for records associated with a phone number.
///
/// Uses the v2 API: POST https://api.dehashed.com/v2/search with a
/// `Dehashed-Api-Key` header. The API key is read from the DEHASHED_API_KEY
/// environment variable (same env-var pattern the Google module uses for its
/// credentials). Returns an empty result set when the key is not configured.
pub async fn search_with_config(phone: &str, client: &reqwest::Client) -> Result<Vec<SearchResult>> {
    let api_key = match env::var("DEHASHED_API_KEY") {
        Ok(k) if !k.trim().is_empty() => k,
        _ => return Ok(Vec::new()),
    };

    let body = serde_json::json!({
        "query": phone,
        "page": 1,
    });

    let response = client
        .post("https://api.dehashed.com/v2/search")
        .header("Dehashed-Api-Key", api_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Dehashed API error: {}", response.status()));
    }

    let parsed: DehashedResponse = response.json().await?;

    let mut results = Vec::new();
    let total = parsed.total.unwrap_or(0);

    for entry in parsed.entries.unwrap_or_default() {
        let name = field_to_string(&entry.name);
        let email = field_to_string(&entry.email);
        let username = field_to_string(&entry.username);
        let address = field_to_string(&entry.address);
        let phone_val = field_to_string(&entry.phone);
        let db = entry.database_name.unwrap_or_default();

        // Title: prefer a name, then email, then username, then the breach name.
        let title = if !name.is_empty() {
            format!("Dehashed: {}", name)
        } else if !email.is_empty() {
            format!("Dehashed: {}", email)
        } else if !username.is_empty() {
            format!("Dehashed: @{}", username)
        } else if !db.is_empty() {
            format!("Dehashed record ({})", db)
        } else {
            "Dehashed record".to_string()
        };

        // Snippet: join whatever identifiers are present so the analyzer can
        // extract emails / usernames / locations downstream.
        let mut parts = Vec::new();
        if !email.is_empty() {
            parts.push(email);
        }
        if !username.is_empty() {
            parts.push(format!("@{}", username));
        }
        if !phone_val.is_empty() {
            parts.push(phone_val);
        }
        if !address.is_empty() {
            parts.push(address);
        }
        if !db.is_empty() {
            parts.push(format!("breach: {}", db));
        }

        let snippet = if parts.is_empty() {
            "Record found in Dehashed".to_string()
        } else {
            parts.join(" | ")
        };

        results.push(SearchResult::new(title, snippet, "Dehashed".to_string()));
    }

    if results.is_empty() && total > 0 {
        results.push(SearchResult::new(
            "Dehashed".to_string(),
            format!("{} record(s) reported but no displayable fields returned", total),
            "Dehashed".to_string(),
        ));
    }

    Ok(results)
}
