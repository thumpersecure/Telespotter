use clap::Parser;
use colored::*;
use std::collections::HashMap;
use std::fs;
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
use crate::search::SearchResult;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Phone number to search
    #[arg(help = "Phone number (digits only or formatted)")]
    phone_number: Option<String>,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,

    /// Number of results per search engine
    #[arg(short = 'n', long, default_value = "5")]
    num_results: usize,

    /// Save results to JSON file
    #[arg(short = 's', long)]
    save: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Print logo
    println!("{}", ASCII_LOGO.cyan().bold());

    if args.debug {
        println!("{}", "🐛 Debug mode enabled\n".yellow());
    }

    // Get phone number
    let phone_number = match args.phone_number {
        Some(num) => num,
        None => {
            print!("{}", "Enter phone number (digits only or formatted): ".cyan());
            use std::io::{self, Write};
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Generate phone formats
    println!("\n{}", "Generating search formats...".yellow());
    let formatter = PhoneFormatter::new(&phone_number)?;
    let formats = formatter.generate_formats();

    println!("{}", format!("Generated {} search format variations\n", formats.len()).green());

    // Store all results
    let mut all_results: HashMap<String, Vec<SearchResult>> = HashMap::new();

    // Search each format
    for (i, format) in formats.iter().enumerate() {
        println!(
            "{}",
            format!("[{}/{}] Searching: {}", i + 1, formats.len(), format).blue()
        );

        let mut format_results = Vec::new();

        // Search Google
        print!("  {} ", "→ Searching Google...".cyan());
        match google::search(format, args.num_results).await {
            Ok(results) => {
                let count = results.len();
                format_results.extend(results);
                println!("{}", format!("({} results)", count).green());
            }
            Err(e) if args.debug => {
                println!("{}", format!("Error: {}", e).yellow());
            }
            Err(_) => {
                println!("{}", "(0 results)".yellow());
            }
        }

        sleep(Duration::from_secs(1)).await;

        // Search Bing
        print!("  {} ", "→ Searching Bing...".cyan());
        match bing::search(format, args.num_results).await {
            Ok(results) => {
                let count = results.len();
                format_results.extend(results);
                println!("{}", format!("({} results)", count).green());
            }
            Err(e) if args.debug => {
                println!("{}", format!("Error: {}", e).yellow());
            }
            Err(_) => {
                println!("{}", "(0 results)".yellow());
            }
        }

        sleep(Duration::from_secs(1)).await;

        // Search DuckDuckGo
        print!("  {} ", "→ Searching DuckDuckGo...".cyan());
        match duckduckgo::search(format, args.num_results).await {
            Ok(results) => {
                let count = results.len();
                format_results.extend(results);
                println!("{}", format!("({} results)", count).green());
            }
            Err(e) if args.debug => {
                println!("{}", format!("Error: {}", e).yellow());
            }
            Err(_) => {
                println!("{}", "(0 results)".yellow());
            }
        }

        println!(
            "  {}",
            format!("✓ Total: {} results for this format", format_results.len()).green()
        );

        if args.debug && !format_results.is_empty() {
            let sample = &format_results[0].title;
            let truncated = if sample.len() > 60 {
                format!("{}...", &sample[..60])
            } else {
                sample.clone()
            };
            println!("  {}", format!("Debug: Sample - {}", truncated).yellow());
        }

        all_results.insert(format.clone(), format_results);

        // Rate limiting between formats
        if i < formats.len() - 1 {
            println!("  {}\n", "⏳ Waiting 3 seconds...".yellow());
            sleep(Duration::from_secs(3)).await;
        } else {
            println!();
        }
    }

    // Analyze patterns
    println!("{}", "Analyzing patterns across all results...".yellow());
    let analyzer = PatternAnalyzer::new();
    let patterns = analyzer.analyze(&all_results);

    // Print summary
    patterns.print_summary();

    // Save results if requested or prompted
    let should_save = if args.save {
        true
    } else {
        print!("{}", "Save detailed results to file? (y/n): ".cyan());
        use std::io::{self, Write};
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_lowercase() == "y"
    };

    if should_save {
        let digits: String = phone_number.chars().filter(|c| c.is_numeric()).collect();
        let filename = format!("telespotter_results_{}.json", digits);

        let output = serde_json::json!({
            "phone_number": phone_number,
            "search_formats": formats,
            "results": all_results,
            "pattern_analysis": patterns.to_json()
        });

        fs::write(&filename, serde_json::to_string_pretty(&output)?)?;
        println!("{}", format!("Results saved to: {}\n", filename).green());
    }

    Ok(())
}
