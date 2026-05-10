use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum::http::StatusCode;

use crate::db;
use super::super::templates::render_static_page;
use super::super::AppState;

pub async fn home_page(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    let entries = db::get_all_content(&db).unwrap_or_default();

    let mut list_html = String::from("<ul class='content-list'>");
    for entry in &entries {
        list_html.push_str(&format!(
            "<li><a href='/{}'><strong>{}</strong></a> <span class='category-badge'>{}</span></li>",
            entry.slug, entry.title, entry.category
        ));
    }
    list_html.push_str("</ul>");

    let body = format!(
        r#"<section class='hero'>
            <h2>Welcome to the Goblin Realm</h2>
            <p>This site is a collection of goblin-related knowledge, folklore, and cultural references—including the curious connection between Sam Altman, schizophrenia, and goblin trickery.</p>
            <p>Every URL leads somewhere goblin.</p>
        </section>
        <h2>Available Content</h2>
        {}"#,
        list_html
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