# 🎯 TeleSpotter - Quick API Reference

## Two Ways to Use TeleSpotter

### 1. Web Scraping (Default - No Setup)
```bash
telespotter 5555551212
```
- ✅ No API key needed
- ✅ Works immediately
- ⚠️ May be rate-limited
- ⚠️ Possible CAPTCHAs

### 2. Google API (Recommended - Better Results)
```bash
export GOOGLE_API_KEY="AIzaSyD..."
export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."
telespotter 5555551212
```
- ✅ 100 free searches/day
- ✅ More reliable
- ✅ No CAPTCHAs
- ✅ Faster

---

## Quick Setup (5 Minutes)

### Step 1: Get API Key
1. Go to https://console.cloud.google.com/
2. Create/select project
3. Enable "Custom Search API"
4. Create credentials → API key
5. Copy your key: `AIzaSyD...`

### Step 2: Get Search Engine ID
1. Go to https://programmablesearchengine.google.com/
2. Create new search engine
3. Enable "Search the entire web"
4. Copy your ID: `a1b2c3d4e...`

### Step 3: Use It
```bash
export GOOGLE_API_KEY="AIzaSyD..."
export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."
telespotter 5555551212
```

**Full guide:** See `GOOGLE_API_SETUP.md`

---

## Make It Permanent

### Linux/macOS
```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'export GOOGLE_API_KEY="AIzaSyD..."' >> ~/.bashrc
echo 'export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."' >> ~/.bashrc
source ~/.bashrc
```

### Windows
1. Search for "Environment Variables"
2. Add User Variables:
   - `GOOGLE_API_KEY` = `AIzaSyD...`
   - `GOOGLE_SEARCH_ENGINE_ID` = `a1b2c3d4e...`

---

## How TeleSpotter Decides

```
┌─────────────────────────────────┐
│ Check for GOOGLE_API_KEY        │
│ and GOOGLE_SEARCH_ENGINE_ID     │
└──────────┬──────────────────────┘
           │
     ┌─────┴─────┐
     │   Found?  │
     └─────┬─────┘
           │
    ┌──────┴──────┐
    │             │
   Yes           No
    │             │
    ▼             ▼
┌───────┐    ┌─────────┐
│  API  │    │   Web   │
│ Mode  │    │ Scraping│
└───────┘    └─────────┘
```

**It's automatic!** No flags or configuration needed.

---

## Checking Which Mode You're Using

```bash
telespotter -d 5555551212
```

Look for:
- **API Mode**: "Using Google Custom Search API"
- **Scraping Mode**: No message (default behavior)

---

## Quota Management

### Free Tier (Default)
- 100 searches per day
- Resets at midnight Pacific Time
- 4 queries per phone number = **25 numbers/day**

### When You Hit the Limit
TeleSpotter **automatically switches** to web scraping!

### Need More?
Enable billing in Google Cloud Console:
- $5 per 1,000 queries
- Up to 10,000 queries/day

---

## Troubleshooting

### Not using API even with credentials set?
```bash
# Verify they're set
echo $GOOGLE_API_KEY
echo $GOOGLE_SEARCH_ENGINE_ID

# Re-export if empty
export GOOGLE_API_KEY="AIzaSyD..."
export GOOGLE_SEARCH_ENGINE_ID="a1b2c3d4e..."
```

### API returning errors?
```bash
# Check in debug mode
telespotter -d 5555551212
```

Common issues:
- Invalid API key → Regenerate in Cloud Console
- API not enabled → Enable "Custom Search API"
- Quota exceeded → Wait until tomorrow or enable billing

### Want to force web scraping?
```bash
# Temporarily unset
unset GOOGLE_API_KEY
unset GOOGLE_SEARCH_ENGINE_ID
telespotter 5555551212
```

---

## Best Practices

### ✅ DO
- Use environment variables
- Restrict API key to Custom Search API only
- Enable "Search the entire web" in search engine settings
- Keep API keys secret

### ❌ DON'T
- Commit API keys to Git
- Share API keys publicly
- Hardcode keys in scripts
- Use same key for multiple projects

---

## Cost Calculator

| Usage | Searches/Day | Phone #s/Day | Cost |
|-------|--------------|--------------|------|
| Light | 0-100 | 0-25 | **FREE** |
| Medium | 100-500 | 25-125 | $2/day |
| Heavy | 500-1000 | 125-250 | $4.50/day |

Each phone number = 4 searches (4 format variations)

---

## More Help

- **Detailed setup**: `GOOGLE_API_SETUP.md`
- **Main docs**: `README.md`
- **Examples**: `EXAMPLES.md`
- **Debug**: `telespotter -d 5555551212`

---

**Remember**: TeleSpotter works great without API! The web scraping is reliable for moderate use. API is just for heavy users who want guaranteed availability.

🔍 Happy searching!
