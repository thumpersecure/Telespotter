# 📚 TeleSpotter Rust - Usage Examples

Comprehensive examples for using TeleSpotter.

## Basic Searches

### Simple search
```bash
telespotter 5555551212
```

### Search with formatted number
```bash
telespotter "(555) 555-1212"
telespotter "555-555-1212"
telespotter "1-555-555-1212"
```

### Interactive mode (no arguments)
```bash
telespotter
# You'll be prompted to enter the phone number
```

## Debug Mode

### See detailed error messages
```bash
telespotter --debug 5555551212
telespotter -d "(555) 555-1212"
```

Debug mode shows:
- HTTP request errors
- HTML parsing issues
- Sample results from each search
- Rate limiting delays

## Customizing Results

### Get more results per engine (default is 5)
```bash
# 10 results per engine (40 total results)
telespotter -n 10 5555551212

# 15 results per engine (60 total results)
telespotter -n 15 5555551212
```

### Combine with debug mode
```bash
telespotter -d -n 10 5555551212
```

## Saving Results

### Auto-save to JSON
```bash
# Automatically saves without prompting
telespotter --save 5555551212
telespotter -s 5555551212
```

### Output filename
Results are saved as: `telespotter_results_5555551212.json`

### Combine all options
```bash
# Debug mode + auto-save + 10 results per engine
telespotter -d -s -n 10 5555551212
```

## Real-World Scenarios

### Investigation Workflow
```bash
# 1. Quick initial search
telespotter 2155551234

# 2. If no results, try debug mode
telespotter -d 2155551234

# 3. Need more data? Increase results
telespotter -n 15 -s 2155551234

# 4. Save for legal documentation
telespotter -s 2155551234
```

### Batch Processing (Script)
```bash
#!/bin/bash
# search_multiple.sh

numbers=(
    "5555551234"
    "5555555678"
    "5555559012"
)

for number in "${numbers[@]}"; do
    echo "Searching $number..."
    telespotter -s -n 10 "$number"
    sleep 60  # Wait between searches to avoid rate limiting
done
```

### Legal Evidence Gathering
```bash
# Comprehensive search with full documentation
telespotter -s -n 15 2155551234

# Results saved with timestamp in filename
# Use for court documentation or case files
```

## Output Examples

### Successful Search
```
████████╗███████╗██╗     ███████╗███████╗██████╗  ██████╗ ████████╗
╚══██╔══╝██╔════╝██║     ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚══██╔══╝
   ██║   █████╗  ██║     █████╗  ███████╗██████╔╝██║   ██║   ██║   
   ██║   ██╔══╝  ██║     ██╔══╝  ╚════██║██╔═══╝ ██║   ██║   ██║   
   ██║   ███████╗███████╗███████╗███████║██║     ╚██████╔╝   ██║   
   ╚═╝   ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝      ╚═════╝    ╚═╝   
                                                         version 2.0

Generating search formats...
Generated 4 search format variations

[1/4] Searching: 555-555-1212
  → Searching Google... (8 results)
  → Searching Bing... (5 results)
  → Searching DuckDuckGo... (4 results)
  ✓ Total: 17 results for this format
  ⏳ Waiting 3 seconds...

[2/4] Searching: (555) 555-1212
  → Searching Google... (7 results)
  → Searching Bing... (6 results)
  → Searching DuckDuckGo... (3 results)
  ✓ Total: 16 results for this format
  ⏳ Waiting 3 seconds...

[3/4] Searching: 5555551212
  → Searching Google... (6 results)
  → Searching Bing... (4 results)
  → Searching DuckDuckGo... (5 results)
  ✓ Total: 15 results for this format
  ⏳ Waiting 3 seconds...

[4/4] Searching: 1 555-555-1212
  → Searching Google... (5 results)
  → Searching Bing... (3 results)
  → Searching DuckDuckGo... (4 results)
  ✓ Total: 12 results for this format

Analyzing patterns across all results...

================================================================================
PATTERN ANALYSIS SUMMARY
================================================================================

Total Results Found: 60

Results by Source:
  • Google: 26 results
  • Bing: 18 results
  • DuckDuckGo: 16 results

📛 Names Found:
  • John Smith: mentioned 12 time(s)
  • Jane Doe: mentioned 8 time(s)
  • Robert Johnson: mentioned 5 time(s)
  • Mary Williams: mentioned 3 time(s)

📍 Locations Mentioned:
  • Philadelphia, PA: 18 occurrence(s)
  • PA: 15 occurrence(s)
  • 19102: 8 occurrence(s)
  • Pennsylvania: 6 occurrence(s)

🔍 Key Insights:
  • Most associated name: John Smith
  • Most associated location: Philadelphia, PA

================================================================================

Save detailed results to file? (y/n): y
Results saved to: telespotter_results_5555551212.json
```

### No Results Found
```
================================================================================
PATTERN ANALYSIS SUMMARY
================================================================================

Total Results Found: 0

No names detected in search results

No locations detected in search results

🔍 Key Insights:
  • No results found for this phone number

================================================================================
```

## JSON Output Structure

```json
{
  "phone_number": "5555551212",
  "search_formats": [
    "555-555-1212",
    "(555) 555-1212",
    "5555551212",
    "1 555-555-1212"
  ],
  "results": {
    "555-555-1212": [
      {
        "title": "John Smith - Philadelphia Area",
        "snippet": "Contact John Smith at 555-555-1212 in Philadelphia, PA",
        "source": "Google"
      },
      {
        "title": "Business Listing",
        "snippet": "555-555-1212 - Philadelphia, PA 19102",
        "source": "Bing"
      }
    ]
  },
  "pattern_analysis": {
    "total_results": 60,
    "results_by_source": {
      "Google": 26,
      "Bing": 18,
      "DuckDuckGo": 16
    },
    "common_names": [
      ["John Smith", 12],
      ["Jane Doe", 8],
      ["Robert Johnson", 5]
    ],
    "common_locations": [
      ["Philadelphia, PA", 18],
      ["PA", 15],
      ["19102", 8]
    ]
  }
}
```

## Scripting and Automation

### Check if number has results (exit code)
```bash
#!/bin/bash
telespotter -s 5555551212 > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "Search completed successfully"
else
    echo "Search failed"
fi
```

### Parse JSON output in script
```bash
#!/bin/bash
telespotter -s 5555551212

# Extract data with jq
TOTAL=$(jq '.pattern_analysis.total_results' telespotter_results_5555551212.json)
echo "Found $TOTAL results"

NAMES=$(jq -r '.pattern_analysis.common_names[0][0]' telespotter_results_5555551212.json)
echo "Top name: $NAMES"
```

### Integration with other tools
```bash
# Combine with other OSINT tools
telespotter -s 2155551234
sherlock john_smith  # Username search
theHarvester -d example.com -b google
```

## Performance Benchmarks

### Small search (5 results per engine)
```bash
time telespotter 5555551212
# Real: ~18 seconds
# User: ~0.5 seconds
# Sys: ~0.1 seconds
```

### Large search (15 results per engine)
```bash
time telespotter -n 15 5555551212
# Real: ~35 seconds
# User: ~1.2 seconds
# Sys: ~0.2 seconds
```

## Error Handling Examples

### Rate Limited
```bash
telespotter -d 5555551212
# Output shows:
# → Searching Google... Error: Too Many Requests
# → Searching Bing... (5 results)
# Solution: Wait 10-15 minutes before retrying
```

### Network Issues
```bash
telespotter -d 5555551212
# Output shows:
# → Searching Google... Error: connection timeout
# Solution: Check internet connection
```

### Invalid Phone Number
```bash
telespotter 123
# Output:
# Error: Phone number must be 10 or 11 digits
```

## Advanced Usage

### Custom user agent (requires code modification)
Edit `src/search.rs`:
```rust
pub fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("YourCustomAgent/1.0")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}
```

### Adjust rate limiting (requires code modification)
Edit `src/main.rs`:
```rust
// Change from 1 second to 2 seconds between engines
sleep(Duration::from_secs(2)).await;

// Change from 3 seconds to 5 seconds between formats
sleep(Duration::from_secs(5)).await;
```

## Tips and Tricks

### 1. Use debug mode for troubleshooting
```bash
telespotter -d 5555551212
```

### 2. Save results automatically for documentation
```bash
telespotter -s 5555551212
```

### 3. Increase results for thorough investigations
```bash
telespotter -n 15 5555551212
```

### 4. Respect rate limits
Wait at least 15 minutes between searches to avoid being blocked.

### 5. Use VPN for privacy
```bash
# Connect to VPN first
sudo openvpn config.ovpn
# Then search
telespotter 5555551212
```

### 6. Batch process with delays
```bash
for num in $(cat numbers.txt); do
    telespotter -s "$num"
    sleep 900  # 15 minutes between searches
done
```

## Help and Support

### View all options
```bash
telespotter --help
```

### Check version
```bash
telespotter --version
```

### Report issues
Open an issue on GitHub with:
- Command you ran
- Error message
- Output from debug mode (`-d`)
- Your OS and Rust version

---

**More questions?** Check the [README](README.md) or [open an issue](https://github.com/thumpersecure/telespotter-rust/issues)!
