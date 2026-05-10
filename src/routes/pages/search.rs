use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use axum::http::StatusCode;

use crate::db;
use super::super::templates::render_static_page;
use super::super::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
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

    Ok(Html(render_static_page(
        "Search GoblinSlop",
        &body,
        "search",
        "search",
        "/search",
        &state.base_url,
    )))
}