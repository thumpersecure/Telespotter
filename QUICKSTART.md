# TeleSpot Quick Start Guide

Get up and running with TeleSpot in under 5 minutes! 🚀

## Installation (Choose One Method)

### Method 1: Download Pre-built Binary (Easiest) ⚡

**Linux:**
```bash
wget https://github.com/thumpersecure/Telespot/releases/latest/download/telespot-linux-x64
chmod +x telespot-linux-x64
sudo mv telespot-linux-x64 /usr/local/bin/telespot
```

**macOS:**
```bash
curl -LO https://github.com/thumpersecure/Telespot/releases/latest/download/telespot-macos-x64
chmod +x telespot-macos-x64
sudo mv telespot-macos-x64 /usr/local/bin/telespot
```

**Windows:**
1. Download `telespot-windows-x64.exe` from [Releases](https://github.com/thumpersecure/Telespot/releases)
2. Move to a folder in your PATH or run directly

### Method 2: Build from Source 🔨

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/thumpersecure/Telespot.git
cd Telespot/rust
cargo build --release

# Binary at: target/release/telespot
```

## Basic Usage

### Interactive Mode
```bash
telespot
# Then enter phone number when prompted
```

### Command-Line Mode
```bash
telespot 2155551234
telespot "(215) 555-1234"
telespot 1-215-555-1234
```

### With Options
```bash
# Debug mode (see what's happening)
telespot --debug 2155551234

# More results per search engine
telespot --num-results 10 2155551234

# Both together
telespot -d -n 10 2155551234
```

## Understanding the Output

TeleSpot will show:

1. **Search Progress**: Live updates as it searches each format
2. **Pattern Analysis**: 
   - Total results found
   - Results per search engine
   - Names associated with the number
   - Locations mentioned
   - Key insights

3. **Option to Save**: JSON file with complete results

## Example Session

```bash
$ telespot 2155551234

████████╗███████╗██╗     ███████╗███████╗██████╗  ██████╗ ████████╗
[ASCII art continues...]

Generating search formats...
Generated 4 search format variations

[1/4] Searching: 215-555-1234
  → Searching Google... (5 results)
  → Searching Bing... (3 results)
  → Searching DuckDuckGo... (4 results)
  ✓ Total: 12 results for this format
  ⏳ Waiting 3 seconds...

[... continues for all formats ...]

Analyzing patterns across all results...

================================================================================
PATTERN ANALYSIS SUMMARY
================================================================================

Total Results Found: 42

Results by Source:
  • Google: 18 results
  • Bing: 15 results
  • DuckDuckGo: 9 results

📛 Names Found:
  • John Smith: mentioned 8 time(s)
  • Jane Doe: mentioned 3 time(s)

📍 Locations Mentioned:
  • Philadelphia, PA: 12 occurrence(s)
  • PA: 8 occurrence(s)

🔍 Key Insights:
  • Most associated name: John Smith
  • Most associated location: Philadelphia, PA

================================================================================

Save detailed results to file? (y/n):
```

## Common Scenarios

### Investigating Unknown Number
```bash
telespot 5555551212
# Review names and locations in output
# Save JSON for records
```

### Verifying Business Number
```bash
telespot --num-results 10 8001234567
# More results = better verification
```

### Bulk Investigation
```bash
# Save phone numbers in a file, then:
while read phone; do
  telespot "$phone" < /dev/null
  sleep 60  # Wait between searches
done < phone_numbers.txt
```

### Documenting for Legal Case
```bash
telespot -d 2155551234 | tee investigation_log.txt
# Saves complete output to file
# Include in case documentation
```

## Tips for Best Results

1. **Rate Limiting**: Tool waits between searches - be patient!
2. **Format Variation**: All formats are tried automatically
3. **Debug Mode**: Use `-d` if you get no results
4. **Save Results**: Always save JSON for records
5. **Multiple Runs**: If blocked, wait 10-15 minutes
6. **VPN**: Consider using if frequently blocked

## Troubleshooting

### No Results?
```bash
# Try debug mode
telespot --debug 2155551234

# Check internet connection
ping google.com

# Try a known business number
telespot 8004444444  # Example toll-free
```

### "Command not found"?
```bash
# If installed with cargo
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or run directly
./target/release/telespot
```

### Build Errors?
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Next Steps

- Read the full [README](README.md) for detailed info
- Check [CONTRIBUTING](CONTRIBUTING.md) if you want to contribute
- Report issues on [GitHub](https://github.com/thumpersecure/Telespot/issues)

## Need Help?

- Open an issue on GitHub
- Check existing issues for solutions
- Read the troubleshooting section in README

---

Happy investigating! 🔍
