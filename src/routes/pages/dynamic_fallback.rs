use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use axum::http::StatusCode;
use rand::thread_rng;

use crate::db;
use super::super::generator::{generate_dynamic_page_content, parse_path_into_keywords};
use super::super::templates::{render_content_page, render_dynamic_page};
use super::super::AppState;

fn normalize_slug(slug: &str) -> String {
    slug.replace('_', "-")
}

fn is_canonical(slug: &str) -> bool {
    !slug.contains('_')
}

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
    
    // Pick random image from pool for dynamic/generate pages
    let mut rng = thread_rng();
    Ok(Html(render_dynamic_page(&dyn_page, &format!("/{}", slug), &state.base_url, &mut rng)).into_response())
}