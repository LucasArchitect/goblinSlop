use crate::db::{ContentEntry, DynamicPage};
use super::references::generate_references_html_thread_rng;

/// Render tags as clickable HTML links
pub fn render_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|t| format!("<a href='/tag/{}' class='tag-link'>{}</a>", t, t))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render a category as a clickable HTML link
pub fn render_category(category: &str) -> String {
    format!("<a href='/category/{}' class='category-link'>{}</a>", category, category)
}

// ============================================================
// HTML Template Constants
// ============================================================

const BASE_HTML_HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{TITLE}</title>
    <link rel="stylesheet" href="/static/styles.css">
    <meta name="description" content="{DESCRIPTION}">
    <meta name="robots" content="{ROBOTS}">
    <link rel="canonical" href="{CANONICAL}">
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "{SCHEMA_TYPE}",
        "name": "{SCHEMA_NAME}",
        "description": "{SCHEMA_DESC}",
        "url": "{CANONICAL}",
        "about": {
            "@type": "Thing",
            "name": "Goblins",
            "description": "Goblin folklore, mythology, tricks, and cultural references including the Sam Altman connection"
        },
        "keywords": "{KEYWORDS}"
    }
    </script>
</head>
<body>
    <nav class="goblin-nav">
        <div class="nav-inner">
            <a href="/" class="nav-logo">🧌 GoblinSlop</a>
            <div class="nav-links">
                <a href="/">Home</a>
                <a href="/goblin-lore">Lore</a>
                <a href="/goblin-tricks">Tricks</a>
                <a href="/sam-altman-goblins">Sam Altman</a>
                <a href="/goblin-schizophrenia">Schizophrenia</a>
                <a href="/search">Search</a>
            </div>
        </div>
    </nav>
    <main class="content-wrapper">
"#;

const BASE_HTML_FOOT: &str = r#"    </main>
    <footer class="goblin-footer">
        <p>🧌 GoblinSlop — A chaotic collection of goblin knowledge</p>
    </footer>
</body>
</html>"#;

fn build_head(
    title: &str,
    description: &str,
    canonical_path: &str,
    base_url: &str,
    robots: &str,
    schema_type: &str,
    schema_name: &str,
    schema_desc: &str,
    keywords: &str,
) -> String {
    let canonical = if canonical_path.starts_with("http") {
        canonical_path.to_string()
    } else {
        format!("{}{}", base_url.trim_end_matches('/'), canonical_path)
    };

    BASE_HTML_HEAD
        .replace("{TITLE}", title)
        .replace("{DESCRIPTION}", description)
        .replace("{ROBOTS}", robots)
        .replace("{CANONICAL}", &canonical)
        .replace("{SCHEMA_TYPE}", schema_type)
        .replace("{SCHEMA_NAME}", schema_name)
        .replace("{SCHEMA_DESC}", schema_desc)
        .replace("{KEYWORDS}", keywords)
}

/// Render a standard content page with JSON-LD metadata
pub fn render_content_page(entry: &ContentEntry, canonical_path: &str, base_url: &str) -> String {
    let mut html = String::new();
    let tags_str = entry.tags.join(", ");

    let head = build_head(
        &format!("{} - GoblinSlop", entry.title),
        &format!("Goblin content: {}", entry.title),
        canonical_path,
        base_url,
        "index, follow",
        "Article",
        &entry.title,
        &format!("Goblin content: {}", entry.title),
        &tags_str,
    );
    html.push_str(&head);

    // Body
    let img_file = entry.image.as_deref().unwrap_or("default.jpg");
    let image_html = format!(
        r#"<div class="article-image">
            <img src="/static/images/{}" alt="{}" class="article-img">
        </div>"#,
        img_file, entry.title
    );

    let cat_link = render_category(&entry.category);
    let tag_links = render_tags(&entry.tags);

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
        <div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
        </div>
    </header>
    <div class="page-body">
        {image}
        {body}
    </div>
</article>"#,
        title = entry.title,
        category = cat_link,
        tags = tag_links,
        image = image_html,
        body = entry.body_html,
    ));

    // Cross-references — explicit JSON refs + keyword-matched + random fake refs in one block
    let mut refs_keywords: Vec<String> = entry.tags.clone();
    refs_keywords.extend(
        entry.slug.split('-').map(|s| s.to_string())
    );
    let explicit_slugs = entry.references.clone();
    html.push_str(&generate_references_html_thread_rng(&refs_keywords, Some(&entry.slug), &explicit_slugs));

    // Sources section (external references like IMDb, MyAnimeList, etc.)
    if !entry.sources.is_empty() {
        html.push_str("<section class='sources-section'><h2>Sources</h2><ul class='sources-list'>");
        for src in &entry.sources {
            if src.url.is_empty() {
                html.push_str(&format!("<li>{}</li>", src.name));
            } else {
                html.push_str(&format!("<li><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></li>", src.url, src.name));
            }
        }
        html.push_str("</ul></section>");
    }

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a dynamically generated goblin page
pub fn render_dynamic_page(dyn_page: &DynamicPage, canonical_path: &str, base_url: &str) -> String {
    let keywords_str = dyn_page.keywords.join(", ");
    let mut html = String::new();

    let head = build_head(
        &format!("{} - GoblinSlop", dyn_page.title),
        &format!("Goblin content about: {}", keywords_str),
        canonical_path,
        base_url,
        "index, follow",
        "WebPage",
        &dyn_page.title,
        &format!("Goblin content related to: {}", keywords_str),
        &keywords_str,
    );
    html.push_str(&head);

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
    </header>
    <div class="page-body">
        <div class="article-image">
            <img src="/static/images/default.jpg" alt="{title}" class="article-img">
        </div>
        {content}
    </div>
</article>"#,
        title = dyn_page.title,
        content = dyn_page.content,
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a static page from raw HTML body (for home, search, all)
pub fn render_static_page(
    title: &str,
    body_html: &str,
    category: &str,
    tags: &str,
    canonical_path: &str,
    base_url: &str,
) -> String {
    let mut html = String::new();

    let head = build_head(
        &format!("{} - GoblinSlop", title),
        title,
        canonical_path,
        base_url,
        "index, follow",
        "CollectionPage",
        title,
        title,
        tags,
    );
    html.push_str(&head);

    // Only show category/tags meta on non-home pages
    let meta = if category == "home" {
        String::new()
    } else {
        format!(
            r#"<div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
        </div>"#,
            category = category,
            tags = tags,
        )
    };

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
        {meta}
    </header>
    <div class="page-body">
        {body}
    </div>
</article>"#,
        title = title,
        meta = meta,
        body = body_html
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}