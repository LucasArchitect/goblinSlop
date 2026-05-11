use rand::Rng;
use crate::db::{ContentEntry, DynamicPage};
use super::references::generate_references_html_thread_rng;

/// Scans static/images/ directory at runtime and returns all jpg filenames (excludes default.jpg)
pub fn get_image_pool() -> Vec<String> {
    let mut images: Vec<String> = std::fs::read_dir("static/images")
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name.ends_with(".jpg") && !name.starts_with("default"))
        .collect();
    images.sort();
    images
}

/// Render tags as clickable HTML links
pub fn render_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|t| format!("<a href='/tag/{}' class='tag-link'>{}</a>", t, t))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render a category as a clickable HTML link
pub fn render_category(category: &str) -> String {
    format!(r#"<a href='/category/{}' class='category-link'>{}</a>"#, category, category)
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
    <link rel="icon" href="/static/favicon.ico" type="image/x-icon">
    <link rel="apple-touch-icon" href="/static/images/default.jpg">
    <meta name="description" content="{DESCRIPTION}">
    <meta name="robots" content="{ROBOTS}">
    <meta name="keywords" content="{KEYWORDS}">
    <meta name="author" content="GoblinSlop Editorial Collective">
    <link rel="canonical" href="{CANONICAL}">
    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="{OG_TYPE}">
    <meta property="og:url" content="{CANONICAL}">
    <meta property="og:title" content="{OG_TITLE}">
    <meta property="og:description" content="{OG_DESC}">
    <meta property="og:image" content="{OG_IMAGE}">
    <meta property="og:site_name" content="GoblinSlop">
    <meta property="og:locale" content="en_US">
    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:url" content="{CANONICAL}">
    <meta name="twitter:title" content="{OG_TITLE}">
    <meta name="twitter:description" content="{OG_DESC}">
    <meta name="twitter:image" content="{OG_IMAGE}">
    <!-- RSS Feed -->
    <link rel="alternate" type="application/rss+xml" title="GoblinSlop — Goblin Lore & Slop" href="/feed.xml">
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "{SCHEMA_TYPE}",
        "name": "{SCHEMA_NAME}",
        "description": "{SCHEMA_DESC}",
        "url": "{CANONICAL}",
        "publisher": {
            "@type": "Organization",
            "name": "GoblinSlop",
            "logo": {
                "@type": "ImageObject",
                "url": "https://goblin.geno.su/static/images/default.jpg"
            }
        },
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

/// Escape HTML entities for safe embedding in JSON-LD
fn json_escape(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('"', r#"\""#)
        .replace('\n', r"\n")
        .replace('\r', "")
        .replace('\t', "\\t")
}

/// Build the HTML head block with all meta tags including OG/Twitter cards
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
    og_type: &str,
    og_title: &str,
    og_desc: &str,
    og_image: &str,
) -> String {
    let canonical = if canonical_path.starts_with("http") {
        canonical_path.to_string()
    } else {
        format!("{}{}", base_url.trim_end_matches('/'), canonical_path)
    };

    // JSON-LD requires HTML-escaped strings
    let esc_name = json_escape(schema_name);
    let esc_desc = json_escape(schema_desc);

   BASE_HTML_HEAD
        .replace("{TITLE}", title)
        .replace("{DESCRIPTION}", description)
        .replace("{ROBOTS}", robots)
        .replace("{CANONICAL}", &canonical)
        .replace("{SCHEMA_TYPE}", schema_type)
        .replace("{SCHEMA_NAME}", &esc_name)
        .replace("{SCHEMA_DESC}", &esc_desc)
        .replace("{KEYWORDS}", keywords)
        .replace("{OG_TYPE}", og_type)
        .replace("{OG_TITLE}", og_title)
        .replace("{OG_DESC}", og_desc)
        .replace("{OG_IMAGE}", og_image)
}

/// Render a standard content page with JSON-LD metadata
pub fn render_content_page(entry: &ContentEntry, canonical_path: &str, base_url: &str) -> String {
    let mut html = String::new();
    let tags_str = entry.tags.join(", ");
    let img_file = entry.image.as_deref().unwrap_or("default.jpg");
    let og_image = format!("{}{}", base_url.trim_end_matches('/'), format!("/static/images/{}", img_file));
    let og_title = &entry.title;
    let og_desc = &format!("Goblin content: {}", entry.title);

    let head = build_head(
        &format!("{} - GoblinSlop", entry.title),
        og_desc,
        canonical_path,
        base_url,
        "index, follow",
        "Article",
        &entry.title,
        og_desc,
        &tags_str,
        "Article",           // og:type
        og_title,            // og:title
        og_desc,             // og:description
        &og_image,           // og:image
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
    refs_keywords.extend(entry.slug.split('-').map(|s| s.to_string()));
    let explicit_slugs = entry.references.clone();
    html.push_str(&generate_references_html_thread_rng(&refs_keywords, Some(&entry.slug), &explicit_slugs));

    // Sources section (external references like IMDb, MyAnimeList, etc.)
    if !entry.sources.is_empty() {
        html.push_str("<section class='sources-section'><h2>Sources</h2><ul class='sources-list'>");
        for src in &entry.sources {
            if src.url.is_empty() {
                html.push_str(&format!("<li>{}</li>", src.name));
            } else {
                html.push_str(&format!(
                    "<li><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></li>",
                    src.url, src.name
                ));
            }
        }
        html.push_str("</ul></section>");
    }

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a dynamically generated goblin page — picks random image from pool per request
pub fn render_dynamic_page<R: Rng>(
    dyn_page: &DynamicPage,
    canonical_path: &str,
    base_url: &str,
    rng: &mut R,
) -> String {
    let keywords_str = dyn_page.keywords.join(", ");
    let mut html = String::new();

    // Pick random image first (for OG meta)
    let pool = get_image_pool();
    let img_idx = rng.gen_range(0..pool.len());
    let selected_img = &pool[img_idx];
    let og_image = format!("{}{}", base_url.trim_end_matches('/'), &format!("/static/images/{}", selected_img));

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
        "WebPage",         // og:type
        &dyn_page.title,   // og:title
        &format!("Goblin content about: {}", keywords_str), // og:description
        &og_image,         // og:image
    );
    html.push_str(&head);

    // Pick random image — dynamically loaded from static/images/ (no hardcoded list)
    let pool = get_image_pool();
    let img_idx = rng.gen_range(0..pool.len());
    let selected_img = &pool[img_idx];

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
    </header>
    <div class="page-body">
        <div class="article-image">
            <img src="/static/images/{}" alt="{title}" class="article-img">
        </div>
        {content}
    </div>
</article>"#,
        selected_img,
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

    // Default OG image for collection pages
    let og_image = format!("{}{}", base_url.trim_end_matches('/'), "/static/images/default.jpg");
    let og_desc = if title.len() > 150 { &title[..150] } else { title };

    let head = build_head(
        &format!("{} - GoblinSlop", title),
        title,
        canonical_path,
        base_url,
        "index, follow",
        "CollectionPage",
        title,
        og_desc,
        tags,
        "WebPage",     // og:type
        &format!("{} - GoblinSlop", title), // og:title
        og_desc,       // og:description
        &og_image,     // og:image
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
