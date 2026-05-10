use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

// ============================================================
// Types
// ============================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceRef {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentEntry {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body_markdown: String,
    pub body_html: String,
    pub category: String,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub sources: Vec<SourceRef>,
    pub is_dynamic: bool,
    pub date_added: String,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicPage {
    pub path: String,
    pub title: String,
    pub content: String,
    pub keywords: Vec<String>,
}

// ============================================================
// Schema
// ============================================================

pub fn init_db(path: &str) -> SqlResult<Connection> {
    let conn = Connection::open(path)?;

    // Delete old DB so we always start fresh (no migrations needed)
    conn.execute_batch(
        "DROP TABLE IF EXISTS content;
         DROP TABLE IF EXISTS content_tags;
         DROP TABLE IF EXISTS content_references;
         DROP TABLE IF EXISTS content_sources;
         DROP TABLE IF EXISTS dynamic_pages;
         DROP TABLE IF EXISTS keywords;

         CREATE TABLE content (
             id              INTEGER PRIMARY KEY AUTOINCREMENT,
             slug            TEXT NOT NULL UNIQUE,
             title           TEXT NOT NULL,
             body_markdown   TEXT NOT NULL,
             body_html       TEXT NOT NULL,
             category        TEXT NOT NULL DEFAULT 'general',
             is_dynamic      INTEGER NOT NULL DEFAULT 0,
             date_added      TEXT NOT NULL DEFAULT '1970-01-01T00:00:00Z',
             image           TEXT DEFAULT NULL
         );

         CREATE TABLE content_tags (
             content_id  INTEGER NOT NULL,
             tag         TEXT NOT NULL,
             PRIMARY KEY (content_id, tag),
             FOREIGN KEY (content_id) REFERENCES content(id)
         );

         CREATE TABLE content_references (
             content_id  INTEGER NOT NULL,
             ref_slug    TEXT NOT NULL,
             PRIMARY KEY (content_id, ref_slug),
             FOREIGN KEY (content_id) REFERENCES content(id)
         );

         CREATE TABLE content_sources (
             id          INTEGER PRIMARY KEY AUTOINCREMENT,
             content_id  INTEGER NOT NULL,
             source_name TEXT NOT NULL,
             source_url  TEXT NOT NULL DEFAULT '',
             FOREIGN KEY (content_id) REFERENCES content(id)
         );

         CREATE TABLE dynamic_pages (
             path TEXT PRIMARY KEY,
             title TEXT NOT NULL,
             content TEXT NOT NULL,
             keywords TEXT NOT NULL DEFAULT ''
         );",
    )?;

    Ok(conn)
}

// ============================================================
// Insert
// ============================================================

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

// ============================================================
// Helpers
// ============================================================

fn load_tags(conn: &Connection, content_id: i64) -> SqlResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT tag FROM content_tags WHERE content_id = ?1 ORDER BY tag",
    )?;
    Ok(stmt
        .query_map(rusqlite::params![content_id], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?)
}

fn load_references(conn: &Connection, content_id: i64) -> SqlResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT ref_slug FROM content_references WHERE content_id = ?1 ORDER BY ref_slug",
    )?;
    Ok(stmt
        .query_map(rusqlite::params![content_id], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?)
}

fn load_sources(conn: &Connection, content_id: i64) -> SqlResult<Vec<SourceRef>> {
    let mut stmt = conn.prepare(
        "SELECT source_name, source_url FROM content_sources WHERE content_id = ?1 ORDER BY id",
    )?;
    Ok(stmt
        .query_map(rusqlite::params![content_id], |row| {
            Ok(SourceRef {
                name: row.get(0)?,
                url: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?)
}

fn hydrate_row(conn: &Connection, row: &rusqlite::Row) -> SqlResult<ContentEntry> {
    let id: i64 = row.get(0)?;
    let image_raw: Option<String> = row.get(8)?;
    // Convert empty string to None for consistency
    let image = image_raw.filter(|s| !s.is_empty());
    Ok(ContentEntry {
        id,
        slug: row.get(1)?,
        title: row.get(2)?,
        body_markdown: row.get(3)?,
        body_html: row.get(4)?,
        category: row.get(5)?,
        tags: load_tags(conn, id)?,
        references: load_references(conn, id)?,
        sources: load_sources(conn, id)?,
        is_dynamic: row.get::<_, i32>(6)? != 0,
        date_added: row.get(7)?,
        image,
    })
}

const SELECT_CONTENT: &str =
    "SELECT id, slug, title, body_markdown, body_html, category, is_dynamic, date_added, image FROM content";

// ============================================================
// Queries
// ============================================================

pub fn get_content_by_slug(conn: &Connection, slug: &str) -> SqlResult<Option<ContentEntry>> {
    let mut stmt = conn.prepare(&format!("{SELECT_CONTENT} WHERE slug = ?1"))?;
    let mut rows = stmt.query_map(rusqlite::params![slug], |row| hydrate_row(conn, row))?;
    match rows.next() {
        Some(Ok(entry)) => Ok(Some(entry)),
        _ => Ok(None),
    }
}

pub fn get_content_paginated(
    conn: &Connection,
    page: u64,
    per_page: u64,
) -> SqlResult<Vec<ContentEntry>> {
    let offset = (page.saturating_sub(1)) * per_page;
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} ORDER BY date_added DESC, id DESC LIMIT ?1 OFFSET ?2"
    ))?;
    stmt.query_map(rusqlite::params![per_page, offset], |row| hydrate_row(conn, row))?
        .collect()
}

pub fn count_all_content(conn: &Connection) -> SqlResult<u64> {
    conn.query_row("SELECT COUNT(*) FROM content", [], |row| row.get(0))
}

pub fn get_all_content(conn: &Connection) -> SqlResult<Vec<ContentEntry>> {
    let mut stmt =
        conn.prepare(&format!("{SELECT_CONTENT} ORDER BY date_added DESC, id DESC"))?;
    stmt.query_map([], |row| hydrate_row(conn, row))?
        .collect()
}

pub fn search_content(conn: &Connection, query: &str) -> SqlResult<Vec<ContentEntry>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} WHERE title LIKE ?1 OR body_markdown LIKE ?1 OR slug IN (
            SELECT content_id FROM content_tags WHERE tag LIKE ?1
        ) ORDER BY date_added DESC, id DESC"
    ))?;
    stmt.query_map(rusqlite::params![pattern], |row| hydrate_row(conn, row))?
        .collect()
}

// ============================================================
// Dynamic pages (unused but kept)
// ============================================================

#[allow(dead_code)]
pub fn insert_dynamic_page(conn: &Connection, page: &DynamicPage) -> SqlResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO dynamic_pages (path, title, content, keywords)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![page.path, page.title, page.content, page.keywords.join(",")],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_dynamic_page(conn: &Connection, path: &str) -> SqlResult<Option<DynamicPage>> {
    let mut stmt = conn.prepare(
        "SELECT path, title, content, keywords FROM dynamic_pages WHERE path = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![path], |row| {
        let kw_str: String = row.get(3)?;
        Ok(DynamicPage {
            path: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            keywords: kw_str.split(',').map(|s| s.to_string()).collect(),
        })
    })?;
    match rows.next() {
        Some(Ok(page)) => Ok(Some(page)),
        _ => Ok(None),
    }
}