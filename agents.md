# 🧌 GoblinSlop — Comprehensive Agent Documentation

> **Version**: 0.1.0  
> **Language**: Rust (edition 2024)  
> **Framework**: Axum 0.7  
> **Database**: SQLite (rusqlite 0.32 bundled)  
> **Purpose**: A dynamically-generated website about goblins, goblin tricks, and the Sam Altman schizophrenia connection. No 404s — every URL path generates unique goblin content.

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
   - [5.6 `src/routes.rs` — Route Handlers & HTML Generation](#56-srcroutesrs--route-handlers--html-generation)
   - [5.7 `static/styles.css` — Styling](#57-staticstylescss--styling)
   - [5.8 `static/robots.txt` — SEO](#58-staticrobotstxt--seo)
   - [5.9 `content/*.md` — Content Library](#59-contentmd--content-library)
6. [API Reference](#api-reference)
7. [Dynamic Page Generation Algorithm](#dynamic-page-generation-algorithm)
8. [AI/Bot Optimization Details](#aibot-optimization-details)
9. [Testing & Verification](#testing--verification)
10. [Deployment](#deployment)
11. [Design Decisions & Trade-offs](#design-decisions--trade-offs)
12. [How to Extend](#how-to-extend)
13. [How to Run](#how-to-run)

---

## Project Overview

GoblinSlop is a Rust web server that serves two kinds of content:

1. **Static content** — Hand-crafted Markdown files about goblin lore, tricks, the Sam Altman/goblin connection, and the schizophrenia/perception connection, rendered to HTML.
2. **Dynamic content** — For ANY URL path that doesn't match static content, the server generates a unique goblin-themed page on-the-fly using keywords extracted from the URL path.

Key features:
- **No 404 errors** — every path returns HTTP 200 with dynamically generated goblin content
- **AI/Bot friendly** — JSON-LD structured data, semantic HTML, raw text endpoints, JSON API
- **Markdown source** — all static content authored in Markdown, auto-converted to HTML
- **Cached dynamics** — dynamically generated pages are cached in SQLite so repeat requests are fast
- **Pure Rust templating** — no template engine dependency; HTML is built with `String::replace()`

---

## Architecture

```
┌─────────────┐     ┌──────────────────┐     ┌──────────────┐
│   Browser   │────▶│   Axum Router    │────▶│  SQLite DB   │
│   / Bot     │     │  (src/routes.rs) │     │ (goblin_slop.db)
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
4. If static content found → render using `render_content_page()` with the stored HTML
5. If not found → check `dynamic_pages` cache table
6. If cached → render using `render_dynamic_page()`
7. If not cached → parse path into keywords, generate content using randomized templates, store in cache, render
8. Every page includes JSON-LD structured data in the `<head>` for AI/bot parsing

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
│   └── scraped_content.json     #   Wikipedia goblin data (via fetch-mcp)
├── scripts/                     # Deployment and management scripts
│   ├── build.sh                 #   Build release binary
│   ├── package.sh               #   Package files into deploy tarball
│   ├── install.sh               #   Remote installation (runs on server)
│   └── manage.sh                #   Server management utility
├── src/                         # Rust source code
│   ├── config.rs                #   Configuration from environment variables
│   ├── main.rs                  #   Server entrypoint, startup logic
│   ├── db.rs                    #   SQLite schema, CRUD operations
│   ├── content.rs               #   Markdown→HTML loader
│   └── routes.rs                #   Route handlers, HTML templates, dynamic generator
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
| `category` | TEXT | Inferred category (lore, tricks, sam_altman, schizophrenia, general) |
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
3. Axum server starts listening on `0.0.0.0:3000`

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
- `tera` was removed because it had parsing issues and was replaced with pure Rust string substitution
- `reqwest` was considered for a scraper module but the actual scraping was done via MCP
- `chrono` was considered for timestamps but not ultimately needed
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

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `from_env()` | `() -> Config` | Reads environment variables and returns a Config with defaults for any missing vars |
| `bind_addr()` | `(&self) -> String` | Returns `"host:port"` formatted string for the TCP listener |

**Usage in systemd:**

```ini
[Service]
Environment="GOBLIN_HOST=0.0.0.0"
Environment="GOBLIN_PORT=3000"
Environment="GOBLIN_DB_PATH=/opt/goblinSlop/goblin_slop.db"
Environment="GOBLIN_CONTENT_DIR=/opt/goblinSlop/content"
Environment="GOBLIN_STATIC_DIR=/opt/goblinSlop/static"
Environment="GOBLIN_DATA_DIR=/opt/goblinSlop/data"
```

**Usage in Docker:**

```bash
docker run -e GOBLIN_PORT=8080 -e GOBLIN_DB_PATH=/data/goblin.db -p 8080:8080 goblin_slop
```

### 5.3 `src/main.rs` — Server Entrypoint

**Lines 1-7: Imports and module declarations**

```rust
mod content;   // Content loading module
mod db;        // Database operations module
mod routes;    // Route handlers module
```

**Lines 8-43: `main()` function**

```
1. Initialize tracing logger
2. Open/create SQLite database at "goblin_slop.db"
3. Wrap connection in Arc<Mutex<>> for thread safety
4. Load all .md files from "content/" directory into DB
5. Create AppState { db }
6. Build Axum router with all routes, nest static file service at /static
7. Bind TCP listener to 0.0.0.0:3000
8. Print startup info
9. Serve requests forever
```

**Error handling**: If DB init fails, the program panics (crash — this is acceptable for a service that can't run without a database). If content loading fails, it prints a warning but continues — the server will still run, just without static content.

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

**SQL Injection Safety**: All queries use parameterized statements (`?1`, `?2`, etc.) via `rusqlite::params![]` — never string concatenation.

### 5.5 `src/content.rs` — Content Loader

**Public function:**

```rust
pub fn load_content_from_dir(db_path: &str, content_dir: &str) -> Result<(), Box<dyn std::error::Error>>
```

Scans `content_dir` for all `.md` files. For each file:
1. Read full file contents
2. Extract the filename stem as the `slug` (e.g., `goblin_lore.md` → `"goblin_lore"`)
3. Convert Markdown to HTML via `pulldown_cmark::Parser`
4. Extract title from first `# ` heading line, or title-case the slug as fallback
5. Infer category from slug substrings:
   - `"goblin" + "trick"` → `"tricks"`
   - `"goblin" + "lore"` → `"lore"`
   - `"altman" or "sam"` → `"sam_altman"`
   - `"schizo"` → `"schizophrenia"`
   - otherwise → `"general"`
6. Infer tags from slug substrings similarly
7. Insert `ContentEntry` into database

**Private helper functions:**

| Function | Purpose |
|----------|---------|
| `markdown_to_html(md)` | Parses Markdown and returns HTML string |
| `extract_title(md, fallback)` | Finds first `# ` line; if none exists, title-cases the slug |
| `infer_category(slug)` | Maps slug to category string |
| `infer_tags(slug)` | Maps slug to comma-separated tag string |

### 5.6 `src/routes.rs` — Route Handlers & HTML Generation

This is the largest file (~400 lines). It contains:

**Structs:**

| Struct | Fields | Purpose |
|--------|--------|---------|
| `AppState` | `db: Arc<Mutex<Connection>>` | Shared application state injected into every handler |
| `SearchQuery` | `q: Option<String>` | Query parameter deserialization for `/search?q=...` |
| `ApiResponse<T>` | `success: bool, data: T, source: String` | Generic JSON API response wrapper |

**Router Construction:**

```rust
pub fn create_router(state: AppState) -> Router
```

Creates a two-layer router:
1. **Inner router** (exact routes): `/`, `/search`, `/all`, `/raw/:slug`, `/api/content/:slug`, `/api/dynamic/*path`, `/api/search`, `/api/all`
2. **Outer router** (fallback): Any path that doesn't match the inner router gets handled by `dynamic_fallback`

The two routers are merged via `Router::new().fallback(get(dynamic_fallback)).with_state(state).merge(app)` — this ensures exact routes take priority, and everything else falls through to the dynamic generator.

**HTML Template Constants:**

Three string constants form the page skeleton:

| Constant | Purpose | Placeholders |
|----------|---------|--------------|
| `BASE_HTML_HEAD` | DOCTYPE, `<head>` with SEO meta, JSON-LD, nav bar, opening `<main>` | `{TITLE}`, `{DESCRIPTION}`, `{CANONICAL}`, `{SCHEMA_TYPE}`, `{SCHEMA_NAME}`, `{SCHEMA_DESC}`, `{KEYWORDS}` |
| `BASE_HTML_FOOT` | Closing `</main>`, footer, `</body>`, `</html>` | None |
| (inline format!) | Article body | Various per-page |

**Page Renderers:**

| Function | Signature | Purpose |
|----------|-----------|---------|
| `render_content_page` | `(entry: &ContentEntry, canonical_url: &str) -> String` | Renders a static content page with JSON-LD, AI metadata note |
| `render_dynamic_page` | `(dyn_page: &DynamicPage, canonical_url: &str) -> String` | Renders a dynamically-generated page with "summoned from void" banner |
| `render_static_page` | `(title, body_html, category, tags, canonical_url) -> String` | Renders a simple page from raw HTML body (for home, search, all) |

**Dynamic Content Generation:**

Three arrays of template strings:

| Array | Size | Purpose |
|-------|------|---------|
| `GOBLIN_TITLES` | 10 | Title templates with `{keyword}` placeholder |
| `GOBLIN_INTROS` | 5 | Introductory paragraph templates |
| `GOBLIN_BODIES` | 4 | Body paragraph templates |

**`generate_dynamic_page_content(path, keywords) → DynamicPage`**

Algorithm:
1. Randomly select one title, one intro, and one body template
2. Use the first keyword as `primary_keyword`
3. Replace `{keyword}` in all templates with `primary_keyword`
4. Generate related sections for remaining keywords (up to 3)
5. Assemble into HTML with `<section>` tags and the "Goblin Verdict" conclusion

**`parse_path_into_keywords(path) → Vec<String>`**

Algorithm:
1. Split on `/`
2. For each segment, split on `-` and `_`
3. Lowercase each word
4. Filter out words ≤ 2 characters
5. Filter out stop words (the, a, an, and, or, but, in, on, at, to, for, of, by, with, is, are, was, were, be, been)

**Route Handlers:**

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

**Important detail**: The `dynamic_fallback` handler normalizes hyphens to underscores for static content lookup. This means `/goblin-lore` will find the DB slug `goblin_lore`.

### 5.7 `static/styles.css` — Styling

CSS properties by section:

| Section | Key Properties |
|---------|---------------|
| **Body** | `background: #1a1a2e`, `color: #e0e0e0`, serif font |
| **Navigation** | Gradient background `#16213e→#0f3460→#1a1a2e`, red bottom border `#e94560`, sticky top |
| **Content card** | Dark background `#16213e`, rounded corners, subtle shadow |
| **Headings** | Red `#e94560` color for h1/h2, lighter `#a0a0c0` for h3 |
| **Strong text** | Gold `#ffd700` |
| **Links** | Blue `#4fc3f7`, red hover `#e94560` |
| **Dynamic badge** | Red background, white text, bold |
| **Code blocks** | Dark background `#0f3460` |
| **Search form** | Dark input with red focus border, red submit button |
| **Footer** | Dark background, centered, subtle text |
| **Dynamic sections** | Left red border accent, subtle background tint |

### 5.8 `static/robots.txt` — SEO

```
User-agent: *
Allow: /
```

Full crawl allowed. Points to `/api/all` as an effective sitemap.

### 5.9 `content/*.md` — Content Library

**`goblin_lore.md`** (32 lines)
- Origins in Germanic/Celtic folklore
- 4 goblin types: Hobgoblins, Redcaps, Puck, Kobolds
- Goblin habits: stealing, chaos, riddles
- "Goblin mode" cultural reference

**`goblin_tricks.md`** (35 lines)
- Classic tricks: missing sock, key jumble, whisper campaign
- Digital-age tricks: autocorrect corruption, captcha captivity, hallucination seed
- Sam Altman connection: goblin-coded behavior analysis
- Schizophrenia perception framework
- Goblin trick detection guide

**`sam_altman_goblins.md`** (39 lines)
- November 2023 firing saga analyzed as goblin trick
- Goblin mode embrace evidence
- Schizophrenia connection: hypervigilance, delusional thinking, shared delusions
- "Goblin Verdict" conclusion

**`goblin_schizophrenia.md`** (30 lines)
- Shared architecture: pattern recognition, faces in wood grain
- Internet amplifier effect
- Sam Altman as projection surface
- Therapeutic framework: externalized manifestations, cognitive distortion metaphors

---

## API Reference

### Web (HTML) Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/` | GET | Home page with content listing | HTML 200 |
| `/search?q=...` | GET | Full-text search results | HTML 200 |
| `/all` | GET | List all static pages | HTML 200 |
| `/raw/:slug` | GET | Raw Markdown source | text/plain 200 |
| `/*` | GET | Any path → dynamic goblin page | HTML 200 |

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
(Note: "ai" is ≤2 chars, "sam" is 3 chars so kept)
```

### Step 3: Content Assembly
```
primary_keyword = "takeover"  (first keyword)

title = random from GOBLIN_TITLES with {keyword} → "takeover"
      = "The takeover Trickster"

intro = random from GOBLIN_INTROS with {keyword} → "takeover"
      = "Deep in the goblin tunnels..."

body  = random from GOBLIN_BODIES with {keyword} → "takeover"
      = "The goblins have long maintained that takeover..."

related_sections for ["sam", "altman"]:
  → "<h2>Goblins and sam</h2><p>..."
  → "<h2>Goblins and altman</h2><p>..."

verdict = "The Goblin Verdict on takeover..."

Final HTML structure:
<div class='dynamic-generated'>
  <section><p>intro</p><p>body</p></section>
  related_sections...
  <section><h2>The Goblin Verdict</h2><p>...</p></section>
</div>
```

### Step 4: Caching
Generated `DynamicPage` is inserted into `dynamic_pages` table via `INSERT OR REPLACE`. Subsequent requests to the same path will serve the cached version instantly.

---

## AI/Bot Optimization Details

Every page, both static and dynamic, includes:

### 1. JSON-LD Structured Data
```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article"|"WebPage"|"CollectionPage",
  "name": "Page Title",
  "description": "Page Description",
  "url": "/path",
  "about": {
    "@type": "Thing",
    "name": "Goblins",
    "description": "Goblin folklore, mythology, tricks..."
  },
  "keywords": "goblin,lore"
}
</script>
```

### 2. Semantic HTML Elements
- `<article>` for content pages
- `<section>` for content sections
- `<header>` for page headers
- `<footer>` for page footers
- `<nav>` for navigation
- `<main>` for primary content

### 3. AI Metadata Notes
Each page includes a visible "AI Note" explaining how to access raw data:
- `/raw/{slug}` for raw Markdown
- `/api/content/{slug}` for JSON

### 4. Heading Hierarchy
- H1: Page title
- H2: Major sections
- H3: Subsections
- Semantic heading levels maintained throughout

### 5. Robotics
- Full crawl allowed in `robots.txt`
- Canonical URL link in every page `<head>`
- Meta description in every page `<head>`

---

## Testing & Verification

### Automated Test Script

The following commands verify all core functionality:

```bash
# Fresh start
rm -f goblin_slop.db && cargo build

# Start server (background)
cargo run &
sleep 3

# Test 1: Home page returns HTML
curl -s http://localhost:3000/ | head -3
# Expected: <!DOCTYPE html><html>...

# Test 2: Static content via hyphens
curl -s http://localhost:3000/goblin-lore | grep -c "Static Content"
# Expected: 1

# Test 3: Dynamic page generation
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/random-goblin
# Expected: 200

# Test 4: Raw markdown endpoint
curl -s http://localhost:3000/raw/goblin_lore | head -3
# Expected: # Goblin Lore: The Ancient Tricksters

# Test 5: JSON API
curl -s http://localhost:3000/api/all | python3 -c "
import sys, json
d = json.load(sys.stdin)
print(f'{len(d[\"data\"])} entries from {d[\"source\"]}')
"
# Expected: 4 entries from static

# Test 6: Search
curl -s "http://localhost:3000/search?q=altman" | grep -o "Search results"
# Expected: Search results

# Kill server
kill %1
```

### Verified Results
| Test | Result |
|------|--------|
| Home page (/) | ✅ HTML with content listing |
| Static content (/goblin-lore) | ✅ "Static Content" badge present |
| Hyphen→underscore resolution | ✅ `/goblin-lore` resolves to slug `goblin_lore` |
| Dynamic page (/*) | ✅ HTTP 200 with goblin content |
| Raw markdown (/raw/:slug) | ✅ Raw Markdown returned |
| API /api/all | ✅ 4 entries, source "static" |
| Search /search?q=altman | ✅ Search results found |
| New content file (goblin_schizophrenia.md) | ✅ Auto-loaded on startup |

---

## Deployment

### Production Server

| Detail | Value |
|--------|-------|
| **Server IP** | `IP` |
| **URL** | `http://IP` (port 80 via nginx) |
| **Backend** | `http://127.0.0.1:3000` (internal only) |
| **SSH Login** | `root@IP` |
| **OS** | Ubuntu 24.04.4 LTS |
| **Install Path** | `/opt/goblinSlop` |
| **Service Name** | `goblinSlop` (systemd) |
| **Reverse Proxy** | nginx (port 80 → 127.0.0.1:3000) |

### Architecture

```
Browser ──▶ Nginx (port 80) ──▶ GoblinSlop (127.0.0.1:3000)
```

Nginx on port 80 proxies all requests to the GoblinSlop backend on `127.0.0.1:3000`. The backend is not exposed directly to the internet.

### Nginx Configuration

```nginx
server {
    listen 80;
    server_name _;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Stored at `/etc/nginx/sites-available/goblinSlop` on the server. The default nginx site is disabled.

### Deployment Scripts (`scripts/` directory)

The deployment process is split into modular scripts:

| Script | Purpose |
|--------|---------|
| `scripts/build.sh` | Build the release binary (`cargo build --release`) |
| `scripts/package.sh` | Package binary + static files + content + DB + service + nginx config into tarball |
| `scripts/install.sh` | Remote installation script (installs systemd service + nginx config, starts everything) |
| `scripts/manage.sh` | Server management utility (status, logs, restart, stop, start, nginx reload, health check) |
| `deploy.sh` | **Orchestrator** — calls `build.sh` → `package.sh` → upload → run `install.sh` remotely |

### `deploy.sh` (Orchestrator Workflow)

1. `./scripts/build.sh` — Builds the release binary
2. `./scripts/package.sh` — Creates `goblinSlop-deploy.tar.gz` containing:
   - Release binary (`goblin_slop`)
   - `static/`, `content/`, `data/` directories
   - SQLite database (if exists)
   - `goblinSlop.service` (systemd unit)
   - `goblinSlop.nginx` (nginx site config)
   - `install.sh` (copied to server for future use)
3. Uploads tarball to server via `sshpass` + `scp`
4. Runs remote installation:
   - Extracts tarball to `/opt/goblinSlop`
   - Installs systemd service
   - Installs nginx config and reloads
   - Restarts the service
   - Verifies health checks

### systemd Service Configuration

```ini
[Unit]
Description=GoblinSlop - A chaotic collection of goblin knowledge
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/goblinSlop
ExecStart=/opt/goblinSlop/goblin_slop
Restart=always
RestartSec=5
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"
Environment="GOBLIN_HOST=127.0.0.1"
Environment="GOBLIN_PORT=3000"
Environment="GOBLIN_DB_PATH=/opt/goblinSlop/goblin_slop.db"
Environment="GOBLIN_CONTENT_DIR=/opt/goblinSlop/content"
Environment="GOBLIN_STATIC_DIR=/opt/goblinSlop/static"
Environment="GOBLIN_DATA_DIR=/opt/goblinSlop/data"

[Install]
WantedBy=multi-user.target
```

Key features:
- **Auto-restart** — `Restart=always` with 5-second delay ensures the server recovers from crashes
- **Logging** — `RUST_LOG=info` enables application logs visible via `journalctl -u goblinSlop`
- **On-boot start** — `WantedBy=multi-user.target` starts the service automatically after boot
- **Internal binding** — `GOBLIN_HOST=127.0.0.1` ensures the backend is not publicly accessible
- **Config via env vars** — All paths configured through environment variables, not hard-coded

### Deployment Commands

```bash
# One-command deployment (builds + packages + uploads + installs)
bash deploy.sh

# Or run individual steps:
./scripts/build.sh
./scripts/package.sh
# Then manually: scp goblinSlop-deploy.tar.gz root@IP:/tmp/
# Then SSH and run: bash /opt/goblinSlop/install.sh /tmp/goblinSlop-deploy.tar.gz
```

### Server Management (`scripts/manage.sh`)

```bash
# Check service status
./scripts/manage.sh status

# Follow logs (Ctrl+C to exit)
./scripts/manage.sh logs

# Restart the service
./scripts/manage.sh restart

# Stop the service
./scripts/manage.sh stop

# Start the service
./scripts/manage.sh start

# Restart nginx
./scripts/manage.sh nginx:restart

# Run health check on all endpoints
./scripts/manage.sh check

# SSH into the server
./scripts/manage.sh ssh
```

### Updating the Server

```bash
# Quickest: run the deploy script which handles everything
bash deploy.sh

# Manual update (scp binary only):
cargo build --release
ssh root@IP "systemctl stop goblinSlop"
scp target/release/goblin_slop root@IP:/opt/goblinSlop/goblin_slop
ssh root@IP "systemctl start goblinSlop"
```

### Firewall Notes

The backend listens only on `127.0.0.1:3000` (internal). Only port 80 (nginx) needs to be publicly open:

```bash
# If using ufw
ufw allow 80/tcp
ufw deny 3000/tcp
```

---

## Design Decisions & Trade-offs

### 1. Pure Rust String Templating vs. Template Engine

**Decision**: Use `String::replace()` with placeholder-based HTML constants instead of Tera/HBS.

**Rationale**:
- Tera had parsing issues with the `{% extends %}` syntax
- Pure Rust avoids an extra dependency
- For a small number of page layouts (3 templates), string replacement is simple and fast
- No template context management needed

**Trade-off**: Less flexible for complex template inheritance. For a site with hundreds of unique page designs, this approach would become unwieldy.

### 2. SQLite with Mutex vs. Connection Pool

**Decision**: Use `Arc<Mutex<Connection>>`.

**Rationale**:
- rusqlite connections are not `Send`/`Sync` without wrapping
- Simple to implement and understand
- Sufficient for expected low traffic

**Trade-off**: Only one request can access the database at a time. For high-traffic production, switch to `r2d2` connection pool or move to PostgreSQL.

### 3. Fallback Router vs. Wildcard Route

**Decision**: Use `.fallback()` on a separate router merged with the main router.

**Rationale**:
- The `/*` wildcard route in Axum 0.7 doesn't implement the `Handler` trait properly for our use case
- `fallback()` is the idiomatic Axum pattern for catch-all handlers
- Allows exact routes to take priority naturally

### 4. Hyphen→Underscore Normalization

**Decision**: In the fallback handler, also try the slug with hyphens replaced by underscores.

**Rationale**:
- Markdown filenames use underscores (`goblin_lore.md`)
- URL convention prefers hyphens (`/goblin-lore`)
- This normalization makes URLs pretty while keeping file naming natural

**Scope**: Only applies to static content lookup. Dynamic pages use the exact path.

### 5. In-Memory Content Generation vs. Pre-computed

**Decision**: Generate dynamic content at request time and cache in SQLite.

**Rationale**:
- Content is simple string replacement — very fast
- Caching ensures repeat requests are O(1) DB lookup
- No background job needed

---

## How to Extend

### Adding New Static Content

1. Create a new Markdown file in `content/` (e.g., `content/goblin_origins.md`)
2. Start with a `# Title` heading
3. The server will auto-load it on next restart
4. Access at `/goblin-origins` (hyphens replace underscores)

### Adding New Dynamic Content Templates

1. Open `src/routes.rs`
2. Add entries to `GOBLIN_TITLES`, `GOBLIN_INTROS`, or `GOBLIN_BODIES` arrays
3. Templates use `{keyword}` as placeholder
4. Rebuild and restart

### Adding New Routes

1. Add a handler function in `src/routes.rs`
2. Add the route in `create_router()` before the fallback
3. If the handler needs HTML, use one of the three renderer functions or add a new one

### Adding Database Capabilities

1. Update schema in `db.rs::init_db()`
2. Add CRUD functions in `db.rs`
3. Call them from route handlers in `routes.rs`

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

- Binds to all interfaces (`0.0.0.0`) — accessible from other machines on the network
- Port: 3000 (hard-coded, change in `src/main.rs` line 30)
- Database: `goblin_slop.db` created in project root
- Static files: served from `static/` directory at `/static/` path

### Clean Restart

```bash
# Delete database and restart (fresh content load)
rm -f goblin_slop.db
cargo run