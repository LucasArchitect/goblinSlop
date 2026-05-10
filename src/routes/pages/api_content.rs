use axum::{
    extract::{Path, State},
    Json,
};

use crate::db::{self, ContentEntry};
use super::super::AppState;
use super::super::handlers::ApiResponse;

pub async fn api_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Json<ApiResponse<Option<ContentEntry>>> {
    let db = state.db.lock().unwrap();
    let entry = db::get_content_by_slug(&db, &slug).unwrap_or(None);
    Json(ApiResponse {
        success: entry.is_some(),
        data: entry,
    })
}