use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::db::{self, ContentEntry};
use super::super::AppState;
use super::super::handlers::ApiResponse;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
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
    })
}