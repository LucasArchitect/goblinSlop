use crate::db::{init_db, insert_content, ContentEntry};
use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Unified content entry — loaded from individual JSON files in data/content/
/// Schema: { id, title, slug, body_markdown, category, tags, references, is_dynamic, date_added }
#[derive(Debug, Deserialize)]
pub struct JsonContentEntry {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default)]
    pub tags: String,
    /// Array of target slugs this article explicitly references
    #[serde(default)]
    pub references: Vec<String>,
    #[serde(default)]
    pub is_dynamic: bool,
    #[serde(default = "default_date_added")]
    pub date_added: String,
}

fn default_category() -> String {
    "general".to_string()
}

fn default_date_added() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

/// Loads all individual JSON content files from `data/content/` into the database.
/// Each .json file is treated as one content unit with unified schema:
/// { id, title, slug, body_markdown, category, tags, references, is_dynamic, date_added }
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
                // Convert markdown → HTML
                let body_html = markdown_to_html(&json_entry.body_markdown);

                let tags = if json_entry.tags.is_empty() {
                    "goblin".to_string()
                } else {
                    json_entry.tags.clone()
                };

                // Join references array into comma-separated string for DB storage
                let references_str = json_entry.references.join(",");

                let content_entry = ContentEntry {
                    id: 0,
                    slug: json_entry.slug.clone(),
                    title: json_entry.title.clone(),
                    body_markdown: json_entry.body_markdown,
                    body_html,
                    category: json_entry.category.clone(),
                    tags,
                    references: references_str,
                    is_dynamic: json_entry.is_dynamic,
                };

                match insert_content(&conn, &content_entry) {
                    Ok(_) => println!(
                        "  ✅ Loaded content: {} (slug: {}, date_added: {}, refs: {} entries)",
                        json_entry.title,
                        json_entry.slug,
                        json_entry.date_added,
                        json_entry.references.len()
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
    use std::fs;

    #[test]
    fn test_deserialize_single_content_unit() {
        // Load one actual JSON file from data/content/ and verify deserialization
        let test_file = std::path::PathBuf::from("data/content/goblin_lore.json");
        assert!(test_file.exists(), "Test file data/content/goblin_lore.json must exist");

        let json_str = fs::read_to_string(&test_file).expect("Failed to read test file");
        let entry: JsonContentEntry = serde_json::from_str(&json_str)
            .expect("Should deserialize goblin_lore.json into JsonContentEntry");

        // Verify all fields populated correctly
        assert!(!entry.id.is_empty(), "id must not be empty");
        assert_eq!(entry.slug, "goblin_lore", "slug should match filename stem");
        assert!(entry.title.contains("Goblin Lore"), "title should contain expected text");
        assert!(
            entry.body_markdown.starts_with("# Goblin Lore"),
            "body_markdown should start with heading"
        );
        assert_eq!(entry.category, "lore", "category inferred from filename stem");
        assert!(
            entry.tags.contains("goblin") || entry.tags.is_empty() == false,
            "tags must not be empty (default: goblin)"
        );
        assert!(!entry.date_added.is_empty(), "date_added must be present");
        assert_eq!(entry.is_dynamic, false, "static content must not be dynamic");

        // Verify date_added format (ISO 8601)
        assert!(
            entry.date_added.len() == 20 && entry.date_added.ends_with('Z'),
            "date_added should be ISO 8601 UTC format"
        );

        println!("✅ test_deserialize_single_content_unit passed: {} (slug: {})", entry.title, entry.slug);
    }
}