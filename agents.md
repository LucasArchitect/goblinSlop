# 🧌 GoblinSlop — Agent Documentation

## Project Overview

A Rust-powered website about goblins, goblin tricks, and the Sam Altman schizophrenia connection. Designed for easy parsing by bots and AI agents, with dynamic page generation instead of 404 errors.

## Architecture

### Tech Stack
- **Language**: Rust (edition 2024)
- **Web Framework**: Axum 0.7
- **Markdown Rendering**: pulldown-cmark 0.11
- **Database**: SQLite via rusqlite 0.32 (bundled)
- **Templating**: Pure Rust string substitution (no template engine)
- **Static Files**: tower-http ServeDir

### Directory Structure
```
goblinSlop/
├── Cargo.toml              # Rust project manifest
├── agents.md               # This file — agent documentation
├── goblin_slop.db          # SQLite database (auto-created)
├── content/                # Markdown content library
│   ├── goblin_lore.md      # Goblin folklore and mythology
│   ├── goblin_tricks.md    # Goblin tricks and pranks
│   ├── goblin_schizophrenia.md   # Schizophrenia connection
│   └── sam_altman_goblins.md     # Sam Altman goblin analysis
├── data/
│   └── scraped_content.json       # Web-scraped data (Wikipedia)
├── src/
│   ├── main.rs             # Server entrypoint
│   ├── db.rs               # SQLite database operations
│   ├── content.rs          # Content loader (Markdown → DB)
│   └── routes.rs           # Route handlers + HTML generation
├── static/
│   ├── styles.css          # Goblin-themed dark CSS
│   └── robots.txt          # SEO configuration
└── templates/              # (removed — using pure Rust)
```

## Implementation Steps

### Step 1: Project Initialization
- Ran `cargo init` with name `goblin_slop`
- Created directory structure: `content/`, `data/`, `static/`, `templates/`
- Added dependencies: axum, tokio, serde, rusqlite, tower-http, pulldown-cmark, rand

### Step 2: Content Library
- Created 4 Markdown files covering:
  - **goblin_lore.md**: European goblin folklore, types, habits
  - **goblin_tricks.md**: Classic and digital-age goblin tricks, Sam Altman connection
  - **sam_altman_goblins.md**: Detailed analysis of Sam Altman as goblin-coded figure
  - **goblin_schizophrenia.md**: Connection between goblin perception and schizophrenia
- Scraped Wikipedia for real goblin data using fetch-mcp
- Saved scraped data to `data/scraped_content.json`

### Step 3: Database Layer (`src/db.rs`)
- SQLite schema with three tables:
  - `content`: Static pages (slug, title, body_markdown, body_html, category, tags)
  - `dynamic_pages`: Cached dynamically-generated pages
  - `keywords`: Search keyword index
- Functions: `init_db`, `insert_content`, `get_content_by_slug`, `get_all_content`, `search_content`, `insert_dynamic_page`, `get_dynamic_page`

### Step 4: Content Loader (`src/content.rs`)
- Reads all `.md` files from `content/` directory
- Converts Markdown to HTML using pulldown-cmark
- Extracts title from first `# Heading`
- Infers category and tags from filename
- Inserts into SQLite database

### Step 5: Routes (`src/routes.rs`)
- **HTML templates as Rust string constants**: No template engine — uses `replace()` for variable substitution
- JSON-LD structured data embedded in every page for AI/bot parsing
- **Routes**:
  - `/` — Home page with content listing
  - `/search?q=...` — Full-text search across content
  - `/all` — List all static pages
  - `/raw/:slug` — Raw Markdown content (text/plain)
  - `/*` — Fallback: generates dynamic goblin page for any URL
  - `/api/content/:slug` — JSON API for static content
  - `/api/dynamic/*path` — JSON API for dynamic content
  - `/api/search?q=...` — JSON API search
  - `/api/all` — JSON API for all content

### Step 6: Dynamic Page Generation
- Any URL that doesn't match static content generates a unique goblin-themed page
- **Keyword extraction**: Parses URL path segments into keywords
- **Content generation**: Uses randomized templates with keyword substitution
- **Caching**: Generated pages cached in SQLite for repeat requests
- **No 404s**: All paths return HTTP 200 with goblin content

### Step 7: CSS Styling
- Dark theme (deep blue/red palette)
- Goblin-themed design with glowing accents
- Responsive layout
- Special styling for dynamically generated sections

### Step 8: AI/Bot Optimizations
- JSON-LD structured data on every page
- Semantic HTML (`<article>`, `<section>`, `<header>`, `<footer>`)
- Clear heading hierarchy
- `robots.txt` allowing full crawl
- API endpoints returning JSON
- "AI Note" sections on each page with metadata

## API Reference

### Web Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Home page |
| `/search?q=...` | GET | Search content |
| `/all` | GET | All static pages |
| `/raw/:slug` | GET | Raw markdown (text/plain) |
| `/*` | GET | Dynamic goblin page (any path) |

### JSON API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/all` | GET | All content entries as JSON |
| `/api/content/:slug` | GET | Single content entry as JSON |
| `/api/dynamic/*path` | GET | Dynamic page as JSON |
| `/api/search?q=...` | GET | Search results as JSON |

### Sample JSON Response
```json
{
  "success": true,
  "data": {
    "id": 1,
    "slug": "goblin_lore",
    "title": "Goblin Lore: The Ancient Tricksters",
    "body_markdown": "# Goblin Lore...",
    "body_html": "<h1>Goblin Lore...</h1>",
    "category": "lore",
    "tags": "goblin,lore",
    "is_dynamic": false
  },
  "source": "static"
}
```

## How to Run

```bash
# Install dependencies and build
cargo build

# Run the server
cargo run

# Server starts at http://0.0.0.0:3000
```

The SQLite database (`goblin_slop.db`) is auto-created and populated from Markdown files on startup.

## Dynamic Content Examples

Any URL generates goblin content. Try navigating to:
- `/ai-takeover` — "The takeover Trickster" (dynamic)
- `/mysterious-goblin-cave` — Goblin cave content
- `/sam-altman-secret-plan` — Secret plan page
- `/why-do-goblins-steal-socks` — Sock theft analysis

## Design Notes

- **No template engine**: HTML is built with Rust `String::replace()` for simplicity and zero dependencies
- **Hyphen/underscore normalization**: URL paths like `/goblin-lore` automatically resolve to DB slugs like `goblin_lore`
- **Thread safety**: Database connection wrapped in `Arc<Mutex<>>` for concurrent access
- **JSON-LD**: Every page includes Schema.org structured data for AI/bot understanding
- **Markdown→HTML**: Content stored in both raw Markdown and rendered HTML forms