use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};

use crate::db;
use super::super::AppState;

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