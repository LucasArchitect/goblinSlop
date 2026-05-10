use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use axum::http::StatusCode;

use crate::db;
use super::super::templates::{render_static_page, render_tags, render_category};
use super::super::AppState;

/// Strip leading Markdown heading markers and return first ~240 chars of plain-ish text.
fn make_preview(markdown: &str) -> String {
    let body = markdown
        .lines()
        .find(|l| {
            let trimmed = l.trim();
            !trimmed.starts_with('#')
                && !trimmed.starts_with("---")
                && !trimmed.is_empty()
        })
        .unwrap_or("");
    let body = body.trim();
    let body = body
        .replace("**", "")
        .replace("__", "")
        .replace("~~", "")
        .replace("*", "")
        .replace("`", "");
    if body.len() > 240 {
        format!("{}…", &body[..237])
    } else {
        body.to_string()
    }
}

pub async fn category_page(
    State(state): State<AppState>,
    Path(category): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();

    let entries = db::get_content_by_category(&db, &category).unwrap_or_default();
    let count = entries.len();

    let mut body = format!(
        "<h2>Category: <span class='category-link'>{}</span></h2>",
        category,
    );

    if count == 0 {
        body.push_str("<p>No articles found in this category.</p>");
    } else {
        body.push_str(&format!("<p>{} article{} found.</p>", count, if count == 1 { "" } else { "s" }));
        body.push_str("<div class='article-grid'>");
        for entry in &entries {
            let date_str = if entry.date_added.len() >= 10 {
                &entry.date_added[..10]
            } else {
                &entry.date_added
            };
            let img_file = entry.image.as_deref().unwrap_or("default.jpg");
            let tag_links = render_tags(&entry.tags);
            let cat_link = render_category(&entry.category);
            let preview = make_preview(&entry.body_markdown);

            body.push_str(&format!(
                r#"<div class='article-card'>
                    <div class='card-top'>
                        <div class='card-image'>
                            <img src="/static/images/{}" alt="{}" class="card-img">
                        </div>
                        <div class='card-top-right'>
                            <a href='/{}' class='card-title-link'><h3 class='card-title'>{}</h3></a>
                            <div class='card-header'>
                                <span class='card-date'>{}</span>
                                {}
                            </div>
                        </div>
                    </div>
                    <p class='card-preview'>{}</p>
                    <div class='card-footer'>
                        <span class='card-tags'>{}</span>
                    </div>
                </div>"#,
                img_file,
                entry.title,
                entry.slug,
                entry.title,
                date_str,
                cat_link,
                preview,
                tag_links,
            ));
        }
        body.push_str("</div>");
    }

    Ok(Html(render_static_page(
        &format!("Category: {} - GoblinSlop", category),
        &body,
        "category",
        &category,
        &format!("/category/{}", category),
        &state.base_url,
    )))
}