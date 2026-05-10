use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentEntry {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body_markdown: String,
    pub body_html: String,
    pub category: String,
    pub tags: String, // comma-separated
    /// Comma-separated list of target slugs this article explicitly references (from JSON)
    pub references: String,
    pub is_dynamic: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicPage {
    pub path: String,
    pub title: String,
    pub content: String,
    pub keywords: Vec<String>,
}

pub fn init_db(path: &str) -> SqlResult<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS content (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            body_markdown TEXT NOT NULL,
            body_html TEXT NOT NULL,
            category TEXT NOT NULL DEFAULT 'general',
            tags TEXT NOT NULL DEFAULT '',
            \"references\" TEXT NOT NULL DEFAULT '',
            is_dynamic INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS dynamic_pages (
            path TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            keywords TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE IF NOT EXISTS keywords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            keyword TEXT NOT NULL UNIQUE,
            content_id INTEGER,
            FOREIGN KEY (content_id) REFERENCES content(id)
        );",
    )?;

    Ok(conn)
}

pub fn insert_content(conn: &Connection, entry: &ContentEntry) -> SqlResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO content (slug, title, body_markdown, body_html, category, tags, \"references\", is_dynamic)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            entry.slug,
            entry.title,
            entry.body_markdown,
            entry.body_html,
            entry.category,
            entry.tags,
            entry.references,
            entry.is_dynamic as i32,
        ],
    )?;
    Ok(())
}

pub fn get_content_by_slug(conn: &Connection, slug: &str) -> SqlResult<Option<ContentEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, slug, title, body_markdown, body_html, category, tags, \"references\", is_dynamic
         FROM content WHERE slug = ?1",
    )?;

    let mut rows = stmt.query_map(rusqlite::params![slug], |row| {
        Ok(ContentEntry {
            id: row.get(0)?,
            slug: row.get(1)?,
            title: row.get(2)?,
            body_markdown: row.get(3)?,
            body_html: row.get(4)?,
            category: row.get(5)?,
            tags: row.get(6)?,
            references: row.get(7)?,
            is_dynamic: row.get::<_, i32>(8)? != 0,
        })
    })?;

    match rows.next() {
        Some(Ok(entry)) => Ok(Some(entry)),
        _ => Ok(None),
    }
}

pub fn get_all_content(conn: &Connection) -> SqlResult<Vec<ContentEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, slug, title, body_markdown, body_html, category, tags, \"references\", is_dynamic
         FROM content ORDER BY id",
    )?;

    let entries = stmt
        .query_map([], |row| {
            Ok(ContentEntry {
                id: row.get(0)?,
                slug: row.get(1)?,
                title: row.get(2)?,
                body_markdown: row.get(3)?,
                body_html: row.get(4)?,
                category: row.get(5)?,
                tags: row.get(6)?,
                references: row.get(7)?,
                is_dynamic: row.get::<_, i32>(8)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}

pub fn search_content(conn: &Connection, query: &str) -> SqlResult<Vec<ContentEntry>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, slug, title, body_markdown, body_html, category, tags, \"references\", is_dynamic
         FROM content
         WHERE title LIKE ?1 OR body_markdown LIKE ?1 OR tags LIKE ?1
         ORDER BY id",
    )?;

    let entries = stmt
        .query_map(rusqlite::params![pattern], |row| {
            Ok(ContentEntry {
                id: row.get(0)?,
                slug: row.get(1)?,
                title: row.get(2)?,
                body_markdown: row.get(3)?,
                body_html: row.get(4)?,
                category: row.get(5)?,
                tags: row.get(6)?,
                references: row.get(7)?,
                is_dynamic: row.get::<_, i32>(8)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}

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