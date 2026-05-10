# 🧌 GoblinSlop — Comprehensive Agent Documentation

> **Version**: 0.1.0 (unified content)  
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
   - [5.5 `src/json_content_loader.rs` — Unified JSON Content Loader](#55-srcjson_content_loaderrs--unified-json-content-loader)
   - [5.6 `src/routes/` — Route Handlers Module Directory](#56-srcroutes--route-handlers-module-directory)
   - [5.7 `static/styles.css` — Styling](#57-staticstylescss--styling)
   - [5.8 `static/robots.txt` — SEO](#58-staticrobotstxt--seo)
   - [5.9 `data/content/*.json` — Unified Content Library](#59-datacontentjson--unified-content-library)
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

1. **Static content** — Unified JSON files about goblin lore, tricks, the Sam Altman/goblin connection, anime, games, and pop culture. Each file contains Markdown body + metadata (title, slug, category, tags, date_added), auto-converted to HTML.
2. **Dynamic content** — For ANY URL path that doesn't match existing content, the server generates a unique goblin-themed page on-the-fly using keywords extracted from the URL path. The user never sees any indication that pages are generated — everything looks equally authentic.

Key features:
- **No 404 errors** — every path returns HTTP 200 with goblin content
- **AI/Bot friendly** — JSON-LD structured data, semantic HTML, raw text endpoints, JSON API
- **Unified JSON format** — all content in individual `.json` files under `data/content/`, each with `{id, title, slug, body_markdown, category, tags, is_dynamic, date_added}`
- **Deterministic dynamic generation** — same URL path always produces identical output via seeded RNG; no DB caching needed
- **SEO optimized** — canonical URLs (absolute), sitemap.xml, 301 redirects for duplicate slug variants, unique page titles, meta robots tags
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
                    │  HTML Generator  │     │  JSON Loader │
                    │ (string replace) │     │ (serde_json) │
                    └──────────────────┘     │ (pulldown-   │
                                             │  cmark)      │
                                             └──────────────┘
```

### Request Flow

1. Client sends HTTP GET to a path (e.g., `/goblin-lore` or `/some-random-path`)
2. Axum router matches the path against defined routes or the fallback handler
3. If the path contains underscores (e.g. `/goblin_lore`), the fallback handler issues a **301 MOVED_PERMANENTLY** redirect to the canonical hyphen form (`/goblin-lore`)
4. Fallback handler checks SQLite for static content matching the path
5. If static content found → render using `templates::render_content_page()` with the stored HTML, absolute canonical URL
6. If not found → parse path into keywords, generate content deterministically using `generator::generate_dynamic_page_content()` (seeded RNG from URL path), render directly
7. **No DB caching of dynamic pages** — generation is deterministic so the same path always produces identical output without cache lookups or writes
8. All pages include JSON-LD structured data, absolute canonical URLs, and meta robots tags in the `<head>`
9. **No "dynamically generated" badges, summon messages, or AI notes are shown to the user** — all pages appear equally authentic

### Thread Safety Model

The SQLite `Connection` is non-thread-safe, so it is wrapped in `Arc<Mutex<Connection>>` and cloned into every handler via Axum's `State` extractor. This means:
- Only one request at a time can access the database
- For a site like this with expected low traffic, this is acceptable
- For high traffic, a connection pool (like `r2d2`) would be needed

---

## Directory Structure

```
goblinSlop/
├── agents.md                    # This file — complete documentation
├── Cargo.toml                   # Rust project manifest with dependencies
├── Cargo.lock                   # Dependency lock file (auto-generated)
├── .env                         # Deployment config (gitignored — DEPLOY_USER, DEPLOY_HOST, APP_USER)
├── example.env                  # Environment variable template for deployment
├── data/                        # Unified content library (individual JSON files)
│   └── content/                 #   37 individual .json files (goblin lore, anime, pop culture, games...)
├── deploy/                      # Deployment scripts & systemd service template
│   ├── build-and-deploy.sh      #   Build → package → SCP → SSH deploy one-shot script
│   └── goblinSlop.service       #   systemd unit template (__APP_USER__ placeholder)
├── src/                         # Rust source code
│   ├── config.rs                #   Configuration from environment variables
│   ├── main.rs                  #   Server entrypoint, startup logic
│   ├── db.rs                    #   SQLite schema, CRUD operations
│   ├── json_content_loader.rs   #   Unified JSON content loader (single source)
│   └── routes/                  #   Route handlers module directory
│       ├── mod.rs               #     Module declaration, AppState, create_router()
│       ├── handlers.rs          #     Shared `ApiResponse` type
│       ├── pages/               #     One file per route handler (9 files)
│       ├── templates.rs         #     HTML template rendering functions
│       ├── content_templates.rs #     Text templates (titles, intros, bodies, verdicts)
│       ├── references.rs        #     Real & randomly-generated fake page references
│       └── generator.rs         #     Coordinator: assembles dynamic page from above
├── static/                      # Static files served at /static/
│   ├── styles.css               #   Goblin-themed dark CSS
│   └── robots.txt               #   SEO/crawler instructions
├── scripts/                     # Legacy scripts (nginx config, log viewer)
│   ├── goblinSlop.nginx.conf    #   Main nginx config with custom log format
│   └── logs.sh                  #   Nginx log viewer (tail, top IPs, slow requests, etc.)
└── target/                      # Compiled output (gitignored)
```

---

## Data Flow

### Database Schema

**Table: `content`** — Core article data
| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER PK AUTO | Auto-incrementing ID |
| `slug` | TEXT UNIQUE | URL-friendly identifier (e.g., `goblin-lore`) |
| `title` | TEXT | Page title |
| `body_markdown` | TEXT | Raw Markdown source |
| `body_html` | TEXT | Pre-rendered HTML from Markdown |
| `category` | TEXT | Category (lore, tricks, anime, pop_culture, etc.) |
| `is_dynamic` | INTEGER | Boolean: 0 = static; 1 = dynamic |
| `date_added` | TEXT | ISO 8601 UTC timestamp (e.g., `2026-05-09T17:33:37Z`) |
| `image` | TEXT | Image filename (e.g., `default.jpg`). `NULL` if not set. Displayed as small inline thumbnail at top of article and on preview cards. |

**Table: `content_tags`** — Tags (one row per tag per article)
| Column | Type | Description |
|--------|------|-------------|
| `content_id` | INTEGER | FK → `content.id` |
| `tag` | TEXT | Single tag (e.g., `goblin`, `lore`) |
| *(composite PK)* | | `(content_id, tag)` |

**Table: `content_references`** — Cross-references (one row per target slug per article)
| Column | Type | Description |
|--------|------|-------------|
| `content_id` | INTEGER | FK → `content.id` |
| `ref_slug` | TEXT | Target article slug |
| *(composite PK)* | | `(content_id, ref_slug)` |

**Table: `content_sources`** — External sources (one row per source per article)
| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER PK AUTO | Auto-incrementing ID |
| `content_id` | INTEGER | FK → `content.id` |
| `source_name` | TEXT | Display name for external source |
| `source_url` | TEXT | URL for external source |

**Table: `dynamic_pages`**
| Column | Type | Description |
|--------|------|-------------|
| `path` | TEXT PK | The URL path (e.g., `ai-takeover-sam-altman`) |
| `title` | TEXT | Generated title |
| `content` | TEXT | Generated HTML content body |
| `keywords` | TEXT | Comma-separated keywords extracted from path |

### Unified JSON Format

Each content file in `data/content/` follows this schema:

```json
{
  "id": "goblin_lore",
  "title": "Goblin Lore: The Ancient Tricksters",
  "slug": "goblin_lore",
  "body_markdown": "# Goblin Lore: The Ancient Tricksters\n\n...",
  "category": "lore",
  "tags": ["goblin", "lore"],
  "sources": [
    { "name": "Wikipedia - Goblin", "url": "https://en.wikipedia.org/wiki/Goblin" }
  ],
  "references": ["goblin-tricks", "goblin-schizophrenia", "slop-goblin-manifesto"],
  "is_dynamic": false,
  "date_added": "2026-05-09T17:33:37Z"
}
```

**Fields:**
- `id`: Unique identifier (same as slug)
- `title`: Human-readable page title
- `slug`: URL-friendly identifier (hyphenated)
- `body_markdown`: Raw Markdown content (auto-converted to HTML on load)
- `category`: Content category (lore, tricks, anime, pop_culture, ttrpg, games, visual_novels, linguistics, etc.)
- `tags`: JSON array of tag strings (defaults to `["goblin"]` if empty). Always an array, not a comma-separated string.
- `sources`: JSON array of `{name, url}` objects. External references (e.g., IMDb, MyAnimeList, Wikipedia). Displayed at bottom of content pages.
- `references`: JSON array of target slugs this article explicitly cross-references
- `is_dynamic`: Always `false` for stored content
- `date_added`: ISO 8601 UTC timestamp when the file was created

### Startup Sequence

1. `main()` calls `db::init_db("goblin_slop.db")` which drops old tables and creates fresh normalized schema (no migrations needed)
2. `json_content_loader::load_all_content("goblin_slop.db", "data/content")` reads every `.json` file in `data/content/`:
   - Each file is deserialized into `JsonContentEntry` via serde
   - Markdown body is converted to HTML via `pulldown_cmark`
   - `ContentEntry` is inserted/updated into the database
   - On success: prints `✅ Loaded content: {title} (slug: {slug}, date_added: {date})`
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
- `bundled` flag on `rusqlite` means SQLite is compiled with the project — no system library needed

### 5.2 `src/config.rs` — Configuration Module

This module loads all runtime configuration from environment variables, following the [12-factor app](https://12factor.net/config) methodology.

**Struct: `Config`**

| Field | Type | Env Variable | Default | Description |
|-------|------|-------------|---------|-------------|
| `host` | `String` | `GOBLIN_HOST` | `0.0.0.0` | Server bind address |
| `port` | `u16` | `GOBLIN_PORT` | `3000` | Server port |
| `db_path` | `String` | `GOBLIN_DB_PATH` | `goblin_slop.db` | SQLite database file path |
| `content_dir` | `String` | `GOBLIN_CONTENT_DIR` | `data/content` | Directory containing JSON content files |
| `static_dir` | `String` | `GOBLIN_STATIC_DIR` | `static` | Directory containing static assets |
| `base_url` | `String` | `GOBLIN_BASE_URL` | `http://goblin.geno.su` | Base URL for canonical links & sitemap |

### 5.3 `src/main.rs` — Server Entrypoint

**Module declarations:**
```rust
mod config;
mod db;
mod json_content_loader;
mod routes;
```

**`main()` function flow:**
```
1. Initialize tracing logger
2. Open/create SQLite database at "goblin_slop.db"
3. Wrap connection in Arc<Mutex<>> for thread safety
4. Load all unified JSON content from "data/content/" directory into DB (single loader)
5. Create AppState { db, base_url }
6. Build Axum router with all routes (including /sitemap.xml), nest static file service at /static
7. Bind TCP listener to 0.0.0.0:3000
8. Print startup info
9. Serve requests forever
```

**Error handling**: If DB init fails, the program panics. If content loading fails, it prints a warning but continues.

### 5.4 `src/db.rs` — Database Layer

**Structs:**

| Struct | Fields | Purpose |
|--------|--------|---------|
| `SourceRef` | `name, url` | Represents one external source. Stored in `content_sources` table. |
| `ContentEntry` | `id, slug, title, body_markdown, body_html, category, tags: Vec<String>, references: Vec<String>, sources: Vec<SourceRef>, is_dynamic, date_added, image` | Represents a content article with all related data from normalized tables. |
| `DynamicPage` | `path, title, content, keywords` | Represents a cached dynamically-generated page. |

**Functions:**

| Function | Signature | Description |
|----------|-----------|-------------|
| `init_db` | `(path: &str) -> SqlResult<Connection>` | Drops old tables, creates fresh normalized schema |
| `insert_content` | `(conn: &Connection, entry: &ContentEntry) -> SqlResult<i64>` | Inserts article + related tags/references/sources into normalized tables |
| `get_content_by_slug` | `(conn: &Connection, slug: &str) -> SqlResult<Option<ContentEntry>>` | SELECT by slug, hydrates all related data from normalized tables |
| `get_content_paginated` | `(conn: &Connection, page, per_page) -> SqlResult<Vec<ContentEntry>>` | Paginated SELECT with LIMIT/OFFSET, newest-first |
| `count_all_content` | `(conn: &Connection) -> SqlResult<u64>` | COUNT(*) of all content entries |
| `get_all_content` | `(conn: &Connection) -> SqlResult<Vec<ContentEntry>>` | SELECT all content, newest-first |
| `search_content` | `(conn: &Connection, query: &str) -> SqlResult<Vec<ContentEntry>>` | LIKE search across title, body, tags (including tag table) |
| `insert_dynamic_page` | `(conn: &Connection, page: &DynamicPage) -> SqlResult<()>` | INSERT OR REPLACE into dynamic_pages table |
| `get_dynamic_page` | `(conn: &Connection, path: &str) -> SqlResult<Option<DynamicPage>>` | SELECT by path, returns None if not found |

### 5.5 `src/json_content_loader.rs` — Unified JSON Content Loader

This is the single source of truth for all content loading. Replaces the old two-module approach (`content.rs` + `data_loader.rs`).

**Struct: `JsonContentEntry`** (input format)
```rust
pub struct JsonContentEntry {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    pub category: String,        // default: "general"
    pub tags: Vec<String>,       // default: ["goblin"]
    pub references: Vec<String>, // cross-reference target slugs
    pub sources: Vec<SourceRef>, // external sources [{name, url}]
    pub is_dynamic: bool,
    pub date_added: String,      // ISO 8601 UTC, default: "1970-01-01T00:00:00Z"
    pub image: Option<String>,   // Image filename (e.g., Some("default.jpg")), None if not set
}
```

**Function: `load_all_content`**
```rust
pub fn load_all_content(db_path: &str, content_dir: &str) -> Result<(), Box<dyn std::error::Error>>
```

Scans `content_dir` for all `.json` files. For each file:
1. Read and deserialize JSON into `JsonContentEntry` (serde)
2. Convert Markdown body to HTML via `pulldown_cmark::Parser`
3. Map to `ContentEntry` for database insertion
4. INSERT OR REPLACE into `content` table

**Processing logic:**
- Empty `tags` → defaults to `["goblin"]` before DB insert
- `sources` are stored in the `content_sources` table, one row per source
- `references` are stored in the `content_references` table, one row per target slug
- Sorts files alphabetically for deterministic load order
- Prints `✅` on success, `❌` on DB error, `⚠️` on invalid JSON

### 5.6 `src/routes/` — Route Handlers Module Directory

The `src/routes/` directory holds six files, split by concern:

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations, `AppState`, `create_router()` |
| `handlers.rs` | Shared `ApiResponse` type (all handler logic moved to `pages/`) |
| `templates.rs` | HTML page layout rendering (JSON-LD, nav, footer, canonical URLs, robots) |
| `content_templates.rs` | Text template arrays (titles, intros, bodies, verdicts) + related section generator |
| `references.rs` | Real page references + randomly-generated fake page reference engine |
| `generator.rs` | Coordinator: assembles dynamic page from templates + refs |
| `pages/` | **One file per route handler** — all handler logic lives here |

#### `src/routes/mod.rs`
Module declaration and router construction:

```rust
pub mod content_templates;
pub mod generator;
pub mod handlers;
pub mod pages;
pub mod references;
pub mod templates;
```

- Declares `AppState` struct with `db` and `base_url` fields
- `create_router(state: AppState) -> Router` — builds a two-layer router:
  1. **Inner router** (exact routes): `/`, `/search`, `/sitemap.xml`, `/raw/:slug`, `/api/content/:slug`, `/api/dynamic/*path`, `/api/search`, `/api/all`
  2. **Outer router** (fallback): Any path that doesn't match the inner router gets handled by `dynamic_fallback`

#### `src/routes/pages/` — One file per route handler

| File | Route | Purpose |
|------|-------|---------|
| `home.rs` | `GET /` | Query content with pagination (`?page=N`), 20 per page, newest-first by `date_added` |
| `sitemap.rs` | `GET /sitemap.xml` | XML sitemap listing home, search, all static content entries |
| `search.rs` | `GET /search?q=` | If q present: search DB, show results. If not: show search form |
| `raw.rs` | `GET /raw/:slug` | Return raw Markdown body as text/plain |
| `dynamic_fallback.rs` | `GET /*` | Underscores → 301 redirect, else check static or generate deterministic dynamic page |
| `api_content.rs` | `GET /api/content/:slug` | Return single ContentEntry as JSON |
| `api_dynamic.rs` | `GET /api/dynamic/*path` | Return DynamicPage as JSON (deterministic, no DB cache) |
| `api_search.rs` | `GET /api/search?q=` | Return search results as JSON |
| `api_all.rs` | `GET /api/all` | Return all ContentEntries as JSON |

`src/routes/handlers.rs` now only contains the shared `ApiResponse<T>` struct.

#### `src/routes/templates.rs`
HTML template rendering functions:

| Function | Signature | Purpose |
|----------|-----------|---------|
| `build_head` | `(title, desc, canonical_path, base_url, robots, schema_type, schema_name, schema_desc, keywords) -> String` | Shared head builder — constructs absolute canonical URL from `base_url` + path, fills all template placeholders |
| `render_content_page` | `(entry: &ContentEntry, canonical_path: &str, base_url: &str) -> String` | Renders a static content page with JSON-LD |
| `render_dynamic_page` | `(dyn_page: &DynamicPage, canonical_path: &str, base_url: &str) -> String` | Renders a dynamically-generated page **without revealing it was generated** — no badges, no summon text, no AI notes |
| `render_static_page` | `(title, body_html, category, tags, canonical_path, base_url) -> String` | Renders a simple page from raw HTML body (for home, search, all) |

All pages include:
- `<meta name="robots" content="index, follow">`
- `<link rel="canonical" href="...">` with full absolute URL

**Key design principle: The user must never know pages are generated.** All dynamic/metadata markers from `render_dynamic_page()` have been removed.

#### `src/routes/generator.rs` (Coordinator)
Thin module that imports from `content_templates` and `references` and assembles the final `DynamicPage`. Two public functions:

**`generate_dynamic_page_content(path, keywords) → DynamicPage`** — selects random title/intro/body/verdict from `content_templates`, fills `{keyword}`, generates related sections, appends references from `references::generate_references_html()`, assembles everything into HTML.

**`parse_path_into_keywords(path) → Vec<String>`** — splits URL path by `/`, `-`, `_`, lowercases, filters stop words and short words.

#### `src/routes/content_templates.rs`
Pure data module — static text template arrays and a helper:

| Array | Size | Purpose |
|-------|------|---------|
| `GOBLIN_TITLES` | ~60 | Title templates with `{keyword}` placeholder (10 themes × 5-7 each) |
| `GOBLIN_INTROS` | ~24 | Introductory paragraph templates (7 narrative voices) |
| `GOBLIN_BODIES` | ~22 | Body paragraph templates (7 writing styles) |
| `VERDICT_TEMPLATES` | 4 | Goblin Verdict conclusion variants |
| `RELATED_SECTION_FORMATS` | 5 | Format strings for related-keyword sections |

#### `src/routes/references.rs`
Cross-reference engine:

| Constant | Size | Purpose |
|----------|------|---------|
| `REAL_PAGE_REFERENCES` | 29 | Known content pages (static + scraped) |
| `FAKE_SLUG_PARTS_A` | ~45 | First-word pool for slug generation |
| `FAKE_SLUG_PARTS_B` | ~40 | Second-word pool for slug generation |
| `FAKE_TITLE_TEMPLATES` | 20 | `{A}/{B}` title templates for fake refs |

**`generate_references_html(keywords) -> String`** — delegates to `generate_references_html_ex(keywords, None)`.

**`generate_references_html_ex(keywords, exclude_slug) -> String`** — the core algorithm:

1. **Keyword-matched real refs**: Scans all `REAL_PAGE_REFERENCES` for slugs that contain any keyword or vice versa. Skips the `exclude_slug` if provided (prevents self-references on content pages).
2. **Fill to at least 3 real refs**: After keyword matching, if there are fewer than 3-4 (random) matched real refs, the remainder are picked randomly from all pages. This **guarantees every article always gets multiple real cross-reference links**.
3. **Truncate if over 4**: If more than 3-4 matched refs, shuffle and keep only the desired count.
4. **Generate 3-5 fake refs**: Uses `generate_random_fake_ref()` which picks random words from `FAKE_SLUG_PARTS_A` and `FAKE_SLUG_PARTS_B` plus random title templates from `FAKE_TITLE_TEMPLATES`.
5. **Render unified block**: All real and fake references are rendered in a single `<section class='references-section'>` with identical CSS — **no visual distinction between real and fake**.

**Key property**: Every page — static or dynamic — always gets multiple real links (keyword-matched + random fill) and multiple fake links in one block. No article is left reference-less.

### 5.7 `static/styles.css` — Styling

| Section | Key Properties |
|---------|---------------|
| **Body** | `background: #1a1a2e`, `color: #e0e0e0`, serif font |
| **Navigation** | Gradient background `#16213e→#0f3460→#1a1a2e`, red bottom border `#e94560`, sticky top |
| **Content card** | Dark background `#16213e`, rounded corners, subtle shadow |
| **Headings** | Red `#e94560` color for h1/h2, lighter `#a0a0c0` for h3 |
| **Strong text** | Gold `#ffd700` |
| **Links** | Blue `#4fc3f7`, red hover `#e94560` |
| **Images (page body)** | `max-width: 100%`, `height: auto`, `display: inline`, rounded corners — prevents overflow and keeps images inline with text |
| **Article image** | `.article-img`: `max-width: 200px`, centered, small inline thumbnail at top of article |
| **Card image** | `.card-img`: `max-height: 120px`, centered, thumbnail on home page preview cards |
| **Code blocks** | Dark background `#0f3460` |
| **Search form** | Dark input with red focus border, red submit button |
| **Footer** | Dark background, centered, subtle text |

### 5.8 `static/robots.txt` — SEO

```
User-agent: *
Allow: /
```

Full crawl allowed. The sitemap is discoverable via `/sitemap.xml`.

### 5.9 `data/content/*.json` — Unified Content Library

37 individual JSON files, each containing one content unit. Split from two old sources (hand-crafted Markdown + scraped JSON) into a single format:

| Category | Count | Examples |
|----------|-------|----------|
| **Hand-crafted** | 16 | goblin_lore, goblin_tricks, goblin_schizophrenia, slop_goblin_manifesto, altman_miku_goblin_king, etc. |
| **Scraped** | 21 | goblin-slayer-anime, labyrinth-goblin-king, dungeons-and-dragons-goblins, warcraft-goblins, etc. |

---

## API Reference

### Web (HTML) Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/` | GET | Home page with content listing | HTML 200 |
| `/search?q=...` | GET | Full-text search results | HTML 200 |
| `/sitemap.xml` | GET | XML sitemap with all pages | XML 200 |
| `/raw/:slug` | GET | Raw Markdown source | text/plain 200 |
| `/*` | GET | Any path → goblin page (seamless, no "generated" indicators) | HTML 200 |

### JSON API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/api/all` | GET | All content entries | `{ success: true, data: [...] }` |
| `/api/content/:slug` | GET | Single entry by slug | `{ success: true/false, data: {...} }` |
| `/api/dynamic/*path` | GET | Dynamic page by path | `{ success: true, data: {...} }` |
| `/api/search?q=...` | GET | Search results | `{ success: true, data: [...] }` |

### Response Format

All JSON responses follow a consistent structure:

```json
{
  "success": true|false,
  "data": <ContentEntry>|<DynamicPage>|[...]
}
```

---

## Dynamic Page Generation Algorithm

When a path doesn't match any static content:

1. Parse path into keywords (split by `/`, `-`, `_`, lowercase, filter stop words)
2. Derive a deterministic RNG seed from the URL path (via `DefaultHasher`)
3. Select templated text from arrays: title, intro, body, verdict (using seeded RNG)
4. Fill `{keyword}` placeholders with the input keywords
5. Generate related keyword sections using `generate_related_section()`
6. Cross-reference real pages + generate fake pages (identical CSS)
7. Render full HTML page

**Key property**: The seeded RNG ensures that the same URL path always produces identical output. No DB caching is needed. Generation is purely pure-functional — deterministic from path alone.

All dynamic pages appear identical to static pages — no badges, notes, or indicators of generation.

---

## Testing & Verification

### Unit Tests

Located in `src/json_content_loader.rs` and `src/routes/generator.rs`:


**`test_deserialize_single_content_unit`** — Loads one actual JSON file, deserializes into `JsonContentEntry`, verifies all fields: id, slug, title, tags (array), references (array), date_added (ISO 8601), is_dynamic.

**`test_load_and_read_entry_with_all_fields`** — Loads all 41 JSON files into a fresh DB, reads back `goblin-slayer-anime`, verifies all normalized fields: tags (from `content_tags` table), references (from `content_references` table), sources (from `content_sources` table) with proper names and URLs.

**`test_generated_content_is_deterministic`** — Generates dynamic content twice from the same path and asserts byte-for-byte identical output.

**`test_different_paths_produce_different_content`** — Generates dynamic content from two different paths and asserts outputs differ.

```bash
cargo test --release
# running 4 tests
# test json_content_loader::tests::test_deserialize_single_content_unit ... ok
# test routes::generator::tests::test_different_paths_produce_different_content ... ok
# test routes::generator::tests::test_generated_content_is_deterministic ... ok
# test json_content_loader::tests::test_load_and_read_entry_with_all_fields ... ok
# test result: ok. 4 passed; 0 failed; 0 ignored
```

---

## Deployment

### Build & Deploy

The `deploy/build-and-deploy.sh` script:
1. Runs `cargo build --release`
2. SCP's the binary to the VPS
3. SSH's into the VPS and restarts the systemd service

### Environment Variables (Deployment)

| Variable | Description | Example |
|----------|-------------|---------|
| `DEPLOY_USER` | Remote SSH user | `goblin` |
| `DEPLOY_HOST` | VPS hostname/IP | `goblin.geno.su` |
| `APP_USER` | Systemd service user | `goblin` |

---

## Design Decisions & Trade-offs

### Single Unified Content Format (v0.1.0+)

**Before:** Two separate content sources — Markdown files in `content/` and a single JSON array in `data/scraped_content.json`. Each had its own loader, format, and processing logic.

**After:** All 37 entries are individual `.json` files in `data/content/`, following the same schema. One unified loader handles everything.

**Benefits:**
- Single source of truth for content structure
- Easier to add new content (just create a JSON file)
- Each unit is independently versionable/editable
- `date_added` field provides chronological metadata
- No need to manage two different loading pipelines

### SQLite Thread Safety

Using `Arc<Mutex<Connection>>` means only one request at a time accesses the database. This is acceptable for low traffic but would bottleneck under heavy load. A connection pool (`r2d2`) could be added later.

---

## How to Extend

### Adding New Content

1. Create a new file in `data/content/` named `{slug}.json`
2. Fill the unified format:
```json
{
  "id": "my-new-content",
  "title": "My Cool Goblin Article",
  "slug": "my-new-content",
  "body_markdown": "# My Cool Goblin Article\n\nContent here...",
  "category": "general",
  "tags": ["goblin", "example"],
  "sources": [
    { "name": "Source Name", "url": "https://example.com" }
  ],
  "references": ["other-article-slug"],
  "is_dynamic": false,
  "date_added": "2026-05-09T00:00:00Z"
}
```
3. Restart the server (or just restart — content is loaded at startup)

### Adding New Routes

1. Create a new handler file in `src/routes/pages/`
2. Add its module declaration to `src/routes/pages/mod.rs`
3. Register the route in `src/routes/mod.rs` inside `create_router()`
4. The fallback ensures no path returns 404 regardless

---

## How to Run

```bash
# Build
cargo build --release

# Run (default: 0.0.0.0:3000)
./target/release/goblin_slop

# Run with custom config
GOBLIN_HOST=127.0.0.1 GOBLIN_PORT=8080 ./target/release/goblin_slop

# Run tests
cargo test --release
```
