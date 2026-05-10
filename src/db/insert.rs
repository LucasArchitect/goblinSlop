use rusqlite::{Connection, Result as SqlResult};

use super::types::ContentEntry;

pub fn insert_content(conn: &Connection, entry: &ContentEntry) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO content (slug, title, body_markdown, body_html, category, is_dynamic, date_added, image)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            entry.slug,
            entry.title,
            entry.body_markdown,
            entry.body_html,
            entry.category,
            entry.is_dynamic as i32,
            entry.date_added,
            entry.image,
        ],
    )?;
    let content_id = conn.last_insert_rowid();

    // Insert tags
    for tag in &entry.tags {
        conn.execute(
            "INSERT OR IGNORE INTO content_tags (content_id, tag) VALUES (?1, ?2)",
            rusqlite::params![content_id, tag],
        )?;
    }

    // Insert references
    for ref_slug in &entry.references {
        conn.execute(
            "INSERT OR IGNORE INTO content_references (content_id, ref_slug) VALUES (?1, ?2)",
            rusqlite::params![content_id, ref_slug],
        )?;
    }

    // Insert sources
    for src in &entry.sources {
        conn.execute(
            "INSERT INTO content_sources (content_id, source_name, source_url) VALUES (?1, ?2, ?3)",
            rusqlite::params![content_id, src.name, src.url],
        )?;
    }

    Ok(content_id)
}

#[allow(dead_code)]
pub fn insert_dynamic_page(
    conn: &Connection,
    path: &str,
    title: &str,
    content: &str,
    keywords: &[String],
) -> SqlResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO dynamic_pages (path, title, content, keywords)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![path, title, content, keywords.join(",")],
    )?;
    Ok(())
}