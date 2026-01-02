# 🔑 Google Custom Search API Setup Guide

TeleSpotter supports two modes for Google searches:

1. **Web Scraping** (default) - No API key required, but may be rate-limited
2. **Google Custom Search API** (recommended) - More reliable, requires API key

This guide shows you how to set up the Google Custom Search API.

---

## 📋 Prerequisites

- Google account
- ~10 minutes to set up
- Free tier: 100 searches/day

---

## 🚀 Step-by-Step Setup

### Step 1: Get Your Google API Key

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)

2. Create a new project (or select an existing one):
   - Click "Select a project" at the top
   - Click "NEW PROJECT"
   - Name it (e.g., "TeleSpotter")
   - Click "CREATE"

3. Enable the Custom Search API:
   - Go to [APIs & Services > Library](https://console.cloud.google.com/apis/library)
   - Search for "Custom Search API"
   - Click "Custom Search API"
   - Click "ENABLE"

4. Create credentials:
   - Go to [APIs & Services > Credentials](https://console.cloud.google.com/apis/credentials)
   - Click "CREATE CREDENTIALS"
   - Select "API key"
   - **Copy your API key** (it looks like: `AIzaSyD...`)
   - Optionally, click "RESTRICT KEY" to limit it to Custom Search API only (recommended)

### Step 2: Create a Custom Search Engine

1. Go to [Google Programmable Search Engine](https://programmablesearchengine.google.com/)

2. Click "Add" or "Get started"

3. Configure your search engine:
   - **Sites to search**: Enter `*.com` (or leave blank for entire web)
   - **Name**: `TeleSpotter Search` (or any name you want)
   - Click "Create"

4. Get your Search Engine ID:
   - Click on your newly created search engine
   - Click "Setup" or "Edit"
   - Find "Search engine ID" (looks like: `a1b2c3d4e5f6g7h8i9`)
   - **Copy this ID**

5. Enable "Search the entire web":
   - In the setup page, toggle "Search the entire web" to ON
   - Click "Update"

---

## 🔧 Configuration Methods

TeleSpotter checks for API credentials in three ways (in order):

### Method 1: Environment Variables (Recommended)

**Linux/macOS:**
```bash
export GOOGLE_API_KEY="AIzaSyD..."
export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."

# Run TeleSpotter
telespotter 5555551212
```

**To make it permanent (Linux/macOS):**
```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'export GOOGLE_API_KEY="AIzaSyD..."' >> ~/.bashrc
echo 'export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."' >> ~/.bashrc
source ~/.bashrc
```

**Windows (PowerShell):**
```powershell
$env:GOOGLE_API_KEY="AIzaSyD..."
$env:GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."

# Run TeleSpotter
.\telespotter.exe 5555551212
```

**To make it permanent (Windows):**
- Right-click "This PC" → Properties
- Advanced system settings
- Environment Variables
- Add new User variables:
  - `GOOGLE_API_KEY` = `AIzaSyD...`
  - `GOOGLE_SEARCH_ENGINE_ID` = `a1b2c3d4e...`

### Method 2: .env File (Convenient for Development)

Create a `.env` file in the telespotter directory:

```bash
# .env file
GOOGLE_API_KEY=AIzaSyD...
GOOGLE_SEARCH_ENGINE_ID=a1b2c3d4e...
```

Then run with:
```bash
source .env  # Linux/macOS
telespotter 5555551212
```

### Method 3: Config File (Future Enhancement)

*Note: This would require adding a config.toml file reader to the code*

---

## ✅ Verifying Your Setup

### Test if API is being used:

```bash
# Set your credentials
export GOOGLE_API_KEY="your_key_here"
export GOOGLE_SEARCH_ENGINE_ID="your_cx_here"

# Run with debug mode
telespotter -d 5555551212
```

If the API is working, you should see:
- More consistent results
- No CAPTCHA errors
- Faster responses

If API credentials are not found, TeleSpotter will automatically fall back to web scraping (and you'll see this in debug mode).

---

## 💰 Pricing & Limits

### Free Tier
- **100 search queries per day** (FREE)
- Resets at midnight Pacific Time
- No credit card required

### Paid Tier
- $5 per 1000 queries (up to 10k queries/day)
- Enable billing in Google Cloud Console
- [Pricing details](https://developers.google.com/custom-search/v1/overview#pricing)

### For TeleSpotter Usage
- Each phone number search uses **4 queries** (4 formats)
- Free tier = **25 phone numbers per day**
- If you exceed the limit, TeleSpotter falls back to web scraping

---

## 🔒 Security Best Practices

### API Key Security

**DO:**
✅ Use environment variables
✅ Restrict API key to Custom Search API only
✅ Add API key restrictions in Google Cloud Console
✅ Rotate keys periodically

**DON'T:**
❌ Commit API keys to Git repositories
❌ Share API keys publicly
❌ Hardcode keys in source code
❌ Use keys in client-side applications

### Restrict Your API Key

In [Google Cloud Console Credentials](https://console.cloud.google.com/apis/credentials):

1. Click on your API key
2. Under "API restrictions":
   - Select "Restrict key"
   - Check "Custom Search API"
3. Click "Save"

---

## 🐛 Troubleshooting

### "API key not found" - Using web scraping
**Solution**: Set environment variables correctly
```bash
export GOOGLE_API_KEY="your_key"
export GOOGLE_SEARCH_ENGINE_ID="your_cx"
```

### "API returned 403: Forbidden"
**Causes**:
- API key is invalid
- Custom Search API is not enabled
- API key restrictions are too strict

**Solution**:
1. Verify API key is correct
2. Enable Custom Search API in Cloud Console
3. Check API key restrictions

### "API returned 429: Too Many Requests"
**Cause**: Exceeded daily quota (100 free queries)

**Solutions**:
- Wait until tomorrow (resets at midnight PT)
- TeleSpotter will automatically fall back to web scraping
- Enable billing for more queries

### No results from API
**Causes**:
- Search Engine ID is incorrect
- "Search the entire web" is not enabled

**Solution**:
1. Verify Search Engine ID
2. In Programmable Search settings, enable "Search the entire web"

### API works but getting fewer results than web scraping
**Cause**: Google Custom Search API returns max 10 results per request

**Note**: This is a Google API limitation, not a TeleSpotter issue

---

## 📊 Comparison: API vs Web Scraping

| Feature | Web Scraping | Google API |
|---------|--------------|------------|
| **Setup** | None | API key required |
| **Cost** | Free | 100/day free, then $5/1000 |
| **Reliability** | May be blocked | Very reliable |
| **Rate Limits** | Unpredictable | 100/day (free) |
| **Results Quality** | Good | Excellent |
| **Speed** | Fast | Very fast |
| **CAPTCHAs** | Possible | Never |

---

## 🔄 Switching Between Methods

TeleSpotter automatically switches based on environment variables:

```bash
# Use API
export GOOGLE_API_KEY="..."
export GOOGLE_SEARCH_ENGINE_ID="..."
telespotter 5555551212

# Use web scraping (unset variables)
unset GOOGLE_API_KEY
unset GOOGLE_SEARCH_ENGINE_ID
telespotter 5555551212
```

---

## 📝 Example Usage Script

Save this as `search_with_api.sh`:

```bash
#!/bin/bash
# TeleSpotter with Google API

# Set your credentials
export GOOGLE_API_KEY="AIzaSyD..."
export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."

# Run TeleSpotter
telespotter -s -n 10 "$1"

echo ""
echo "API searches used: 4 (for this phone number)"
echo "Remaining today: $((100 - searches_used))"
```

Usage:
```bash
chmod +x search_with_api.sh
./search_with_api.sh 5555551212
```

---

## 🎯 Recommendations

### For Personal Use (< 25 searches/day)
→ Use the **free API tier**
- Reliable results
- No CAPTCHAs
- Better quality

### For Heavy Use (> 25 searches/day)
→ Options:
1. Enable billing for more API quota
2. Use web scraping (free but may be rate-limited)
3. Combine both: API during work hours, scraping for overflow

### For Professional/Legal Work
→ Use the **paid API tier**
- Guaranteed availability
- Professional reliability
- Consistent results for documentation

---

## 📚 Additional Resources

- [Google Custom Search API Documentation](https://developers.google.com/custom-search/v1/overview)
- [Programmable Search Engine Help](https://support.google.com/programmable-search)
- [API Key Best Practices](https://cloud.google.com/docs/authentication/api-keys)
- [Google Cloud Console](https://console.cloud.google.com/)

---

## 🆘 Still Having Issues?

1. Check the [Troubleshooting](#-troubleshooting) section above
2. Run with debug mode: `telespotter -d 5555551212`
3. Verify your API key at: https://console.cloud.google.com/apis/credentials
4. Test your API directly: https://developers.google.com/custom-search/v1/using_rest

---

**Remember**: TeleSpotter works perfectly fine without an API key! The web scraping fallback is automatic and seamless. The API just makes it more reliable and faster for heavy usage.

Happy searching! 🔍
