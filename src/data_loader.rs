use crate::db::{init_db, insert_content, ContentEntry};
use pulldown_cmark::{Parser, html};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct ScrapedEntry {
    source: String,
    url: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    tags: String,
    #[serde(default)]
    slug: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct ScrapedContent {
    sources: Vec<ScrapedEntry>,
}

/// Loads scraped content from `data/scraped_content.json` into the database.
pub fn load_scraped_content(db_path: &str, data_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db(db_path)?;
    let json_path = Path::new(data_dir).join("scraped_content.json");

    if !json_path.exists() {
        println!("No scraped content file found at: {:?}", json_path);
        return Ok(());
    }

    let json_content = fs::read_to_string(&json_path)?;
    let scraped: ScrapedContent = serde_json::from_str(&json_content)?;

    println!("Loading {} scraped entries from: {:?}", scraped.sources.len(), json_path);

    for entry in &scraped.sources {
        let slug = if entry.slug.is_empty() {
            slugify(&entry.source)
        } else {
            entry.slug.clone()
        };

        let category = if entry.category.is_empty() {
            "scraped".to_string()
        } else {
            entry.category.clone()
        };

        let tags = if entry.tags.is_empty() {
            format!("goblin,scraped")
        } else {
            entry.tags.clone()
        };

        // Wrap data in a markdown document with source attribution
        let markdown = format!(
            "# {}\n\n> Source: [{}]({})\n\n{}",
            entry.source, entry.source, entry.url, entry.data
        );

        let body_html = markdown_to_html(&markdown);

        let content_entry = ContentEntry {
            id: 0,
            slug: slug.clone(),
            title: entry.source.clone(),
            body_markdown: markdown,
            body_html,
            category,
            tags,
            is_dynamic: false,
        };

        match insert_content(&conn, &content_entry) {
            Ok(_) => println!("  ✅ Loaded scraped: {} (slug: {})", entry.source, slug),
            Err(e) => eprintln!("  ❌ Failed to load '{}': {}", entry.source, e),
        }
    }

    Ok(())
}

fn slugify(source: &str) -> String {
    source
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new(md);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}