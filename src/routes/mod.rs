pub mod content_templates;
pub mod generator;
pub mod handlers;
pub mod pages;
pub mod references;
pub mod templates;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use pages::{
    api_all::api_all,
    api_content::api_content,
    api_dynamic::api_dynamic,
    api_search::api_search,
    category::category_page,
    dynamic_fallback::dynamic_fallback,
    home::home_page,
    raw::raw_content,
    search::search_page,
    sitemap::sitemap,
    tag::tag_page,
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
        .route("/tag/:tag", get(tag_page))
        .route("/category/:category", get(category_page))
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
