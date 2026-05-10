use rusqlite::{Connection, Result as SqlResult};

pub fn init_db(path: &str) -> SqlResult<Connection> {
    let conn = Connection::open(path)?;

    // Delete old DB so we always start fresh (no migrations needed)
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;

         DROP TABLE IF EXISTS content_tags;
         DROP TABLE IF EXISTS content_references;
         DROP TABLE IF EXISTS content_sources;
         DROP TABLE IF EXISTS dynamic_pages;
         DROP TABLE IF EXISTS content;
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
         );

         CREATE INDEX IF NOT EXISTS idx_content_category ON content(category);
         CREATE INDEX IF NOT EXISTS idx_content_tags_tag ON content_tags(tag);
         CREATE INDEX IF NOT EXISTS idx_content_tags_cid ON content_tags(content_id);",
    )?;

    Ok(conn)
}