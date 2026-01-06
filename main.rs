use clap::{Parser, ValueEnum};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::time::Duration;
use tokio::time::sleep;

mod phone;
mod search;
mod google;
mod bing;
mod duckduckgo;
mod parser;
mod analysis;

use crate::phone::PhoneFormatter;
use crate::search::{SearchResult, SearchConfig};
use crate::analysis::PatternAnalyzer;

const ASCII_LOGO: &str = r#"
████████╗███████╗██╗     ███████╗███████╗██████╗  ██████╗ ████████╗████████╗███████╗██████╗ 
╚══██╔══╝██╔════╝██║     ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚══██╔══╝╚══██╔══╝██╔════╝██╔══██╗
   ██║   █████╗  ██║     █████╗  ███████╗██████╔╝██║   ██║   ██║      ██║   █████╗  ██████╔╝
   ██║   ██╔══╝  ██║     ██╔══╝  ╚════██║██╔═══╝ ██║   ██║   ██║      ██║   ██╔══╝  ██╔══██╗
   ██║   ███████╗███████╗███████╗███████║██║     ╚██████╔╝   ██║      ██║   ███████╗██║  ██║
   ╚═╝   ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝      ╚═════╝    ╚═╝      ╚═╝   ╚══════╝╚═╝  ╚═╝
                                                                              version 2.0
"#;

/// Output format options
#[derive(Debug, Clone, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Json,
    Csv,
    Txt,
}

/// Search engine options
#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Engine {
    Google,
    Bing,
    Duckduckgo,
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Phone Number OSINT Search Tool", long_about = None)]
struct Args {
    /// Phone number to search
    #[arg(help = "Phone number (digits only or formatted)")]
    phone_number: Option<String>,

    /// Enable debug mode (shows errors and sample results)
    #[arg(short, long)]
    debug: bool,

    /// Number of results per search engine
    #[arg(short = 'n', long, default_value = "5")]
    num_results: usize,

    /// Save results to file automatically
    #[arg(short = 's', long)]
    save: bool,

    /// HTTP request timeout in seconds
    #[arg(short = 't', long, default_value = "10")]
    timeout: u64,

    /// Delay between requests in seconds (rate limiting)
    #[arg(long, default_value = "1")]
    delay: u64,

    /// Custom output file path (default: telespotter_results_<phone>.json)
    #[arg(short = 'o', long)]
    output: Option<String>,

    /// Output format: json, csv, or txt
    #[arg(short = 'f', long, value_enum, default_value = "json")]
    format: OutputFormat,

    /// Search engines to use (can specify multiple: -e google -e bing)
    #[arg(short = 'e', long, value_enum, default_value = "all")]
    engines: Vec<Engine>,

    /// Quiet mode - minimal output
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Maximum number of names to display in results
    #[arg(long, default_value = "10")]
    max_names: usize,

    /// Maximum number of locations to display in results
    #[arg(long, default_value = "10")]
    max_locations: usize,

    /// Enable concurrent searches across engines (faster but may trigger rate limits)
    #[arg(short = 'c', long)]
    concurrent: bool,

    /// Number of retry attempts for failed requests
    #[arg(long, default_value = "2")]
    retries: usize,
}

/// Helper to determine if an engine should be used
fn should_use_engine(engines: &[Engine], target: Engine) -> bool {
    engines.is_empty() || engines.contains(&Engine::All) || engines.contains(&target)
}

/// Search Google with retries
async fn search_google_with_retry(
    query: &str,
    num_results: usize,
    config: &SearchConfig,
    retries: usize,
) -> anyhow::Result<Vec<SearchResult>> {
    let mut last_error = None;
    for attempt in 0..=retries {
        match google::search_with_config(query, num_results, config).await {
            Ok(results) => return Ok(results),
            Err(e) => {
                last_error = Some(e);
                if attempt < retries {
                    sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
                }
            }
        }
    }
    Err(last_error.unwrap())
}

/// Search Bing with retries
async fn search_bing_with_retry(
    query: &str,
    num_results: usize,
    config: &SearchConfig,
    retries: usize,
) -> anyhow::Result<Vec<SearchResult>> {
    let mut last_error = None;
    for attempt in 0..=retries {
        match bing::search_with_config(query, num_results, config).await {
            Ok(results) => return Ok(results),
            Err(e) => {
                last_error = Some(e);
                if attempt < retries {
                    sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
                }
            }
        }
    }
    Err(last_error.unwrap())
}

/// Search DuckDuckGo with retries
async fn search_duckduckgo_with_retry(
    query: &str,
    num_results: usize,
    config: &SearchConfig,
    retries: usize,
) -> anyhow::Result<Vec<SearchResult>> {
    let mut last_error = None;
    for attempt in 0..=retries {
        match duckduckgo::search_with_config(query, num_results, config).await {
            Ok(results) => return Ok(results),
            Err(e) => {
                last_error = Some(e);
                if attempt < retries {
                    sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
                }
            }
        }
    }
    Err(last_error.unwrap())
}

/// Search a single engine
async fn search_engine(
    engine: &str,
    query: &str,
    num_results: usize,
    config: &SearchConfig,
    retries: usize,
) -> anyhow::Result<Vec<SearchResult>> {
    match engine {
        "google" => search_google_with_retry(query, num_results, config, retries).await,
        "bing" => search_bing_with_retry(query, num_results, config, retries).await,
        "duckduckgo" => search_duckduckgo_with_retry(query, num_results, config, retries).await,
        _ => Ok(Vec::new()),
    }
}

/// Search all enabled engines concurrently
async fn search_concurrent(
    query: &str,
    num_results: usize,
    config: &SearchConfig,
    engines: &[Engine],
    retries: usize,
) -> Vec<(String, anyhow::Result<Vec<SearchResult>>)> {
    let mut handles = Vec::new();

    if should_use_engine(engines, Engine::Google) {
        let q = query.to_string();
        let cfg = config.clone();
        handles.push(("Google".to_string(), tokio::spawn(async move {
            search_engine("google", &q, num_results, &cfg, retries).await
        })));
    }

    if should_use_engine(engines, Engine::Bing) {
        let q = query.to_string();
        let cfg = config.clone();
        handles.push(("Bing".to_string(), tokio::spawn(async move {
            search_engine("bing", &q, num_results, &cfg, retries).await
        })));
    }

    if should_use_engine(engines, Engine::Duckduckgo) {
        let q = query.to_string();
        let cfg = config.clone();
        handles.push(("DuckDuckGo".to_string(), tokio::spawn(async move {
            search_engine("duckduckgo", &q, num_results, &cfg, retries).await
        })));
    }

    let mut results = Vec::new();
    for (name, handle) in handles {
        let result = handle.await.unwrap_or_else(|e| Err(anyhow::anyhow!("Task failed: {}", e)));
        results.push((name, result));
    }
    results
}

/// Print helper that respects quiet and no-color modes
macro_rules! qprint {
    ($quiet:expr, $no_color:expr, $colored:expr, $plain:expr) => {
        if !$quiet {
            if $no_color {
                println!("{}", $plain);
            } else {
                println!("{}", $colored);
            }
        }
    };
}

macro_rules! qprint_inline {
    ($quiet:expr, $no_color:expr, $colored:expr, $plain:expr) => {
        if !$quiet {
            if $no_color {
                print!("{}", $plain);
            } else {
                print!("{}", $colored);
            }
            let _ = std::io::stdout().flush();
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Handle no-color flag
    if args.no_color {
        colored::control::set_override(false);
    }

    // Print logo (unless quiet mode)
    if !args.quiet {
        if args.no_color {
            println!("{}", ASCII_LOGO);
        } else {
            println!("{}", ASCII_LOGO.cyan().bold());
        }
    }

    if args.debug && !args.quiet {
        qprint!(args.quiet, args.no_color, "🐛 Debug mode enabled\n".yellow(), "Debug mode enabled\n");
    }

    // Get phone number
    let phone_number = match args.phone_number {
        Some(num) => num,
        None => {
            qprint_inline!(false, args.no_color,
                "Enter phone number (digits only or formatted): ".cyan(),
                "Enter phone number (digits only or formatted): ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Generate phone formats
    qprint!(args.quiet, args.no_color,
        "\nGenerating search formats...".yellow(),
        "\nGenerating search formats...");
    let formatter = PhoneFormatter::new(&phone_number)?;
    let formats = formatter.generate_formats();

    qprint!(args.quiet, args.no_color,
        format!("Generated {} search format variations\n", formats.len()).green(),
        format!("Generated {} search format variations\n", formats.len()));

    // Create search config
    let config = SearchConfig {
        timeout_secs: args.timeout,
    };

    // Store all results
    let mut all_results: HashMap<String, Vec<SearchResult>> = HashMap::new();

    // Search each format
    for (i, format) in formats.iter().enumerate() {
        qprint!(args.quiet, args.no_color,
            format!("[{}/{}] Searching: {}", i + 1, formats.len(), format).blue(),
            format!("[{}/{}] Searching: {}", i + 1, formats.len(), format));

        let mut format_results = Vec::new();

        if args.concurrent {
            // Concurrent search mode
            let results = search_concurrent(format, args.num_results, &config, &args.engines, args.retries).await;

            for (engine_name, result) in results {
                match result {
                    Ok(res) => {
                        let count = res.len();
                        format_results.extend(res);
                        qprint!(args.quiet, args.no_color,
                            format!("  → {}: {} results", engine_name, count).green(),
                            format!("  → {}: {} results", engine_name, count));
                    }
                    Err(e) if args.debug => {
                        qprint!(args.quiet, args.no_color,
                            format!("  → {}: Error: {}", engine_name, e).yellow(),
                            format!("  → {}: Error: {}", engine_name, e));
                    }
                    Err(_) => {
                        qprint!(args.quiet, args.no_color,
                            format!("  → {}: 0 results", engine_name).yellow(),
                            format!("  → {}: 0 results", engine_name));
                    }
                }
            }
        } else {
            // Sequential search mode
            if should_use_engine(&args.engines, Engine::Google) {
                qprint_inline!(args.quiet, args.no_color,
                    "  → Searching Google... ".cyan(),
                    "  → Searching Google... ");
                match search_engine("google", format, args.num_results, &config, args.retries).await {
                    Ok(results) => {
                        let count = results.len();
                        format_results.extend(results);
                        qprint!(args.quiet, args.no_color,
                            format!("({} results)", count).green(),
                            format!("({} results)", count));
                    }
                    Err(e) if args.debug => {
                        qprint!(args.quiet, args.no_color,
                            format!("Error: {}", e).yellow(),
                            format!("Error: {}", e));
                    }
                    Err(_) => {
                        qprint!(args.quiet, args.no_color,
                            "(0 results)".yellow(),
                            "(0 results)");
                    }
                }
                sleep(Duration::from_secs(args.delay)).await;
            }

            if should_use_engine(&args.engines, Engine::Bing) {
                qprint_inline!(args.quiet, args.no_color,
                    "  → Searching Bing... ".cyan(),
                    "  → Searching Bing... ");
                match search_engine("bing", format, args.num_results, &config, args.retries).await {
                    Ok(results) => {
                        let count = results.len();
                        format_results.extend(results);
                        qprint!(args.quiet, args.no_color,
                            format!("({} results)", count).green(),
                            format!("({} results)", count));
                    }
                    Err(e) if args.debug => {
                        qprint!(args.quiet, args.no_color,
                            format!("Error: {}", e).yellow(),
                            format!("Error: {}", e));
                    }
                    Err(_) => {
                        qprint!(args.quiet, args.no_color,
                            "(0 results)".yellow(),
                            "(0 results)");
                    }
                }
                sleep(Duration::from_secs(args.delay)).await;
            }

            if should_use_engine(&args.engines, Engine::Duckduckgo) {
                qprint_inline!(args.quiet, args.no_color,
                    "  → Searching DuckDuckGo... ".cyan(),
                    "  → Searching DuckDuckGo... ");
                match search_engine("duckduckgo", format, args.num_results, &config, args.retries).await {
                    Ok(results) => {
                        let count = results.len();
                        format_results.extend(results);
                        qprint!(args.quiet, args.no_color,
                            format!("({} results)", count).green(),
                            format!("({} results)", count));
                    }
                    Err(e) if args.debug => {
                        qprint!(args.quiet, args.no_color,
                            format!("Error: {}", e).yellow(),
                            format!("Error: {}", e));
                    }
                    Err(_) => {
                        qprint!(args.quiet, args.no_color,
                            "(0 results)".yellow(),
                            "(0 results)");
                    }
                }
            }
        }

        qprint!(args.quiet, args.no_color,
            format!("  ✓ Total: {} results for this format", format_results.len()).green(),
            format!("  Total: {} results for this format", format_results.len()));

        if args.debug && !format_results.is_empty() && !args.quiet {
            let sample = &format_results[0].title;
            let truncated = if sample.len() > 60 {
                format!("{}...", &sample[..60])
            } else {
                sample.clone()
            };
            qprint!(args.quiet, args.no_color,
                format!("  Debug: Sample - {}", truncated).yellow(),
                format!("  Debug: Sample - {}", truncated));
        }

        all_results.insert(format.clone(), format_results);

        // Rate limiting between formats
        if i < formats.len() - 1 {
            let wait_time = args.delay * 3;
            qprint!(args.quiet, args.no_color,
                format!("  ⏳ Waiting {} seconds...\n", wait_time).yellow(),
                format!("  Waiting {} seconds...\n", wait_time));
            sleep(Duration::from_secs(wait_time)).await;
        } else if !args.quiet {
            println!();
        }
    }

    // Analyze patterns
    qprint!(args.quiet, args.no_color,
        "Analyzing patterns across all results...".yellow(),
        "Analyzing patterns across all results...");
    let analyzer = PatternAnalyzer::new();
    let patterns = analyzer.analyze(&all_results, args.max_names, args.max_locations);

    // Print summary (unless quiet mode)
    if !args.quiet {
        patterns.print_summary(args.no_color);
    }

    // Save results if requested or prompted
    let should_save = if args.save {
        true
    } else if args.quiet {
        false
    } else {
        qprint_inline!(false, args.no_color,
            "Save detailed results to file? (y/n): ".cyan(),
            "Save detailed results to file? (y/n): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.trim().to_lowercase() == "y"
    };

    if should_save {
        let digits: String = phone_number.chars().filter(|c| c.is_numeric()).collect();

        // Determine filename
        let filename = match &args.output {
            Some(path) => path.clone(),
            None => {
                let ext = match args.format {
                    OutputFormat::Json => "json",
                    OutputFormat::Csv => "csv",
                    OutputFormat::Txt => "txt",
                };
                format!("telespotter_results_{}.{}", digits, ext)
            }
        };

        // Format and save based on output format
        match args.format {
            OutputFormat::Json => {
                let output = serde_json::json!({
                    "phone_number": phone_number,
                    "search_formats": formats,
                    "results": all_results,
                    "pattern_analysis": patterns.to_json()
                });
                fs::write(&filename, serde_json::to_string_pretty(&output)?)?;
            }
            OutputFormat::Csv => {
                let mut csv_content = String::from("Source,Title,Snippet\n");
                for results in all_results.values() {
                    for result in results {
                        let title = result.title.replace('"', "\"\"");
                        let snippet = result.snippet.replace('"', "\"\"");
                        csv_content.push_str(&format!("\"{}\",\"{}\",\"{}\"\n",
                            result.source, title, snippet));
                    }
                }
                fs::write(&filename, csv_content)?;
            }
            OutputFormat::Txt => {
                let mut txt_content = format!("Telespotter Results for: {}\n", phone_number);
                txt_content.push_str(&"=".repeat(60));
                txt_content.push('\n');

                txt_content.push_str(&format!("\nTotal Results: {}\n", patterns.total_results));

                if !patterns.common_names.is_empty() {
                    txt_content.push_str("\nNames Found:\n");
                    for (name, count) in &patterns.common_names {
                        txt_content.push_str(&format!("  - {}: {} time(s)\n", name, count));
                    }
                }

                if !patterns.common_locations.is_empty() {
                    txt_content.push_str("\nLocations Found:\n");
                    for (loc, count) in &patterns.common_locations {
                        txt_content.push_str(&format!("  - {}: {} occurrence(s)\n", loc, count));
                    }
                }

                txt_content.push_str(&format!("\n{}\n", "=".repeat(60)));
                txt_content.push_str("\nDetailed Results:\n\n");

                for (format, results) in &all_results {
                    txt_content.push_str(&format!("Format: {}\n", format));
                    for result in results {
                        txt_content.push_str(&format!("  [{}] {}\n", result.source, result.title));
                        if !result.snippet.is_empty() {
                            txt_content.push_str(&format!("       {}\n", result.snippet));
                        }
                    }
                    txt_content.push('\n');
                }

                fs::write(&filename, txt_content)?;
            }
        }

        qprint!(args.quiet, args.no_color,
            format!("Results saved to: {}\n", filename).green(),
            format!("Results saved to: {}\n", filename));
    }

    Ok(())
}
