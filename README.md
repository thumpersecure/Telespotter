# 📞 TeleSpotter 🔍

```
████████╗███████╗██╗     ███████╗███████╗██████╗  ██████╗ ████████╗████████╗███████╗██████╗
╚══██╔══╝██╔════╝██║     ██╔════╝██╔════╝██╔══██╗██╔═══██╗╚══██╔══╝╚══██╔══╝██╔════╝██╔══██╗
   ██║   █████╗  ██║     █████╗  ███████╗██████╔╝██║   ██║   ██║      ██║   █████╗  ██████╔╝
   ██║   ██╔══╝  ██║     ██╔══╝  ╚════██║██╔═══╝ ██║   ██║   ██║      ██║   ██╔══╝  ██╔══██╗
   ██║   ███████╗███████╗███████╗███████║██║     ╚██████╔╝   ██║      ██║   ███████╗██║  ██║
   ╚═╝   ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝      ╚═════╝    ╚═╝      ╚═╝   ╚══════╝╚═╝  ╚═╝
                                                                              version 2.1
```

[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/Version-2.1.0-blue)](https://github.com/thumpersecure/telespotter)

> 🚀 **A blazingly fast phone number OSINT tool written in Rust** — Search across multiple engines and people lookup sites to gather intelligence on any phone number.

---

## 🌟 What's New in v2.1

- 🎭 **Random User Agent Rotation** — Avoid detection with 15 different browser signatures
- 🔍 **People Search Sites** — Search Whitepages, TruePeopleSearch, FastPeopleSearch, ThatsThem & USPhoneBook
- 📧 **Email Extraction** — Automatically find associated email addresses
- 👤 **Username Detection** — Find social media handles from results
- 🔗 **OSINT Tool Integration** — Auto-prompt for Sherlock, Blackbird & email2phonenumber

---

## ✨ Features

### 🔎 Multi-Engine Search
| Engine | Description |
|--------|-------------|
| 🔵 **Google** | Web scraping with retry logic |
| 🟢 **Bing** | Microsoft search engine |
| 🦆 **DuckDuckGo** | Privacy-focused search |

### 🏠 People Search Sites
| Site | What It Finds |
|------|---------------|
| 📖 **Whitepages** | Names, addresses, phone records |
| 👥 **TruePeopleSearch** | Owner info, relatives, associates |
| ⚡ **FastPeopleSearch** | Quick lookups with age info |
| 🎯 **ThatsThem** | Comprehensive people data |
| 📱 **USPhoneBook** | Phone carrier & owner details |

### 📊 Pattern Analysis
- 📛 **Names** — People associated with the number
- 📍 **Locations** — Cities, states, ZIP codes
- 📧 **Emails** — Associated email addresses
- 👤 **Usernames** — Social media handles (@mentions)
- 🔗 **Social Profiles** — Extracted from URLs

### 🛡️ Anti-Detection
- 🎭 **15 User Agents** — Chrome, Firefox, Safari, Edge on Windows/macOS/Linux
- ⏱️ **Rate Limiting** — Configurable delays between requests
- 🔄 **Retry Logic** — Automatic retries with exponential backoff

### 🔧 OSINT Integration
- 🔎 **Sherlock** — Find usernames across 400+ social networks
- 🐦 **Blackbird** — Search emails across platforms
- 📱 **email2phonenumber** — Reverse lookup emails to phones

---

## 📦 Installation

### Quick Start 🚀

```bash
# Clone the repo
git clone https://github.com/thumpersecure/telespotter.git
cd telespotter

# Build optimized release
cargo build --release

# Run it!
./target/release/telespotter --help
```

### Install System-Wide 🌍

```bash
cargo install --path .
telespotter --help
```

### Prerequisites 📋

- 🦀 **Rust 1.70+** — Install from [rustup.rs](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🚀 Usage

### Basic Search

```bash
# Interactive mode (prompts for number)
telespotter

# Direct number input
telespotter 5551234567
telespotter "(555) 123-4567"
telespotter 1-555-123-4567
```

### 🎯 Quick Examples

```bash
# 🔥 Full OSINT scan with everything enabled
telespotter 5551234567 -p --random-ua -c -s

# 🔍 Search with people lookup sites
telespotter 5551234567 -p

# 🎭 Use random user agents to avoid detection
telespotter 5551234567 --random-ua

# ⚡ Fast concurrent mode
telespotter 5551234567 -c

# 💾 Auto-save results to JSON
telespotter 5551234567 -s

# 🐛 Debug mode for troubleshooting
telespotter 5551234567 -d
```

### 🏠 People Search Options

```bash
# Search ALL people lookup sites
telespotter 5551234567 -p

# Search specific sites only
telespotter 5551234567 -p --whitepages
telespotter 5551234567 -p --truepeoplesearch
telespotter 5551234567 -p --fastpeoplesearch
telespotter 5551234567 -p --thatsthem
telespotter 5551234567 -p --usphonebook

# Combine multiple sites
telespotter 5551234567 -p --whitepages --thatsthem
```

### 🔎 OSINT Tool Integration

```bash
# Auto-run Sherlock on found usernames
telespotter 5551234567 --sherlock

# Auto-run Blackbird on found emails
telespotter 5551234567 --blackbird

# Run email2phonenumber reverse lookup
telespotter 5551234567 --email2phone

# Skip OSINT prompts (for scripting)
telespotter 5551234567 --no-osint-prompts
```

### 🛠️ Advanced Options

```bash
# Custom number of results per engine (default: 5)
telespotter 5551234567 -n 10

# Adjust timeout (seconds)
telespotter 5551234567 -t 30

# Custom delay between requests (seconds)
telespotter 5551234567 --delay 2

# Select specific search engines
telespotter 5551234567 -e google
telespotter 5551234567 -e google -e bing

# Output formats
telespotter 5551234567 -s -f json    # JSON (default)
telespotter 5551234567 -s -f csv     # CSV format
telespotter 5551234567 -s -f txt     # Plain text

# Custom output file
telespotter 5551234567 -s -o results.json

# Limit analysis display
telespotter 5551234567 --max-names 5 --max-locations 5

# Quiet mode (minimal output)
telespotter 5551234567 -q

# No colors (for piping/logging)
telespotter 5551234567 --no-color
```

---

## 📋 All Command-Line Options

```
USAGE:
    telespotter [OPTIONS] [PHONE_NUMBER]

ARGUMENTS:
    [PHONE_NUMBER]    Phone number (digits only or formatted)

OPTIONS:
    -d, --debug                 🐛 Enable debug mode
    -n, --num-results <NUM>     🔢 Results per search engine [default: 5]
    -s, --save                  💾 Auto-save results to file
    -t, --timeout <SECS>        ⏱️  HTTP timeout in seconds [default: 10]
        --delay <SECS>          ⏳ Delay between requests [default: 1]
    -o, --output <FILE>         📁 Custom output file path
    -f, --format <FMT>          📄 Output format: json, csv, txt [default: json]
    -e, --engines <ENGINE>      🔍 Engines: google, bing, duckduckgo, all
    -q, --quiet                 🤫 Quiet mode (minimal output)
        --no-color              🎨 Disable colored output
        --max-names <NUM>       📛 Max names to display [default: 10]
        --max-locations <NUM>   📍 Max locations to display [default: 10]
        --max-emails <NUM>      📧 Max emails to display [default: 10]
        --max-usernames <NUM>   👤 Max usernames to display [default: 10]
    -c, --concurrent            ⚡ Concurrent searches (faster)
        --retries <NUM>         🔄 Retry attempts [default: 2]
        --random-ua             🎭 Random user agent rotation
    -p, --people-search         🏠 Search people lookup sites
        --whitepages            📖 Search Whitepages
        --truepeoplesearch      👥 Search TruePeopleSearch
        --fastpeoplesearch      ⚡ Search FastPeopleSearch
        --thatsthem             🎯 Search ThatsThem
        --usphonebook           📱 Search USPhoneBook
        --sherlock              🔎 Auto-run Sherlock on usernames
        --blackbird             🐦 Auto-run Blackbird on emails
        --email2phone           📱 Run email2phonenumber lookup
        --no-osint-prompts      🚫 Skip OSINT tool prompts
    -h, --help                  ❓ Print help
    -V, --version               📌 Print version
```

---

## 📊 Sample Output

```
================================================================================
PATTERN ANALYSIS SUMMARY
================================================================================

Total Results Found: 47

Results by Source:
  • Google: 15 results
  • Bing: 12 results
  • DuckDuckGo: 8 results
  • Whitepages: 4 results
  • TruePeopleSearch: 5 results
  • ThatsThem: 3 results

📛 Names Found:
  • John Smith: mentioned 8 time(s)
  • Jane Doe: mentioned 3 time(s)
  • Michael Johnson: mentioned 2 time(s)

📍 Locations Mentioned:
  • Philadelphia, PA: 12 occurrence(s)
  • PA: 8 occurrence(s)
  • 19102: 3 occurrence(s)

📧 Emails Found:
  • jsmith@email.com: 2 occurrence(s)
  • contact@business.com: 1 occurrence(s)

👤 Usernames/Social Media Found:
  • @johnsmith: 3 occurrence(s)
  • @jsmith2024: 1 occurrence(s)

🔍 Key Insights:
  • Most associated name: John Smith
  • Most associated location: Philadelphia, PA
================================================================================
```

---

## 🎯 Use Cases

| Use Case | Command Example |
|----------|-----------------|
| 🕵️ **Full OSINT Investigation** | `telespotter 5551234567 -p --random-ua -c -s --sherlock` |
| 🚫 **Spam Number Check** | `telespotter 5551234567 -p -s` |
| ✅ **Business Verification** | `telespotter 5551234567 -p --whitepages` |
| 🔎 **Skip Tracing** | `telespotter 5551234567 -p -n 10 --max-names 20` |
| ⚖️ **Legal Investigation** | `telespotter 5551234567 -p -s -f txt -o evidence.txt` |
| 🤖 **Automated/Scripted** | `telespotter 5551234567 -q --no-osint-prompts -s` |

---

## ⚡ Performance

### Rust vs Python Comparison

| Metric | Python | Rust | Improvement |
|--------|--------|------|-------------|
| ⏱️ Execution | 65s | 18s | **3.6x faster** |
| 💾 Memory | 48MB | 8MB | **6x less** |
| 📦 Binary | Needs Python | 4.2MB | **Single file** |
| 🚀 Startup | 800ms | 2ms | **400x faster** |

---

## 🔧 Development

### Project Structure

```
telespotter/
├── 📄 Cargo.toml           # Dependencies
├── 📄 main.rs              # Entry point & CLI
├── 📄 phone.rs             # Phone formatting
├── 📄 search.rs            # HTTP client & config
├── 📄 google.rs            # Google scraper
├── 📄 bing.rs              # Bing scraper
├── 📄 duckduckgo.rs        # DuckDuckGo scraper
├── 📄 whitepages.rs        # Whitepages scraper
├── 📄 truepeoplesearch.rs  # TruePeopleSearch scraper
├── 📄 fastpeoplesearch.rs  # FastPeopleSearch scraper
├── 📄 thatsthem.rs         # ThatsThem scraper
├── 📄 usphonebook.rs       # USPhoneBook scraper
├── 📄 parser.rs            # Pattern extraction
├── 📄 analysis.rs          # Results analysis
└── 📄 README.md
```

### Building & Testing

```bash
# 🔨 Build debug version
cargo build

# 🚀 Build optimized release
cargo build --release

# 🧪 Run tests
cargo test

# 📝 Format code
cargo fmt

# 🔍 Lint code
cargo clippy
```

---

## 🛡️ Optional OSINT Tools

For enhanced functionality, install these tools:

### Sherlock 🔎
```bash
pip install sherlock-project
```

### Blackbird 🐦
```bash
pip install blackbird
```

### email2phonenumber 📱
```bash
pip install email2phonenumber
```

---

## 🔒 Privacy & Legal

> ⚠️ **Important**: This tool is for **legitimate investigative purposes only**.

- ✅ Uses publicly available search data
- ✅ Respect privacy laws and regulations
- ✅ Obtain proper authorization when required
- ❌ Do not use for harassment or stalking
- ❌ Do not violate terms of service

---

## 🆘 Troubleshooting

### Common Issues

| Problem | Solution |
|---------|----------|
| 🔴 0 results | Try `-d` debug mode, check internet, wait 10-15 min |
| 🔴 Timeout errors | Increase with `-t 30`, check firewall |
| 🔴 Rate limited | Use `--delay 3` and `--random-ua` |
| 🔴 Build errors | Run `rustup update` and `cargo clean` |

### Getting Help

```bash
# Show all options
telespotter --help

# Debug mode for errors
telespotter -d 5551234567
```

---

## 📦 Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | ⚡ Async runtime |
| `reqwest` | 🌐 HTTP client |
| `scraper` | 📄 HTML parsing |
| `clap` | 🖥️ CLI parsing |
| `colored` | 🎨 Terminal colors |
| `serde` | 📦 Serialization |
| `regex` | 🔍 Pattern matching |
| `rand` | 🎲 Random selection |

---

## 👤 Author

Created by **Spin Apin** ([@thumpersecure](https://github.com/thumpersecure))

---

## 🤝 Contributing

- 🐛 [Report bugs](https://github.com/thumpersecure/telespotter/issues)
- 💡 Suggest features
- 🔧 Submit pull requests
- ⭐ Star if you find it useful!

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

<div align="center">

Made with 💻 and 🦀 for OSINT professionals

**⚡ Fast • 🔒 Safe • 🎯 Effective**

</div>
