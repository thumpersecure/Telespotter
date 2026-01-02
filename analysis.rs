use crate::parser::{extract_locations, extract_names};
use crate::search::SearchResult;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternAnalysis {
    pub total_results: usize,
    pub results_by_source: HashMap<String, usize>,
    pub common_names: Vec<(String, usize)>,
    pub common_locations: Vec<(String, usize)>,
}

impl PatternAnalysis {
    pub fn print_summary(&self) {
        println!("\n{}", "=".repeat(80).magenta().bold());
        println!("{}", "PATTERN ANALYSIS SUMMARY".magenta().bold());
        println!("{}", "=".repeat(80).magenta().bold());
        println!();

        println!(
            "{} {}",
            "Total Results Found:".cyan(),
            self.total_results
        );
        println!();

        // Source breakdown
        if !self.results_by_source.is_empty() {
            println!("{}", "Results by Source:".blue().bold());
            for (source, count) in &self.results_by_source {
                println!("  • {}: {} results", source.green(), count);
            }
            println!();
        }

        // Name patterns
        if !self.common_names.is_empty() {
            println!("{}", "📛 Names Found:".blue().bold());
            for (name, count) in &self.common_names {
                println!("  • {}: mentioned {} time(s)", name.green(), count);
            }
            println!();
        } else {
            println!("{}\n", "No names detected in search results".yellow());
        }

        // Location patterns
        if !self.common_locations.is_empty() {
            println!("{}", "📍 Locations Mentioned:".blue().bold());
            for (location, count) in &self.common_locations {
                println!("  • {}: {} occurrence(s)", location.green(), count);
            }
            println!();
        } else {
            println!("{}\n", "No locations detected in search results".yellow());
        }

        // Key insights
        println!("{}", "🔍 Key Insights:".blue().bold());

        if self.total_results == 0 {
            println!(
                "  • {}",
                "No results found for this phone number".yellow()
            );
        } else {
            if let Some((name, _)) = self.common_names.first() {
                println!("  • {}: {}", "Most associated name".green(), name);
            }

            if let Some((location, _)) = self.common_locations.first() {
                println!("  • {}: {}", "Most associated location".green(), location);
            }

            if self.common_names.is_empty() && self.common_locations.is_empty() {
                println!(
                    "  • {}",
                    "Found results but no clear name or location patterns".yellow()
                );
            }
        }

        println!("\n{}\n", "=".repeat(80).magenta().bold());
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "total_results": self.total_results,
            "results_by_source": self.results_by_source,
            "common_names": self.common_names,
            "common_locations": self.common_locations,
        })
    }
}

pub struct PatternAnalyzer;

impl PatternAnalyzer {
    pub fn new() -> Self {
        PatternAnalyzer
    }

    pub fn analyze(&self, all_results: &HashMap<String, Vec<SearchResult>>) -> PatternAnalysis {
        let mut all_text = Vec::new();
        let mut source_counts: HashMap<String, usize> = HashMap::new();

        // Collect all text and count sources
        for results in all_results.values() {
            for result in results {
                let text = format!("{} {}", result.title, result.snippet);
                all_text.push(text);
                *source_counts.entry(result.source.clone()).or_insert(0) += 1;
            }
        }

        // Combine all text
        let combined_text = all_text.join(" ");

        // Extract names and locations
        let names = extract_names(&combined_text);
        let locations = extract_locations(&combined_text);

        // Count occurrences
        let name_counts = count_occurrences(&names);
        let location_counts = count_occurrences(&locations);

        // Sort by frequency and take top 10
        let mut common_names: Vec<(String, usize)> = name_counts.into_iter().collect();
        common_names.sort_by(|a, b| b.1.cmp(&a.1));
        common_names.truncate(10);

        let mut common_locations: Vec<(String, usize)> = location_counts.into_iter().collect();
        common_locations.sort_by(|a, b| b.1.cmp(&a.1));
        common_locations.truncate(10);

        PatternAnalysis {
            total_results: all_text.len(),
            results_by_source: source_counts,
            common_names,
            common_locations,
        }
    }
}

fn count_occurrences(items: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurrences() {
        let items = vec![
            "John Smith".to_string(),
            "Jane Doe".to_string(),
            "John Smith".to_string(),
        ];
        let counts = count_occurrences(&items);
        assert_eq!(counts.get("John Smith"), Some(&2));
        assert_eq!(counts.get("Jane Doe"), Some(&1));
    }
}
