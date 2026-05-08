use crate::db::{ContentEntry, DynamicPage};

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
    <meta name="robots" content="index, follow">
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
                <a href="/all">All Pages</a>
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

/// Render a standard content page with JSON-LD metadata
pub fn render_content_page(entry: &ContentEntry, canonical_url: &str) -> String {
    let mut html = String::new();

    // Head
    let head = BASE_HTML_HEAD
        .replace("{TITLE}", &format!("{} - GoblinSlop", entry.title))
        .replace("{DESCRIPTION}", &format!("Goblin content: {}", entry.title))
        .replace("{CANONICAL}", canonical_url)
        .replace("{SCHEMA_TYPE}", "Article")
        .replace("{SCHEMA_NAME}", &entry.title)
        .replace("{SCHEMA_DESC}", &format!("Goblin content: {}", entry.title))
        .replace("{KEYWORDS}", &entry.tags);
    html.push_str(&head);

    // Body
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
        {body}
    </div>
</article>"#,
        title = entry.title,
        category = entry.category,
        tags = entry.tags,
        body = entry.body_html,
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a dynamically generated goblin page (secretly)
pub fn render_dynamic_page(dyn_page: &DynamicPage, _canonical_url: &str) -> String {
    let keywords_str = dyn_page.keywords.join(", ");
    let mut html = String::new();

    let head = BASE_HTML_HEAD
        .replace("{TITLE}", &format!("{} - GoblinSlop", dyn_page.title))
        .replace("{DESCRIPTION}", &format!("Goblin content about: {}", keywords_str))
        .replace("{CANONICAL}", &format!("/{}", dyn_page.path))
        .replace("{SCHEMA_TYPE}", "WebPage")
        .replace("{SCHEMA_NAME}", &dyn_page.title)
        .replace("{SCHEMA_DESC}", &format!("Goblin content related to: {}", keywords_str))
        .replace("{KEYWORDS}", &keywords_str);
    html.push_str(&head);

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
    </header>
    <div class="page-body">
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
pub fn render_static_page(title: &str, body_html: &str, category: &str, tags: &str, canonical_url: &str) -> String {
    let mut html = String::new();

    let head = BASE_HTML_HEAD
        .replace("{TITLE}", &format!("{} - GoblinSlop", title))
        .replace("{DESCRIPTION}", title)
        .replace("{CANONICAL}", canonical_url)
        .replace("{SCHEMA_TYPE}", "CollectionPage")
        .replace("{SCHEMA_NAME}", title)
        .replace("{SCHEMA_DESC}", title)
        .replace("{KEYWORDS}", tags);
    html.push_str(&head);

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
        {body}
    </div>
</article>"#,
        title = title,
        category = category,
        tags = tags,
        body = body_html
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}