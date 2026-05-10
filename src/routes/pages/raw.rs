use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::db;
use super::super::AppState;

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