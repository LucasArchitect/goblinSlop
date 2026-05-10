use axum::{
    extract::State,
    Json,
};

use crate::db::{self, ContentEntry};
use super::super::AppState;
use super::super::handlers::ApiResponse;

pub async fn api_all(State(state): State<AppState>) -> Json<ApiResponse<Vec<ContentEntry>>> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();
    Json(ApiResponse {
        success: true,
        data: entries,
    })
}