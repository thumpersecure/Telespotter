<div align="center">

<!-- Animated Typing Header -->
<a href="https://github.com/thumpersecure/telespotter">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=28&duration=3000&pause=1000&color=00D4FF&center=true&vCenter=true&multiline=true&repeat=true&width=600&height=100&lines=%F0%9F%93%9E+TELESPOTTER+%F0%9F%94%8D;Phone+Number+OSINT+Tool;Written+in+Rust+%F0%9F%A6%80" alt="TeleSpotter" />
</a>

<!-- Compact ASCII Logo -->
```
  ______     __     ____                  __  __
 /_  __/__  / /__  / __/__  ___  ______  / /_/ /____  _____
  / / / _ \/ / _ \_\ \/ _ \/ _ \/ __/ / / __/ __/ _ \/ ___/
 / / /  __/ /  __/___/ .__/\___/\__/\__/\__/\__/\___/_/
/_/  \___/_/\___/   /_/                           v2.1.0
```

<!-- Blue Gradient Line -->
<img src="https://capsule-render.vercel.app/api?type=rect&color=gradient&customColorList=0,2,2,5,30&height=2&section=header" width="100%"/>

<!-- Badges -->
[![License](https://img.shields.io/badge/License-MIT-00d4ff?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-ff6b35?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/Version-2.1.0-7c3aed?style=for-the-badge)](https://github.com/thumpersecure/telespotter)
[![Stars](https://img.shields.io/github/stars/thumpersecure/telespotter?style=for-the-badge&color=fbbf24)](https://github.com/thumpersecure/telespotter)

**A blazingly fast phone number OSINT tool** — Search across multiple engines and people lookup sites to gather intelligence on any phone number.

<img src="https://capsule-render.vercel.app/api?type=rect&color=gradient&customColorList=0,2,2,5,30&height=2&section=header" width="100%"/>

</div>

## ⚡ Quick Start

```bash
git clone https://github.com/thumpersecure/telespotter.git && cd telespotter
cargo build --release
./target/release/telespotter 5551234567 -p --random-ua -c -s
```

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 🌟 What's New in v2.1

| Feature | Description |
|---------|-------------|
| 🎭 **Random User Agent** | 15 browser signatures to avoid detection |
| 🏠 **People Search** | Whitepages, TruePeopleSearch, FastPeopleSearch, ThatsThem, USPhoneBook |
| 📧 **Email Extraction** | Auto-find associated email addresses |
| 👤 **Username Detection** | Find social media handles |
| 🔗 **OSINT Integration** | Sherlock, Blackbird & email2phonenumber |
| 📍 **DC Support** | District of Columbia now recognized |
| 📊 **JSON Metadata** | Version & timestamp in output files |

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 🔍 Search Sources

<table>
<tr>
<td width="50%">

### 🌐 Search Engines
| Engine | Status |
|--------|--------|
| Google | ✅ |
| Bing | ✅ |
| DuckDuckGo | ✅ |

</td>
<td width="50%">

### 🏠 People Search
| Site | Status |
|------|--------|
| Whitepages | ✅ |
| TruePeopleSearch | ✅ |
| FastPeopleSearch | ✅ |
| ThatsThem | ✅ |
| USPhoneBook | ✅ |

</td>
</tr>
</table>

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 📊 What It Extracts

```
📛 Names         → People associated with the number
📍 Locations     → Cities, states (incl. DC), ZIP codes
📧 Emails        → Associated email addresses
👤 Usernames     → Social media handles (@mentions)
🔗 Social URLs   → Profile links from major platforms
```

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 🚀 Usage Examples

```bash
# Full OSINT scan
telespotter 5551234567 -p --random-ua -c -s

# Search specific people sites
telespotter 5551234567 -p --whitepages --thatsthem

# Auto-run Sherlock on usernames found
telespotter 5551234567 --sherlock

# Quiet mode for scripts
telespotter 5551234567 -q --no-osint-prompts -s

# Custom output limits
telespotter 5551234567 --max-names 20 --max-emails 15
```

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 📋 CLI Reference

```
USAGE: telespotter [OPTIONS] [PHONE_NUMBER]

CORE OPTIONS:
  -p, --people-search         Search people lookup sites
  -c, --concurrent            Fast parallel searches
  -s, --save                  Auto-save results
  -d, --debug                 Debug mode
  -q, --quiet                 Minimal output

SEARCH TUNING:
  -n, --num-results <N>       Results per engine [default: 5]
  -t, --timeout <SECS>        HTTP timeout [default: 10]
      --delay <SECS>          Rate limit delay [default: 1]
      --retries <N>           Retry attempts [default: 2]
      --random-ua             Random user agent rotation
  -e, --engines <ENGINE>      google, bing, duckduckgo, all

OUTPUT OPTIONS:
  -o, --output <FILE>         Custom output path
  -f, --format <FMT>          json, csv, txt [default: json]
      --no-color              Disable colors
      --max-names <N>         Name limit [default: 10]
      --max-locations <N>     Location limit [default: 10]
      --max-emails <N>        Email limit [default: 10]
      --max-usernames <N>     Username limit [default: 10]

PEOPLE SEARCH SITES:
      --whitepages            Whitepages only
      --truepeoplesearch      TruePeopleSearch only
      --fastpeoplesearch      FastPeopleSearch only
      --thatsthem             ThatsThem only
      --usphonebook           USPhoneBook only

OSINT TOOLS:
      --sherlock              Run Sherlock on usernames
      --blackbird             Run Blackbird on emails
      --email2phone           Run email2phonenumber
      --no-osint-prompts      Skip tool prompts
```

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## ⚡ Performance

| Metric | Python | Rust | Gain |
|--------|--------|------|------|
| Execution | 65s | 18s | **3.6x** |
| Memory | 48MB | 8MB | **6x** |
| Startup | 800ms | 2ms | **400x** |
| Binary | Interpreter | 4.2MB | Single file |

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 🛠️ Installation

### From Source
```bash
# Prerequisites: Rust 1.70+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
git clone https://github.com/thumpersecure/telespotter.git
cd telespotter
cargo build --release

# Install system-wide (optional)
cargo install --path .
```

### Optional OSINT Tools
```bash
pip install sherlock-project    # Username search
pip install blackbird           # Email search
pip install email2phonenumber   # Reverse lookup
```

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 📁 Project Structure

```
telespotter/
├── main.rs              # CLI & orchestration
├── search.rs            # HTTP client, user agents
├── phone.rs             # Phone formatting
├── parser.rs            # Pattern extraction (names, emails, etc.)
├── analysis.rs          # Results analysis
├── google.rs            # Google scraper
├── bing.rs              # Bing scraper
├── duckduckgo.rs        # DuckDuckGo scraper
├── whitepages.rs        # Whitepages scraper
├── truepeoplesearch.rs  # TruePeopleSearch scraper
├── fastpeoplesearch.rs  # FastPeopleSearch scraper
├── thatsthem.rs         # ThatsThem scraper
└── usphonebook.rs       # USPhoneBook scraper
```

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 🔒 Legal Notice

> **For legitimate investigative purposes only.**

- ✅ Uses publicly available data
- ✅ Respect privacy laws
- ❌ No harassment or stalking
- ❌ No ToS violations

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

## 🆘 Troubleshooting

| Issue | Solution |
|-------|----------|
| 0 results | Use `-d` debug, check connection |
| Timeouts | Increase `-t 30` |
| Rate limited | Use `--delay 3 --random-ua` |
| Build errors | `rustup update && cargo clean` |

<img src="https://capsule-render.vercel.app/api?type=rect&color=0:00d4ff,100:7c3aed&height=1" width="100%"/>

<div align="center">

**Created by [Spin Apin](https://github.com/thumpersecure)**

[![GitHub](https://img.shields.io/badge/GitHub-thumpersecure-181717?style=for-the-badge&logo=github)](https://github.com/thumpersecure)

<sub>Made with 🦀 Rust for OSINT professionals</sub>

**⚡ Fast • 🔒 Safe • 🎯 Effective**

<img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=0,2,2,5,30&height=100&section=footer"/>

</div>
