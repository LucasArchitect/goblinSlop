use crate::db::{init_db, insert_content, ContentEntry};
use pulldown_cmark::{Parser, html};
use std::fs;
use std::path::Path;

/// Loads all markdown files from a content directory into the database
pub fn load_content_from_dir(db_path: &str, content_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db(db_path)?;
    let content_path = Path::new(content_dir);

    if !content_path.exists() {
        return Err(format!("Content directory not found: {}", content_dir).into());
    }

    for entry in fs::read_dir(content_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let markdown = fs::read_to_string(&path)?;
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            let html = markdown_to_html(&markdown);
            let title = extract_title(&markdown, &slug);

            let content_entry = ContentEntry {
                id: 0,
                slug: slug.clone(),
                title,
                body_markdown: markdown,
                body_html: html,
                category: infer_category(&slug),
                tags: infer_tags(&slug),
                is_dynamic: false,
            };

            insert_content(&conn, &content_entry)?;
            println!("Loaded content: {}", slug);
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

fn extract_title(markdown: &str, fallback: &str) -> String {
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return trimmed.trim_start_matches("# ").to_string();
        }
    }
    // Title-case the slug as fallback
    fallback
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn infer_category(slug: &str) -> String {
    if slug.contains("goblin") && slug.contains("trick") {
        "tricks".to_string()
    } else if slug.contains("goblin") && slug.contains("lore") {
        "lore".to_string()
    } else if slug.contains("altman") || slug.contains("sam") {
        "sam_altman".to_string()
    } else if slug.contains("schizo") {
        "schizophrenia".to_string()
    } else {
        "general".to_string()
    }
}

fn infer_tags(slug: &str) -> String {
    let mut tags = Vec::new();

    if slug.contains("goblin") {
        tags.push("goblin");
    }
    if slug.contains("trick") || slug.contains("prank") {
        tags.push("tricks");
    }
    if slug.contains("lore") || slug.contains("myth") {
        tags.push("lore");
    }
    if slug.contains("altman") || slug.contains("sam") {
        tags.push("sam-altman");
        tags.push("openai");
    }
    if slug.contains("schizo") {
        tags.push("schizophrenia");
        tags.push("psychology");
    }

    tags.join(",")
}