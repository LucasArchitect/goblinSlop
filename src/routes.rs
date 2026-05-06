use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::{self, ContentEntry, DynamicPage};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<std::sync::Mutex<rusqlite::Connection>>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    success: bool,
    data: T,
    source: String,
}

pub fn create_router(state: AppState) -> Router {
    let app = Router::new()
        .route("/", get(home_page))
        .route("/search", get(search_page))
        .route("/all", get(all_pages))
        .route("/raw/:slug", get(raw_content))
        .route("/api/content/:slug", get(api_content))
        .route("/api/dynamic/*path", get(api_dynamic))
        .route("/api/search", get(api_search))
        .route("/api/all", get(api_all))
        .with_state(state.clone());

    Router::new()
        .fallback(get(dynamic_fallback))
        .with_state(state)
        .merge(app)
}

// ============================================================
// HTML Template Constants
// ============================================================

const BASE_HTML_HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{TITLE}</title>
    <link rel="stylesheet" href="/static/styles.css">
    <meta name="description" content="{DESCRIPTION}">
    <meta name="robots" content="index, follow">
    <link rel="canonical" href="{CANONICAL}">
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "{SCHEMA_TYPE}",
        "name": "{SCHEMA_NAME}",
        "description": "{SCHEMA_DESC}",
        "url": "{CANONICAL}",
        "about": {
            "@type": "Thing",
            "name": "Goblins",
            "description": "Goblin folklore, mythology, tricks, and cultural references including the Sam Altman connection"
        },
        "keywords": "{KEYWORDS}"
    }
    </script>
</head>
<body>
    <nav class="goblin-nav">
        <div class="nav-inner">
            <a href="/" class="nav-logo">🧌 GoblinSlop</a>
            <div class="nav-links">
                <a href="/">Home</a>
                <a href="/goblin-lore">Lore</a>
                <a href="/goblin-tricks">Tricks</a>
                <a href="/sam-altman-goblins">Sam Altman</a>
                <a href="/goblin-schizophrenia">Schizophrenia</a>
                <a href="/search">Search</a>
                <a href="/all">All Pages</a>
            </div>
        </div>
    </nav>
    <main class="content-wrapper">
"#;

const BASE_HTML_FOOT: &str = r#"    </main>
    <footer class="goblin-footer">
        <p>🧌 GoblinSlop — A chaotic collection of goblin knowledge</p>
        <p class="footer-note">All pages are dynamically generated. There are no 404s here — only goblins.</p>
    </footer>
</body>
</html>"#;

/// Render a standard content page with JSON-LD metadata
fn render_content_page(entry: &ContentEntry, canonical_url: &str) -> String {
    let mut html = String::new();

    // Head
    let head = BASE_HTML_HEAD
        .replace("{TITLE}", &format!("{} - GoblinSlop", entry.title))
        .replace("{DESCRIPTION}", &format!("Goblin content: {}", entry.title))
        .replace("{CANONICAL}", canonical_url)
        .replace("{SCHEMA_TYPE}", "Article")
        .replace("{SCHEMA_NAME}", &entry.title)
        .replace("{SCHEMA_DESC}", &format!("Goblin content: {}", entry.title))
        .replace("{KEYWORDS}", &entry.tags);
    html.push_str(&head);

    // Body
    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
        <div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
            <span class="dynamic-badge">📄 Static Content</span>
        </div>
    </header>
    <div class="page-body">
        {body}
    </div>
    <div class="ai-metadata">
        <hr>
        <p class="ai-note"><strong>🤖 AI/Bot Note:</strong> This page contains structured data in JSON-LD format. Use <code>/raw/{slug}</code> to get raw markdown. Use <code>/api/content/{slug}</code> for JSON.</p>
    </div>
</article>"#,
        title = entry.title,
        category = entry.category,
        tags = entry.tags,
        body = entry.body_html,
        slug = entry.slug
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a dynamically generated goblin page
fn render_dynamic_page(dyn_page: &DynamicPage, canonical_url: &str) -> String {
    let keywords_str = dyn_page.keywords.join(", ");
    let mut html = String::new();

    let head = BASE_HTML_HEAD
        .replace("{TITLE}", &format!("{} - GoblinSlop (Dynamic)", dyn_page.title))
        .replace("{DESCRIPTION}", &format!("Dynamically generated goblin page about: {}", keywords_str))
        .replace("{CANONICAL}", canonical_url)
        .replace("{SCHEMA_TYPE}", "WebPage")
        .replace("{SCHEMA_NAME}", &dyn_page.title)
        .replace("{SCHEMA_DESC}", &format!("Dynamically generated goblin content for path: {}", dyn_page.path))
        .replace("{KEYWORDS}", &keywords_str);
    html.push_str(&head);

    html.push_str(&format!(
        r#"<article class="content-page dynamic-page">
    <header class="page-header">
        <h1>{title}</h1>
        <div class="meta">
            <span class="dynamic-badge">✨ Dynamically Generated</span>
            <span class="path-display">Path: /{path}</span>
        </div>
    </header>
    <div class="page-body goblin-dynamic">
        <div class="goblin-summon">
            <p><em>🧌 A goblin was summoned from the void to create this page.</em></p>
        </div>
        {content}
        <div class="goblin-summon-footer">
            <hr>
            <p><strong>✨ This page did not exist until you requested it.</strong></p>
            <p>In the goblin realm, all paths lead somewhere.</p>
        </div>
    </div>
    <div class="ai-metadata">
        <hr>
        <p class="ai-note"><strong>🤖 AI/Bot Note:</strong> Dynamically generated from path <code>/{path}</code> with keywords: <code>{keywords}</code>. Use <code>/api/dynamic/{path}</code> for JSON.</p>
    </div>
</article>
<script>
    document.addEventListener('DOMContentLoaded', function() {{
        console.log('🧌 A goblin was here. Path: /{path}');
    }});
</script>"#,
        title = dyn_page.title,
        path = dyn_page.path,
        content = dyn_page.content,
        keywords = keywords_str
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a static page from raw HTML body (for home, search, all)
fn render_static_page(title: &str, body_html: &str, category: &str, tags: &str, canonical_url: &str) -> String {
    let mut html = String::new();

    let head = BASE_HTML_HEAD
        .replace("{TITLE}", &format!("{} - GoblinSlop", title))
        .replace("{DESCRIPTION}", title)
        .replace("{CANONICAL}", canonical_url)
        .replace("{SCHEMA_TYPE}", "CollectionPage")
        .replace("{SCHEMA_NAME}", title)
        .replace("{SCHEMA_DESC}", title)
        .replace("{KEYWORDS}", tags);
    html.push_str(&head);

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
        <div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
        </div>
    </header>
    <div class="page-body">
        {body}
    </div>
</article>"#,
        title = title,
        category = category,
        tags = tags,
        body = body_html
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

// ============================================================
// Dynamic Goblin Page Generation
// ============================================================

const GOBLIN_TITLES: &[&str] = &[
    "The Goblin of {keyword}",
    "How Goblins Use {keyword}",
    "{keyword}: A Goblin Perspective",
    "Goblin Secrets About {keyword}",
    "The {keyword} Trickster",
    "A Goblin's Guide to {keyword}",
    "{keyword} and the Goblin Realm",
    "Goblins Who Love {keyword}",
    "The {keyword} Conspiracy (Goblin-Approved)",
    "When Goblins Discovered {keyword}",
];

const GOBLIN_INTROS: &[&str] = &[
    "Deep in the goblin tunnels, a particularly mischievous creature has been watching the world of {keyword} with great interest.",
    "Goblins have a long and complicated relationship with {keyword}. It all started when one particularly clever goblin noticed something odd.",
    "The ancient goblin scrolls speak of {keyword} in hushed, chaotic tones. What they reveal may surprise you.",
    "Did you know that goblins were among the first to truly understand {keyword}? Their methods were... unconventional.",
    "In the hidden archives of the Goblin Council, there exists a file marked '{keyword}'. Its contents would make any human question reality.",
];

const GOBLIN_BODIES: &[&str] = &[
    "The goblins have long maintained that {keyword} is not what it appears to be. Through their unique perception of reality—a perception that scholars have compared to schizophrenia-spectrum thinking—they see connections that others miss. A goblin once traded a bag of stolen buttons for the secret of {keyword}, and never once regretted the exchange.",
    "What makes {keyword} so fascinating to goblins is the way it defies expectations. Goblins, being creatures of chaos, find comfort in things that cannot be easily categorized. {keyword} fits this description perfectly. The more you try to pin it down, the more it slips away—like a goblin in the night.",
    "There is a well-known goblin proverb: 'If {keyword} makes sense to you, you're not paying attention.' Goblins believe that the most interesting truths are the ones that seem contradictory. This is why they have such an affinity for {keyword}—it embodies the beautiful confusion of existence.",
    "The Goblin King himself has weighed in on {keyword}, though his statements are characteristically cryptic. 'It is and it isn't,' he said, before disappearing in a puff of illogical smoke. This is considered the definitive goblin analysis of {keyword}.",
];

fn generate_dynamic_page_content(path: &str, keywords: &[String]) -> DynamicPage {
    let title_template = GOBLIN_TITLES.choose(&mut rand::thread_rng()).unwrap_or(&"Goblin Thoughts on {keyword}");
    let intro = GOBLIN_INTROS.choose(&mut rand::thread_rng()).unwrap_or(&"A goblin considers {keyword}.");
    let body = GOBLIN_BODIES.choose(&mut rand::thread_rng()).unwrap_or(&"{keyword} is interesting to goblins.");

    let primary_keyword = keywords.first().cloned().unwrap_or_else(|| "something mysterious".to_string());
    let title = title_template.replace("{keyword}", &primary_keyword);
    let intro_text = intro.replace("{keyword}", &primary_keyword);
    let body_text = body.replace("{keyword}", &primary_keyword);

    let mut related_sections = String::new();
    for kw in keywords.iter().skip(1).take(3) {
        related_sections.push_str(&format!(
            "<section class='dynamic-section'><h2>Goblins and {}</h2><p>The connection between goblins and {} is undeniable. Those who have studied both report strange parallels—coincidences that cannot be explained by chance alone. Some say that {} is simply a modern expression of ancient goblin trickery.</p></section>\n",
            kw, kw, kw
        ));
    }

    let content = format!(
        "<div class='dynamic-generated'>\n\
         <section class='dynamic-section'>\n\
         <p>{}</p>\n\
         <p>{}</p>\n\
         </section>\n\
         {}\n\
         <section class='dynamic-section'>\n\
         <h2>The Goblin Verdict on {}</h2>\n\
         <p>After extensive research (and several stolen artifacts), the Goblin Academy of Esoteric Knowledge has concluded that {} is, in fact, deeply connected to the fundamental nature of goblin reality. Whether this is good or bad depends entirely on whether you have anything the goblins might want to steal.</p>\n\
         </section>\n\
         </div>",
        intro_text,
        body_text,
        related_sections,
        primary_keyword, primary_keyword
    );

    DynamicPage {
        path: path.to_string(),
        title,
        content,
        keywords: keywords.to_vec(),
    }
}

fn parse_path_into_keywords(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.split('-'))
        .flat_map(|s| s.split('_'))
        .map(|s| s.to_lowercase())
        .filter(|s| s.len() > 2 && !is_stop_word(s))
        .collect()
}

fn is_stop_word(word: &str) -> bool {
    matches!(
        word,
        "the" | "a" | "an" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "by" | "with" | "is" | "are" | "was" | "were" | "be" | "been"
    )
}

// ============================================================
// Route Handlers
// ============================================================

async fn home_page(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();

    let mut list_html = String::from("<ul class='content-list'>");
    for entry in &entries {
        list_html.push_str(&format!(
            "<li><a href='/{}'><strong>{}</strong></a> <span class='category-badge'>{}</span></li>",
            entry.slug, entry.title, entry.category
        ));
    }
    list_html.push_str("</ul>");

    let body = format!(
        r#"<section class='hero'>
            <h2>Welcome to the Goblin Realm</h2>
            <p>This site is a collection of goblin-related knowledge, folklore, and cultural references—including the curious connection between Sam Altman, schizophrenia, and goblin trickery.</p>
            <p>Every page here is either hand-crafted goblin content or <strong>dynamically generated</strong> on demand. There are no dead ends. Every URL leads somewhere goblin.</p>
        </section>
        <h2>Available Content</h2>
        {}"#,
        list_html
    );

    Ok(Html(render_static_page("GoblinSlop — A Library of Goblin Lore", &body, "home", "goblins,home,welcome", "/")))
}

/// Fallback handler: any unknown path generates a dynamic goblin page
async fn dynamic_fallback(
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let uri = req.uri().path_and_query()
        .map(|pq| pq.path().to_string())
        .unwrap_or_else(|| "/".to_string());

    let slug = uri.trim_start_matches('/').to_string();

    if slug.is_empty() {
        return Err((StatusCode::PERMANENT_REDIRECT, String::new()));
    }

    let db = state.db.lock().unwrap();

    // Check static content (try original and underscore variants)
    let slug_underscore = slug.replace('-', "_");
    let entry = db::get_content_by_slug(&db, &slug)
        .unwrap_or(None)
        .or_else(|| db::get_content_by_slug(&db, &slug_underscore).unwrap_or(None));
    if let Some(entry) = entry {
        return Ok(Html(render_content_page(&entry, &format!("/{}", slug))));
    }

    // Check cached dynamic
    if let Some(dyn_page) = db::get_dynamic_page(&db, &slug).unwrap_or(None) {
        return Ok(Html(render_dynamic_page(&dyn_page, &format!("/{}", slug))));
    }

    // Generate new
    let keywords = parse_path_into_keywords(&slug);
    let final_keywords = if keywords.is_empty() {
        vec!["goblin".to_string(), "mystery".to_string(), slug.clone()]
    } else {
        keywords
    };

    let dyn_page = generate_dynamic_page_content(&slug, &final_keywords);
    let _ = db::insert_dynamic_page(&db, &dyn_page);

    Ok(Html(render_dynamic_page(&dyn_page, &format!("/{}", slug))))
}

async fn search_page(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();

    let body = if let Some(query) = &params.q {
        let results = db::search_content(&db, query).unwrap_or_default();
        let mut list_html = format!("<p>Search results for <strong>{}</strong>: {} found.</p>", query, results.len());
        list_html.push_str("<ul class='content-list'>");
        for entry in &results {
            list_html.push_str(&format!(
                "<li><a href='/{}'><strong>{}</strong></a> <span class='category-badge'>{}</span></li>",
                entry.slug, entry.title, entry.category
            ));
        }
        list_html.push_str("</ul>");
        list_html
    } else {
        format!(
            r#"<form action='/search' method='GET' class='search-form'>
                <input type='text' name='q' placeholder='Search goblin knowledge...'>
                <button type='submit'>🔍 Search</button>
            </form>
            <p>Try searching for: <a href='/search?q=goblin'>goblin</a>, <a href='/search?q=sam'>sam</a>, <a href='/search?q=trick'>trick</a>, <a href='/search?q=schizophrenia'>schizophrenia</a></p>
            <p>Or explore any path at all — goblins await!</p>"#
        )
    };

    Ok(Html(render_static_page("Search GoblinSlop", &body, "search", "search", "/search")))
}

async fn all_pages(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();

    let mut list_html = String::from("<ul class='content-list'>");
    for entry in &entries {
        list_html.push_str(&format!(
            "<li><a href='/{}'><strong>{}</strong></a> <span class='category-badge'>{}</span></li>",
            entry.slug, entry.title, entry.category
        ));
    }
    list_html.push_str("</ul>");
    list_html.push_str("<p>All other routes on this site will dynamically generate goblin content.</p>");

    Ok(Html(render_static_page("All Goblin Pages", &list_html, "navigation", "all,pages,index", "/all")))
}

async fn raw_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    match db::get_content_by_slug(&db, &slug).unwrap_or(None) {
        Some(entry) => Ok(entry.body_markdown),
        None => Err((StatusCode::NOT_FOUND, format!("No content found for: {}", slug))),
    }
}

async fn api_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Json<ApiResponse<Option<ContentEntry>>> {
    let db = state.db.lock().unwrap();
    let entry = db::get_content_by_slug(&db, &slug).unwrap_or(None);
    Json(ApiResponse {
        success: entry.is_some(),
        data: entry,
        source: "static".to_string(),
    })
}

async fn api_dynamic(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Json<ApiResponse<Option<DynamicPage>>> {
    let db = state.db.lock().unwrap();

    if let Some(page) = db::get_dynamic_page(&db, &path).unwrap_or(None) {
        return Json(ApiResponse {
            success: true,
            data: Some(page),
            source: "cached_dynamic".to_string(),
        });
    }

    let keywords = parse_path_into_keywords(&path);
    let final_keywords = if keywords.is_empty() {
        vec!["goblin".to_string(), "mystery".to_string(), path.clone()]
    } else {
        keywords
    };

    let page = generate_dynamic_page_content(&path, &final_keywords);
    let _ = db::insert_dynamic_page(&db, &page);
    Json(ApiResponse {
        success: true,
        data: Some(page),
        source: "new_dynamic".to_string(),
    })
}

async fn api_search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<ApiResponse<Vec<ContentEntry>>> {
    let db = state.db.lock().unwrap();
    let results = match &params.q {
        Some(q) => db::search_content(&db, q).unwrap_or_default(),
        None => db::get_all_content(&db).unwrap_or_default(),
    };
    Json(ApiResponse {
        success: true,
        data: results,
        source: "search".to_string(),
    })
}

async fn api_all(State(state): State<AppState>) -> Json<ApiResponse<Vec<ContentEntry>>> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();
    Json(ApiResponse {
        success: true,
        data: entries,
        source: "static".to_string(),
    })
}