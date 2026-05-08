# 🧌 GoblinSlop — Comprehensive Agent Documentation

> **Version**: 0.1.0  
> **Language**: Rust (edition 2024)  
> **Framework**: Axum 0.7  
> **Database**: SQLite (rusqlite 0.32 bundled)  
> **Purpose**: A website about goblins, goblin tricks, the Sam Altman schizophrenia connection, and goblins across anime, games, and pop culture. No 404s — every URL path leads somewhere goblin.

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Directory Structure](#directory-structure)
4. [Data Flow](#data-flow)
5. [Source Code Reference](#source-code-reference)
   - [5.1 `Cargo.toml` — Dependencies](#51-cargotoml--dependencies)
   - [5.2 `src/config.rs` — Configuration Module](#52-srcconfigrs--configuration-module)
   - [5.3 `src/main.rs` — Server Entrypoint](#53-srcmainrs--server-entrypoint)
   - [5.4 `src/db.rs` — Database Layer](#54-srcdbrs--database-layer)
   - [5.5 `src/content.rs` — Content Loader](#55-srccontentrs--content-loader)
   - [5.6 `src/data_loader.rs` — Scraped Content Loader](#56-srcdata_loaderrs--scraped-content-loader)
   - [5.7 `src/routes/` — Route Handlers Module Directory](#57-srcroutes--route-handlers-module-directory)
   - [5.8 `static/styles.css` — Styling](#58-staticstylescss--styling)
   - [5.9 `static/robots.txt` — SEO](#59-staticrobotstxt--seo)
   - [5.10 `content/*.md` — Content Library](#510-contentmd--content-library)
   - [5.11 `data/scraped_content.json` — Scraped Data](#511-datascraped_contentjson--scraped-data)
6. [API Reference](#api-reference)
7. [Dynamic Page Generation Algorithm](#dynamic-page-generation-algorithm)
8. [Testing & Verification](#testing--verification)
9. [Deployment](#deployment)
10. [Design Decisions & Trade-offs](#design-decisions--trade-offs)
11. [How to Extend](#how-to-extend)
12. [How to Run](#how-to-run)

---

## Project Overview

GoblinSlop is a Rust web server that serves three kinds of content:

1. **Static content** — Hand-crafted Markdown files about goblin lore, tricks, the Sam Altman/goblin connection, and the schizophrenia/perception connection, rendered to HTML.
2. **Scraped content** — Curated entries from MyAnimeList, VNDB, IMDb, Wikipedia, TV Tropes, D&D, Pathfinder, Warhammer, Warcraft, MTG, and other sources loaded from `data/scraped_content.json`.
3. **Dynamic content** — For ANY URL path that doesn't match existing content, the server generates a unique goblin-themed page on-the-fly using keywords extracted from the URL path. The user never sees any indication that pages are generated — everything looks equally authentic.

Key features:
- **No 404 errors** — every path returns HTTP 200 with goblin content
- **AI/Bot friendly** — JSON-LD structured data, semantic HTML, raw text endpoints, JSON API
- **Markdown source** — all static content authored in Markdown, auto-converted to HTML
- **Cached dynamics** — dynamically generated pages are cached in SQLite so repeat requests are fast
- **Pure Rust templating** — no template engine dependency; HTML is built with `String::replace()`
- **Routes as module directory** — handlers, templates, and generators are split across separate files

---

## Architecture

```
┌─────────────┐     ┌──────────────────┐     ┌──────────────┐
│   Browser   │────▶│   Axum Router    │────▶│  SQLite DB   │
│   / Bot     │     │  (src/routes/)   │     │ (goblin_slop.db)
└─────────────┘     └──────────────────┘     └──────────────┘
                           │                         ▲
                           │                         │
                           ▼                         │
                    ┌──────────────────┐     ┌──────────────┐
                    │  HTML Generator  │     │  Markdown    │
                    │ (string replace) │     │  Parser      │
                    └──────────────────┘     │ (pulldown-   │
                                             │  cmark)      │
                                             └──────────────┘
```

### Request Flow

1. Client sends HTTP GET to a path (e.g., `/goblin-lore` or `/some-random-path`)
2. Axum router matches the path against defined routes or the fallback handler
3. Fallback handler checks SQLite for static content matching the path (with hyphen→underscore normalization)
4. If static content found → render using `templates::render_content_page()` with the stored HTML
5. If not found → check `dynamic_pages` cache table
6. If cached → render using `templates::render_dynamic_page()`
7. If not cached → parse path into keywords, generate content using `generator::generate_dynamic_page_content()`, store in cache, render
8. All pages include JSON-LD structured data in the `<head>` for AI/bot parsing
9. **No "dynamically generated" badges, summon messages, or AI notes are shown to the user** — all pages appear equally authentic

### Thread Safety Model

The SQLite `Connection` is non-thread-safe, so it is wrapped in `Arc<Mutex<Connection>>` and cloned into every handler via Axum's `State` extractor. This means:
- Only one request at a time can access the database
- For a site like this with expected low traffic, this is acceptable
- For high traffic, a connection pool (like `r2d2`) would be needed

---

## Directory Structure

```
/home/azu/projects/goblinSlop/
├── agents.md                    # This file — complete documentation
├── Cargo.toml                   # Rust project manifest with dependencies
├── Cargo.lock                   # Dependency lock file (auto-generated)
├── goblin_slop.db               # SQLite database (auto-created on first run)
├── deploy.sh                    # Orchestrator: build → package → upload → install
├── content/                     # Markdown content library (source files)
│   ├── goblin_lore.md           #   Goblin folklore and mythology
│   ├── goblin_tricks.md         #   Goblin tricks and pranks
│   ├── goblin_schizophrenia.md  #   Schizophrenia/perception connection
│   └── sam_altman_goblins.md    #   Sam Altman goblin-coded analysis
├── data/                        # Scraped/gathered data
│   └── scraped_content.json     #   21 curated goblin entries from MAL, VNDB, IMDb, D&D, etc.
├── scripts/                     # Deployment and management scripts
│   ├── build.sh                 #   Build release binary
│   ├── package.sh               #   Package files into deploy tarball
│   ├── install.sh               #   Remote installation (runs on server)
│   ├── manage.sh                #   Server management utility
│   ├── logs.sh                  #   Nginx log viewer (tail, top IPs, slow requests, etc.)
│   └── goblinSlop.nginx.conf    #   Main nginx config with custom log format
├── src/                         # Rust source code
│   ├── config.rs                #   Configuration from environment variables
│   ├── main.rs                  #   Server entrypoint, startup logic
│   ├── db.rs                    #   SQLite schema, CRUD operations
│   ├── content.rs               #   Markdown→HTML loader
│   ├── data_loader.rs           #   JSON scraped content loader
│   └── routes/                  #   Route handlers module directory
│       ├── mod.rs               #     Module declaration, AppState, create_router()
│       ├── handlers.rs          #     All 9 route handler functions
│       ├── templates.rs         #     HTML template rendering functions
│       └── generator.rs         #     Dynamic goblin page content generation
├── static/                      # Static files served at /static/
│   ├── styles.css               #   Goblin-themed dark CSS
│   └── robots.txt               #   SEO/crawler instructions
└── target/                      # Compiled output (gitignored)
```

---

## Data Flow

### Database Schema

**Table: `content`**
| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER PK AUTO | Auto-incrementing ID |
| `slug` | TEXT UNIQUE | URL-friendly identifier (e.g., `goblin_lore`) |
| `title` | TEXT | Page title (extracted from first `# ` heading) |
| `body_markdown` | TEXT | Raw Markdown source |
| `body_html` | TEXT | Pre-rendered HTML from Markdown |
| `category` | TEXT | Inferred category (lore, tricks, sam_altman, schizophrenia, anime, pop_culture, ttrpg, games, visual_novels, linguistics, general) |
| `tags` | TEXT | Comma-separated tags (e.g., `goblin,lore`) |
| `is_dynamic` | INTEGER | Boolean: 0 = static; 1 = dynamic |

**Table: `dynamic_pages`**
| Column | Type | Description |
|--------|------|-------------|
| `path` | TEXT PK | The URL path (e.g., `ai-takeover-sam-altman`) |
| `title` | TEXT | Generated title |
| `content` | TEXT | Generated HTML content body |
| `keywords` | TEXT | Comma-separated keywords extracted from path |

**Table: `keywords`**
| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER PK AUTO | Auto-incrementing ID |
| `keyword` | TEXT UNIQUE | Individual keyword |
| `content_id` | INTEGER FK | References `content.id` |

### Startup Sequence

1. `main()` calls `db::init_db("goblin_slop.db")` which creates all three tables if they don't exist
2. `content::load_content_from_dir("goblin_slop.db", "content")` reads every `.md` file in `content/`:
   - Converts Markdown to HTML via `pulldown_cmark`
   - Extracts title from first `# ` heading
   - Infers category and tags from filename
   - Inserts/updates into `content` table
3. `data_loader::load_scraped_content("goblin_slop.db", "data")` reads `data/scraped_content.json`:
   - Each entry becomes a `ContentEntry` (same schema as markdown content)
   - Categories include: `anime`, `pop_culture`, `visual_novels`, `ttrpg`, `games`, `literature`, `linguistics`
4. Axum server starts listening on `0.0.0.0:3000`

---

## Source Code Reference

### 5.1 `Cargo.toml` — Dependencies

```toml
[package]
name = "goblin_slop"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.7"              # Web framework (async, extractors, routing)
tokio = { version = "1", features = ["full"] }   # Async runtime
serde = { version = "1", features = ["derive"] } # Serialization
serde_json = "1"          # JSON handling for API responses
rusqlite = { version = "0.32", features = ["bundled"] } # SQLite (bundled = no system dep)
tower-http = { version = "0.5", features = ["fs"] }     # Static file serving
pulldown-cmark = "0.11"   # Markdown→HTML parser
rand = "0.8"              # Random selection for dynamic content
tracing = "0.1"           # Logging
tracing-subscriber = "0.3" # Log output formatting
```

**Why these dependencies?**
- `bundled` flag on `rusqlite` means SQLite is compiled with the project — no system library needed

### 5.2 `src/config.rs` — Configuration Module

This module loads all runtime configuration from environment variables, following the [12-factor app](https://12factor.net/config) methodology.

**Struct: `Config`**

| Field | Type | Env Variable | Default | Description |
|-------|------|-------------|---------|-------------|
| `host` | `String` | `GOBLIN_HOST` | `0.0.0.0` | Server bind address |
| `port` | `u16` | `GOBLIN_PORT` | `3000` | Server port |
| `db_path` | `String` | `GOBLIN_DB_PATH` | `goblin_slop.db` | SQLite database file path |
| `content_dir` | `String` | `GOBLIN_CONTENT_DIR` | `content` | Directory containing markdown files |
| `static_dir` | `String` | `GOBLIN_STATIC_DIR` | `static` | Directory containing static assets |
| `data_dir` | `String` | `GOBLIN_DATA_DIR` | `data` | Directory containing scraped data |

### 5.3 `src/main.rs` — Server Entrypoint

**Module declarations:**
```rust
mod config;
mod content;
mod data_loader;
mod db;
mod routes;
```

**`main()` function flow:**
```
1. Initialize tracing logger
2. Open/create SQLite database at "goblin_slop.db"
3. Wrap connection in Arc<Mutex<>> for thread safety
4. Load all .md files from "content/" directory into DB
5. Load all scraped content from "data/scraped_content.json" into DB
6. Create AppState { db }
7. Build Axum router with all routes, nest static file service at /static
8. Bind TCP listener to 0.0.0.0:3000
9. Print startup info
10. Serve requests forever
```

**Error handling**: If DB init fails, the program panics. If content loading fails, it prints a warning but continues.

### 5.4 `src/db.rs` — Database Layer

**Structs:**

| Struct | Fields | Purpose |
|--------|--------|---------|
| `ContentEntry` | `id, slug, title, body_markdown, body_html, category, tags, is_dynamic` | Represents a row in the `content` table. All fields public, implements `Debug`, `Serialize`, `Deserialize`, `Clone`. |
| `DynamicPage` | `path, title, content, keywords` | Represents a cached dynamically-generated page. `keywords` is stored as comma-separated string in DB but loaded as `Vec<String>`. |

**Functions:**

| Function | Signature | Description |
|----------|-----------|-------------|
| `init_db` | `(path: &str) -> SqlResult<Connection>` | Opens/creates SQLite file, executes CREATE TABLE IF NOT EXISTS for all three tables |
| `insert_content` | `(conn: &Connection, entry: &ContentEntry) -> SqlResult<()>` | INSERT OR REPLACE into content table |
| `get_content_by_slug` | `(conn: &Connection, slug: &str) -> SqlResult<Option<ContentEntry>>` | SELECT by slug, returns None if not found |
| `get_all_content` | `(conn: &Connection) -> SqlResult<Vec<ContentEntry>>` | SELECT all content ordered by id |
| `search_content` | `(conn: &Connection, query: &str) -> SqlResult<Vec<ContentEntry>>` | LIKE search across title, body_markdown, and tags |
| `insert_dynamic_page` | `(conn: &Connection, page: &DynamicPage) -> SqlResult<()>` | INSERT OR REPLACE into dynamic_pages table |
| `get_dynamic_page` | `(conn: &Connection, path: &str) -> SqlResult<Option<DynamicPage>>` | SELECT by path, returns None if not found |

### 5.5 `src/content.rs` — Content Loader

```rust
pub fn load_content_from_dir(db_path: &str, content_dir: &str) -> Result<(), Box<dyn std::error::Error>>
```

Scans `content_dir` for all `.md` files. For each file:
1. Read full file contents
2. Extract the filename stem as the `slug`
3. Convert Markdown to HTML via `pulldown_cmark::Parser`
4. Extract title from first `# ` heading line, or title-case the slug as fallback
5. Infer category and tags from slug substrings
6. Insert `ContentEntry` into database

### 5.6 `src/data_loader.rs` — Scraped Content Loader

```rust
pub fn load_scraped_content(db_path: &str, data_dir: &str) -> Result<(), Box<dyn std::error::Error>>
```

Reads `data/scraped_content.json` and loads each entry into the database:

**JSON format:**
```json
{
  "sources": [
    {
      "source": "MyAnimeList - Goblin Slayer",
      "url": "https://myanimelist.net/anime/31964/Goblin_Slayer",
      "category": "anime",
      "tags": "goblin,anime,fantasy,dark,adventure",
      "slug": "goblin-slayer-anime",
      "data": "Full text content..."
    }
  ]
}
```

**Processing per entry:**
1. If `slug` is empty, auto-generate from source name via `slugify()`
2. If `category` is empty, default to `"scraped"`
3. If `tags` is empty, default to `"goblin,scraped"`
4. Wrap data in a markdown document with source attribution: `# {source}\n\n> Source: [{source}]({url})\n\n{data}`
5. Convert to HTML and insert as a `ContentEntry`

**Currently loaded entries: 21 sources** covering MyAnimeList (5), VNDB (1), IMDb (5), TV Tropes (1), D&D (1), Warcraft (1), Warhammer (1), MTG (1), Pathfinder (1), Discworld (1), Goblin Mode linguistics (1), Japanese band GOBLIN (1).

### 5.7 `src/routes/` — Route Handlers Module Directory

The original monolithic `src/routes.rs` was refactored into a directory module with four files:

#### `src/routes/mod.rs`
Module declaration and router construction:

```rust
pub mod generator;
pub mod handlers;
pub mod templates;
```

- Declares `AppState` struct
- `create_router(state: AppState) -> Router` — builds a two-layer router:
  1. **Inner router** (exact routes): `/`, `/search`, `/all`, `/raw/:slug`, `/api/content/:slug`, `/api/dynamic/*path`, `/api/search`, `/api/all`
  2. **Outer router** (fallback): Any path that doesn't match the inner router gets handled by `dynamic_fallback`

#### `src/routes/handlers.rs`
All 9 route handler functions:

| Handler | Route | Logic |
|---------|-------|-------|
| `home_page` | `GET /` | Query all content, build list HTML, render as static page |
| `dynamic_fallback` | `GET /*` | Extract path from URI, check static by slug+underscore, check cached dynamic, or generate new |
| `search_page` | `GET /search?q=` | If q present: search DB, show results. If not: show search form |
| `all_pages` | `GET /all` | List all static content entries |
| `raw_content` | `GET /raw/:slug` | Return raw Markdown body as text/plain |
| `api_content` | `GET /api/content/:slug` | Return single ContentEntry as JSON |
| `api_dynamic` | `GET /api/dynamic/*path` | Return DynamicPage as JSON (generate if needed) |
| `api_search` | `GET /api/search?q=` | Return search results as JSON |
| `api_all` | `GET /api/all` | Return all ContentEntries as JSON |

#### `src/routes/templates.rs`
HTML template rendering functions:

| Function | Signature | Purpose |
|----------|-----------|---------|
| `render_content_page` | `(entry: &ContentEntry, canonical_url: &str) -> String` | Renders a static content page with JSON-LD |
| `render_dynamic_page` | `(dyn_page: &DynamicPage, _canonical_url: &str) -> String` | Renders a dynamically-generated page **without revealing it was generated** — no badges, no summon text, no AI notes |
| `render_static_page` | `(title, body_html, category, tags, canonical_url) -> String` | Renders a simple page from raw HTML body (for home, search, all) |

**Key design principle: The user must never know pages are generated.** All dynamic/metadata markers from `render_dynamic_page()` have been removed (no "✨ Dynamically Generated" badge, no "summoned from the void" text, no AI metadata notes, no console.log scripts).

#### `src/routes/generator.rs`
Dynamic page content generation logic:

**Template arrays:**
| Array | Size | Purpose |
|-------|------|---------|
| `GOBLIN_TITLES` | 10 | Title templates with `{keyword}` placeholder |
| `GOBLIN_INTROS` | 5 | Introductory paragraph templates |
| `GOBLIN_BODIES` | 4 | Body paragraph templates |

**`generate_dynamic_page_content(path, keywords) → DynamicPage`** — selects random templates, replaces `{keyword}` with the primary keyword, generates related sections for additional keywords, and assembles into HTML.

**`parse_path_into_keywords(path) → Vec<String>`** — splits URL path by `/`, `-`, `_`, lowercases, filters stop words and short words.

### 5.8 `static/styles.css` — Styling

| Section | Key Properties |
|---------|---------------|
| **Body** | `background: #1a1a2e`, `color: #e0e0e0`, serif font |
| **Navigation** | Gradient background `#16213e→#0f3460→#1a1a2e`, red bottom border `#e94560`, sticky top |
| **Content card** | Dark background `#16213e`, rounded corners, subtle shadow |
| **Headings** | Red `#e94560` color for h1/h2, lighter `#a0a0c0` for h3 |
| **Strong text** | Gold `#ffd700` |
| **Links** | Blue `#4fc3f7`, red hover `#e94560` |
| **Code blocks** | Dark background `#0f3460` |
| **Search form** | Dark input with red focus border, red submit button |
| **Footer** | Dark background, centered, subtle text |

### 5.9 `static/robots.txt` — SEO

```
User-agent: *
Allow: /
```

Full crawl allowed.

### 5.10 `content/*.md` — Content Library

**`goblin_lore.md`** (32 lines) — Origins in Germanic/Celtic folklore, 4 goblin types, habits, "Goblin mode" cultural reference

**`goblin_tricks.md`** (35 lines) — Classic and digital-age tricks, Sam Altman connection, schizophrenia perception framework

**`sam_altman_goblins.md`** (39 lines) — November 2023 firing saga as goblin trick, goblin mode evidence, schizophrenia connection

**`goblin_schizophrenia.md`** (30 lines) — Shared architecture between goblins and schizophrenia, internet amplifier effect, Sam Altman as projection surface

### 5.11 `data/scraped_content.json` — Scraped Data

21 curated entries about goblins across media:

| Source Category | Entries |
|----------------|---------|
| **Anime** | Goblin Slayer, Goblin Slayer: Goblin's Crown, Goblin Slayer II, Goblin Is Very Strong, Goblins in Anime Overview, Japanese band GOBLIN |
| **Visual Novels** | Goblin-related VNs (Chaos;Head, Eustia, Rance, Evenicle, etc.) |
| **Pop Culture** | Labyrinth (Goblin King Jareth), Harry Potter Goblins, The Hobbit Goblins, Willow Brownies, Gremlins, Green Goblin/Hobgoblin |
| **TV Tropes** | Goblins in Media — 8 common goblin tropes across all media |
| **TTRPG** | D&D Goblin Lore, Pathfinder Goblins |
| **Games** | Warcraft Goblins, Warhammer Goblins, Magic: The Gathering Goblins |
| **Literature** | Discworld Goblins (Terry Pratchett) |
| **Linguistics** | Goblin Mode — Oxford Word of the Year 2022 |

---

## API Reference

### Web (HTML) Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/` | GET | Home page with content listing | HTML 200 |
| `/search?q=...` | GET | Full-text search results | HTML 200 |
| `/all` | GET | List all static + scraped pages | HTML 200 |
| `/raw/:slug` | GET | Raw Markdown source | text/plain 200 |
| `/*` | GET | Any path → goblin page (seamless, no "generated" indicators) | HTML 200 |

### JSON API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/api/all` | GET | All content entries | `{ success: true, data: [...], source: "static" }` |
| `/api/content/:slug` | GET | Single entry by slug | `{ success: true/false, data: {...}, source: "static" }` |
| `/api/dynamic/*path` | GET | Dynamic page by path | `{ success: true, data: {...}, source: "cached_dynamic"|"new_dynamic" }` |
| `/api/search?q=...` | GET | Search results | `{ success: true, data: [...], source: "search" }` |

### Response Format

All JSON responses follow a consistent structure:

```json
{
  "success": true|false,
  "data": <ContentEntry>|<DynamicPage>|[...],
  "source": "static"|"cached_dynamic"|"new_dynamic"|"search"
}
```

### `ContentEntry` JSON Structure

```json
{
  "id": 1,
  "slug": "goblin_lore",
  "title": "Goblin Lore: The Ancient Tricksters",
  "body_markdown": "# Goblin Lore: The Ancient Tricksters\n\n## Origins...",
  "body_html": "<h1>Goblin Lore: The Ancient Tricksters</h1>\n<h2>Origins</h2>\n...",
  "category": "lore",
  "tags": "goblin,lore",
  "is_dynamic": false
}
```

### `DynamicPage` JSON Structure

```json
{
  "path": "ai-takeover-sam-altman",
  "title": "The takeover Trickster",
  "content": "<div class='dynamic-generated'>...<section>...",
  "keywords": ["takeover", "sam", "altman"]
}
```

---

## Dynamic Page Generation Algorithm

### Step 1: URL Path Extraction
```
Input:  /ai-takeover-sam-altman
Output: "ai-takeover-sam-altman"
```

### Step 2: Keyword Parsing
```
Input:  "ai-takeover-sam-altman"
Split:  ["takeover", "sam", "altman"]
Filter: ["takeover", "sam", "altman"]
```

### Step 3: Content Assembly
```
primary_keyword = "takeover"  (first keyword)

title = random from GOBLIN_TITLES with {keyword} → "takeover"
      = "The takeover Trickster"

intro = random from GOBLIN_INTROS with {keyword} → "takeover"
body  = random from GOBLIN_BODIES with {keyword} → "takeover"

related_sections for ["sam", "altman"]:
  → "<h2>Goblins and sam</h2><p>..."
  → "<h2>Goblins and altman</h2><p>..."

Final HTML structure:
<div>
  <section><p>intro</p><p>body</p></section>
  related_sections...
  <section><h2>The Goblin Verdict</h2><p>...</p></section>
</div>
```

### Step 4: Caching
Generated `DynamicPage` is inserted into `dynamic_pages` table via `INSERT OR REPLACE`. Subsequent requests to the same path will serve the cached version instantly.

---

## Testing & Verification

### Automated Test Script

```bash
# Fresh start
rm -f goblin_slop.db && cargo build

# Start server (background)
cargo run &
sleep 3

# Test 1: Home page returns HTML
curl -s http://localhost:3000/ | head -3
# Expected: <!DOCTYPE html><html>...

# Test 2: Static content
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/goblin-lore
# Expected: 200

# Test 3: Scraped content
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/goblin-slayer-anime
# Expected: 200

# Test 4: Dynamic page (looks identical to any other page)
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/random-goblin
# Expected: 200 (no "generated" badges visible)

# Test 5: Raw markdown endpoint
curl -s http://localhost:3000/raw/goblin_lore | head -3
# Expected: # Goblin Lore: The Ancient Tricksters

# Test 6: JSON API
curl -s http://localhost:3000/api/all | python3 -c "
import sys, json
d = json.load(sys.stdin)
print(f'{len(d[\"data\"])} entries from {d[\"source\"]}')
"
# Expected: 25 entries from static

# Test 7: Search
curl -s "http://localhost:3000/search?q=altman" | grep -o "Search results"
# Expected: Search results

# Kill server
kill %1
```

### Verified Results
| Test | Result |
|------|--------|
| Home page (/) | ✅ HTML with content listing |
| Static content (/goblin-lore) | ✅ HTTP 200 |
| Scraped content (/goblin-slayer-anime) | ✅ HTTP 200 |
| Dynamic page (/*) | ✅ HTTP 200, no "generated" indicators |
| Raw markdown (/raw/:slug) | ✅ Raw Markdown returned |
| API /api/all | ✅ 25 entries (4 markdown + 21 scraped) |
| Search /search?q=altman | ✅ Search results found |
| No "dynamic" leaks in HTML | ✅ No badges, summon text, or AI notes |

---

## Deployment

### Production Server

| Detail | Value |
|--------|-------|
| **Domain** | `goblin.geno.su` |
| **URL** | `https://goblin.geno.su` (port 443, HTTPS) |
| **HTTP Redirect** | `http://goblin.geno.su` → `https://goblin.geno.su` (301) |
| **Backend** | `http://127.0.0.1:3000` (internal only) |
| **SSH Login** | `root@IP` |
| **OS** | Ubuntu 24.04.4 LTS |
| **Install Path** | `/opt/goblinSlop` |
| **Service Name** | `goblinSlop` (systemd) |
| **Reverse Proxy** | nginx (port 443 → 127.0.0.1:3000) |
| **TLS/SSL** | Let's Encrypt (auto-renewing) |

### Architecture

```
Browser ──▶ HTTPS (443) ──▶ Nginx ──▶ GoblinSlop (127.0.0.1:3000)
                │                  │
                │   HTTP (80)      │
                └──▶ 301 redirect ──▶ HTTPS
```

---

## Design Decisions & Trade-offs

### 1. Routes as Module Directory

**Decision**: Refactored `routes.rs` (538 lines) into `routes/mod.rs` + `routes/handlers.rs` + `routes/templates.rs` + `routes/generator.rs`.

**Rationale**: Single-file module became unwieldy. Separating concerns (routing, handlers, HTML rendering, content generation) makes the codebase more maintainable.

### 2. Scraped Data as Static JSON vs. Runtime Scraper

**Decision**: Curate `scraped_content.json` manually instead of building a web scraper.

**Rationale**: Web scraping at runtime is fragile (503 errors, rate limits, HTML structure changes). Manual curation produces clean, structured content without ads/nav/script garbage.

### 3. Hidden Dynamic Generation

**Decision**: Remove all badges, summon text, AI metadata notes, and console.log scripts from dynamically generated pages.

**Rationale**: The user should never know that content is procedurally generated. Every page should look equally authentic — this is the goblin way.

### 4. Pure Rust String Templating vs. Template Engine

**Decision**: Use `String::replace()` with placeholder-based HTML constants.

**Rationale**: No extra dependency. For a small number of page layouts (3 templates), string replacement is simple and fast.

### 5. SQLite with Mutex vs. Connection Pool

**Decision**: Use `Arc<Mutex<Connection>>`.

**Rationale**: Simple to implement. Sufficient for expected low traffic.

### 6. Hyphen→Underscore Normalization

**Decision**: In the fallback handler, also try the slug with hyphens replaced by underscores.

**Rationale**: Markdown filenames use underscores (`goblin_lore.md`). URL convention prefers hyphens (`/goblin-lore`). This normalization makes URLs pretty while keeping file naming natural.

---

## How to Extend

### Adding New Static Content
1. Create a new Markdown file in `content/` (e.g., `content/goblin_origins.md`)
2. Start with a `# Title` heading
3. The server will auto-load it on next restart
4. Access at `/goblin-origins` (hyphens replace underscores)

### Adding New Scraped Content
1. Add a new entry to `data/scraped_content.json` with fields: `source`, `url`, `data`, `category` (optional), `tags` (optional), `slug` (optional)
2. The server will auto-load it on next restart

### Adding New Dynamic Content Templates
1. Open `src/routes/generator.rs`
2. Add entries to `GOBLIN_TITLES`, `GOBLIN_INTROS`, or `GOBLIN_BODIES` arrays
3. Templates use `{keyword}` as placeholder
4. Rebuild and restart

### Adding New Routes
1. Add a handler function in `src/routes/handlers.rs`
2. Register the route in `src/routes/mod.rs::create_router()` before the fallback

### Adding Database Capabilities
1. Update schema in `db.rs::init_db()`
2. Add CRUD functions in `db.rs`
3. Call them from route handlers

---

## How to Run

### Prerequisites
- Rust toolchain (1.92+)
- No system-level dependencies (SQLite is bundled)

### Commands

```bash
# Clone or navigate to project
cd /home/azu/projects/goblinSlop

# Build (optional, cargo run builds automatically)
cargo build

# Run the server
cargo run

# Server starts at:
# http://0.0.0.0:3000
```

### Environment
- Binds to all interfaces (`0.0.0.0`)
- Port: 3000 (default, configurable via `GOBLIN_PORT`)
- Database: `goblin_slop.db` created in project root
- Content: loaded from `content/` directory (Markdown files) and `data/scraped_content.json` (JSON entries)

### Clean Restart
```bash
# Delete database and restart (fresh content load)
rm -f goblin_slop.db
cargo run