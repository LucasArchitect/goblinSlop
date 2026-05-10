use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use axum::http::StatusCode;

use crate::db;
use super::super::templates::render_static_page;
use super::super::AppState;

const PER_PAGE: u64 = 12;

/// Strip leading Markdown heading markers (#, ##) and return first ~240 chars of plain-ish text.
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

#[derive(Deserialize, Default)]
pub struct HomeQuery {
    pub page: Option<u64>,
}

pub async fn home_page(
    State(state): State<AppState>,
    Query(params): Query<HomeQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    let current_page = params.page.unwrap_or(1).max(1);

    let entries = db::get_content_paginated(&db, current_page, PER_PAGE).unwrap_or_default();
    let total = db::count_all_content(&db).unwrap_or(0);
    let total_pages = (total + PER_PAGE - 1) / PER_PAGE;

    let mut cards_html = String::from("<div class='article-grid'>");
    for entry in &entries {
        let date_str = if entry.date_added.len() >= 10 {
            &entry.date_added[..10]
        } else {
            &entry.date_added
        };
        let preview = make_preview(&entry.body_markdown);
        let tag_str = entry.tags.join(" · ");

        cards_html.push_str(&format!(
            r#"<a href='/{}' class='article-card'>
                <div class='card-header'>
                    <span class='card-date'>{}</span>
                    <span class='category-badge'>{}</span>
                </div>
                <h3 class='card-title'>{}</h3>
                <p class='card-preview'>{}</p>
                <div class='card-footer'>
                    <span class='card-tags'>{}</span>
                </div>
            </a>"#,
            entry.slug,
            date_str,
            entry.category,
            entry.title,
            preview,
            tag_str,
        ));
    }
    cards_html.push_str("</div>");

    // Pagination nav
    let mut pagination_html = String::new();
    if total_pages > 1 {
        pagination_html.push_str(&format!(
            "<nav class='pagination'><span class='pagination-info'>Page {} of {}</span>",
            current_page, total_pages
        ));

        if current_page > 1 {
            let prev = current_page - 1;
            pagination_html.push_str(&format!(
                "<a href='/?page={prev}' class='pagination-link'>&laquo; Previous</a>"
            ));
        }

        let start_page = if current_page > 2 { current_page - 2 } else { 1 };
        let end_page = std::cmp::min(start_page + 4, total_pages);

        for p in start_page..=end_page {
            if p == current_page {
                pagination_html.push_str(&format!("<span class='pagination-current'>{p}</span>"));
            } else {
                pagination_html.push_str(&format!("<a href='/?page={p}' class='pagination-link'>{p}</a>"));
            }
        }

        if current_page < total_pages {
            let next = current_page + 1;
            pagination_html.push_str(&format!("<a href='/?page={next}' class='pagination-link'>Next &raquo;</a>"));
        }

        pagination_html.push_str("</nav>");
    }

    let body = format!(
        r#"<section class='hero'>
            <h2>🧌 Welcome to the Goblin Realm</h2>
            <p>A collection of goblin-related knowledge, folklore, and cultural references — including the curious connection between Sam Altman, schizophrenia, and goblin trickery.</p>
            <p>Every URL leads somewhere goblin.</p>
        </section>
        <div class='section-header'>
            <h2>Available Content</h2>
            <span class='count-badge'>{} article{}</span>
        </div>
        {}
        {}"#,
        total,
        if total == 1 { "" } else { "s" },
        cards_html,
        pagination_html,
    );

    Ok(Html(render_static_page(
        "GoblinSlop — A Library of Goblin Lore",
        &body,
        "home",
        "goblins,home,welcome",
        "/",
        &state.base_url,
    )))
}
