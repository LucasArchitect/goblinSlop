pub mod content_templates;
pub mod generator;
pub mod handlers;
pub mod references;
pub mod templates;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use handlers::{
    api_all, api_content, api_dynamic, api_search,
    dynamic_fallback, home_page, raw_content, search_page, sitemap,
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<std::sync::Mutex<rusqlite::Connection>>,
    pub base_url: String,
}

pub fn create_router(state: AppState) -> Router {
    let app = Router::new()
        .route("/", get(home_page))
        .route("/search", get(search_page))
        .route("/raw/:slug", get(raw_content))
        .route("/api/content/:slug", get(api_content))
        .route("/api/dynamic/*path", get(api_dynamic))
        .route("/api/search", get(api_search))
        .route("/sitemap.xml", get(sitemap))
        .route("/api/all", get(api_all))
        .with_state(state.clone());

    Router::new()
        .fallback(get(dynamic_fallback))
        .with_state(state)
        .merge(app)
}