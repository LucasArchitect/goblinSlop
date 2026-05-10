use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Redirect},
};
use serde::{Deserialize, Serialize};

use crate::db::{self, ContentEntry, DynamicPage};
use super::generator::{generate_dynamic_page_content, parse_path_into_keywords};
use super::templates::{render_content_page, render_dynamic_page, render_static_page};
use super::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub source: String,
}

/// Normalize a slug: replace underscores with hyphens for canonical form
fn normalize_slug(slug: &str) -> String {
    slug.replace('_', "-")
}

/// Check if a slug is already in canonical (hyphen) form
fn is_canonical(slug: &str) -> bool {
    !slug.contains('_')
}

pub async fn home_page(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, String)> {
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
            <p>Every URL leads somewhere goblin.</p>
        </section>
        <h2>Available Content</h2>
        {}"#,
        list_html
    );

    Ok(Html(render_static_page("GoblinSlop — A Library of Goblin Lore", &body, "home", "goblins,home,welcome", "/", &state.base_url)))
}

pub async fn sitemap(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();
    let base_url = state.base_url.trim_end_matches('/').to_string();

    let mut urls = String::new();
    urls.push_str(&format!(
        r#"<url><loc>{}/</loc><changefreq>daily</changefreq><priority>1.0</priority></url>"#,
        base_url
    ));
    urls.push_str(&format!(
        r#"<url><loc>{}/search</loc><changefreq>weekly</changefreq><priority>0.5</priority></url>"#,
        base_url
    ));

    for entry in &entries {
        urls.push_str(&format!(
            r#"<url><loc>{}/{}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>"#,
            base_url, entry.slug
        ));
    }

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#,
        urls
    );

    Ok(([(axum::http::header::CONTENT_TYPE, "application/xml")], xml))
}

/// Fallback handler: any unknown path generates a dynamic goblin page
pub async fn dynamic_fallback(
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<axum::response::Response, (StatusCode, String)> {
    let uri = req.uri().path_and_query()
        .map(|pq| pq.path().to_string())
        .unwrap_or_else(|| "/".to_string());

    let slug = uri.trim_start_matches('/').to_string();

    if slug.is_empty() {
        return Err((StatusCode::PERMANENT_REDIRECT, String::new()));
    }

    // If slug contains underscore, redirect to canonical hyphen form
    if !is_canonical(&slug) {
        let canonical = normalize_slug(&slug);
        return Ok(Redirect::permanent(&format!("/{}", canonical)).into_response());
    }

    let db = state.db.lock().unwrap();

    // Check static content
    let entry = db::get_content_by_slug(&db, &slug).unwrap_or(None);
    if let Some(entry) = entry {
        return Ok(Html(render_content_page(&entry, &format!("/{}", slug), &state.base_url)).into_response());
    }

    // Generate new (deterministic from path — no DB caching needed)
    let keywords = parse_path_into_keywords(&slug);
    let final_keywords = if keywords.is_empty() {
        vec!["goblin".to_string(), "mystery".to_string(), slug.clone()]
    } else {
        keywords
    };

    let dyn_page = generate_dynamic_page_content(&slug, &final_keywords);

    Ok(Html(render_dynamic_page(&dyn_page, &format!("/{}", slug), &state.base_url)).into_response())
}

pub async fn search_page(
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
            <p>Or explore any hidden goblin path!</p>"#
        )
    };

    Ok(Html(render_static_page("Search GoblinSlop", &body, "search", "search", "/search", &state.base_url)))
}

pub async fn raw_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    match db::get_content_by_slug(&db, &slug).unwrap_or(None) {
        Some(entry) => Ok(entry.body_markdown),
        None => Err((StatusCode::NOT_FOUND, format!("No content found for: {}", slug))),
    }
}

pub async fn api_content(
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

pub async fn api_dynamic(
    State(_state): State<AppState>,
    Path(path): Path<String>,
) -> Json<ApiResponse<Option<DynamicPage>>> {
    let keywords = parse_path_into_keywords(&path);
    let final_keywords = if keywords.is_empty() {
        vec!["goblin".to_string(), "mystery".to_string(), path.clone()]
    } else {
        keywords
    };

    let page = generate_dynamic_page_content(&path, &final_keywords);
    Json(ApiResponse {
        success: true,
        data: Some(page),
        source: "deterministic".to_string(),
    })
}

pub async fn api_search(
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

pub async fn api_all(State(state): State<AppState>) -> Json<ApiResponse<Vec<ContentEntry>>> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();
    Json(ApiResponse {
        success: true,
        data: entries,
        source: "static".to_string(),
    })
}