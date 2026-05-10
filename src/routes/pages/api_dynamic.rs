use axum::{
    extract::{Path, State},
    Json,
};

use crate::db::DynamicPage;
use super::super::generator::{generate_dynamic_page_content, parse_path_into_keywords};
use super::super::AppState;
use super::super::handlers::ApiResponse;

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
    })
}