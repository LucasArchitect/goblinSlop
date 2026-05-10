use crate::db::{init_db, insert_content, ContentEntry, SourceRef};
use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Unified content entry — loaded from individual JSON files in data/content/
/// Schema: { id, title, slug, body_markdown, category, tags, references, sources, is_dynamic, date_added }
/// All list fields (tags, references, sources) are JSON arrays.
#[derive(Debug, Deserialize)]
pub struct JsonContentEntry {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    #[serde(default = "default_category")]
    pub category: String,
    /// Array of tags, e.g. ["goblin", "lore"]. Defaults to ["goblin"] if empty.
    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
    /// Array of target slugs this article explicitly references
    #[serde(default)]
    pub references: Vec<String>,
    /// Array of external sources [{name, url}]
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    #[serde(default)]
    pub is_dynamic: bool,
    #[serde(default = "default_date_added")]
    pub date_added: String,
}

fn default_category() -> String {
    "general".to_string()
}

fn default_tags() -> Vec<String> {
    vec!["goblin".to_string()]
}

fn default_date_added() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

/// Loads all individual JSON content files from `data/content/` into the database.
pub fn load_all_content(
    db_path: &str,
    content_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db(db_path)?;
    let content_path = Path::new(content_dir);

    if !content_path.exists() {
        return Err(format!("Content directory not found: {}", content_dir).into());
    }

    let mut entries: Vec<fs::DirEntry> = Vec::new();
    for entry in fs::read_dir(content_path)? {
        let entry = entry?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
            entries.push(entry);
        }
    }

    // Sort for deterministic load order
    entries.sort_by_key(|e| e.file_name());

    println!(
        "Loading {} content units from: {:?}",
        entries.len(),
        content_path
    );

    for entry in &entries {
        let path = entry.path();
        let json_content = fs::read_to_string(&path)?;

        match serde_json::from_str::<JsonContentEntry>(&json_content) {
            Ok(json_entry) => {
                let body_html = markdown_to_html(&json_entry.body_markdown);
                let ref_count = json_entry.references.len();

                let content_entry = ContentEntry {
                    id: 0,
                    slug: json_entry.slug.clone(),
                    title: json_entry.title.clone(),
                    body_markdown: json_entry.body_markdown,
                    body_html,
                    category: json_entry.category.clone(),
                    tags: json_entry.tags,
                    references: json_entry.references,
                    sources: json_entry.sources,
                    is_dynamic: json_entry.is_dynamic,
                    date_added: json_entry.date_added.clone(),
                };

                match insert_content(&conn, &content_entry) {
                    Ok(_) => println!(
                        "  ✅ Loaded content: {} (slug: {}, date_added: {}, refs: {} entries)",
                        content_entry.title,
                        content_entry.slug,
                        content_entry.date_added,
                        ref_count
                    ),
                    Err(e) => eprintln!("  ❌ Failed to load '{}': {}", json_entry.id, e),
                }
            }
            Err(e) => {
                eprintln!("  ⚠️  Skipped invalid JSON {}: {}", path.display(), e);
            }
        }
    }

    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_single_content_unit() {
        let test_file = std::path::PathBuf::from("data/content/goblin_lore.json");
        assert!(test_file.exists(), "Test file data/content/goblin_lore.json must exist");

        let json_str = fs::read_to_string(&test_file).expect("Failed to read test file");
        let entry: JsonContentEntry = serde_json::from_str(&json_str)
            .expect("Should deserialize goblin_lore.json into JsonContentEntry");

        assert!(!entry.id.is_empty(), "id must not be empty");
        assert_eq!(entry.slug, "goblin_lore", "slug should match filename stem");
        assert!(entry.title.starts_with("Goblin Lore"), "title should contain expected text");
        assert!(
            entry.body_markdown.starts_with("# Goblin Lore"),
            "body_markdown should start with heading"
        );
        assert_eq!(entry.category, "lore");
        assert_eq!(entry.is_dynamic, false);
        assert!(!entry.date_added.is_empty());
        assert_eq!(entry.date_added.len(), 20);
        assert!(entry.date_added.ends_with('Z'));
        assert!(!entry.references.is_empty(), "references must have entries");

        assert!(!entry.tags.is_empty(), "tags must not be empty");
        assert!(
            entry.tags.iter().any(|t| t == "goblin" || t == "lore"),
            "tags should contain goblin or lore"
        );

        println!(
            "✅ test_deserialize_single_content_unit passed: {} (slug: {})",
            entry.title, entry.slug
        );
    }

    #[test]
    fn test_load_and_read_entry_with_all_fields() {
        use crate::db::get_content_by_slug;
        use rusqlite::Connection;

        let db_path = "test_load_entry.db";
        let test_slug = "goblin-slayer-anime";

        let _ = std::fs::remove_file(db_path);
        load_all_content(db_path, "data/content").expect("load_all_content should succeed");
        let conn = Connection::open(db_path).expect("open db should succeed");

        let entry = get_content_by_slug(&conn, test_slug)
            .expect("query should succeed")
            .expect("entry must exist");

        // Core fields
        assert_eq!(entry.slug, test_slug);
        assert!(!entry.title.is_empty());
        assert!(!entry.body_markdown.is_empty());
        assert!(!entry.body_html.is_empty());
        assert!(!entry.category.is_empty());

        // Tags (normalized into dedicated table)
        assert!(!entry.tags.is_empty());
        assert!(entry.tags.contains(&"goblin".to_string()));

        // References (normalized into dedicated table)
        assert!(!entry.references.is_empty());
        for r in &entry.references {
            assert!(!r.is_empty(), "each reference slug must be non-empty");
        }

        // Sources (normalized into dedicated table)
        assert!(!entry.sources.is_empty());
        for s in &entry.sources {
            assert!(!s.name.is_empty(), "source name must be non-empty");
            assert!(s.url.starts_with("http"), "source url must start with http");
        }

        assert!(!entry.is_dynamic);
        assert!(!entry.date_added.is_empty());
        assert_eq!(entry.date_added.len(), 20);
        assert!(entry.date_added.ends_with('Z'));

        println!(
            "✅ test_load_and_read_entry_with_all_fields passed for {test_slug}"
        );

        let _ = std::fs::remove_file(db_path);
    }
}