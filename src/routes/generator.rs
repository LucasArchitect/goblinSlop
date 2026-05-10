use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::hash::{Hash, Hasher};

use crate::db::DynamicPage;
use super::content_templates::{GOBLIN_TITLES, GOBLIN_INTROS, GOBLIN_BODIES, VERDICT_TEMPLATES, generate_related_section};
use super::references::{generate_references_html_ex};

// ============================================================
// Generator — Assembles dynamic page from templates + refs
//   Deterministic: uses a seed derived from the URL path
// ============================================================

/// Derive a deterministic RNG seed from a string (the URL path).
fn seed_from_string(s: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Generate dynamic page content with a given RNG source.
/// The result is deterministic for the same seed (path).
pub fn generate_dynamic_page_content_with_rng<R: Rng>(path: &str, keywords: &[String], rng: &mut R) -> DynamicPage {
    let title_template = GOBLIN_TITLES.choose(rng).unwrap_or(&"Goblin Thoughts on {keyword}");
    let intro = GOBLIN_INTROS.choose(rng).unwrap_or(&"A goblin considers {keyword}.");
    let body = GOBLIN_BODIES.choose(rng).unwrap_or(&"{keyword} is interesting to goblins.");

    let primary_keyword = keywords.first().cloned().unwrap_or_else(|| "something mysterious".to_string());
    let title = title_template.replace("{keyword}", &primary_keyword);
    let intro_text = intro.replace("{keyword}", &primary_keyword);
    let body_text = body.replace("{keyword}", &primary_keyword);

    // Generate related sections with variety
    let mut related_sections = String::new();
    for kw in keywords.iter().skip(1).take(3) {
        related_sections.push_str(&generate_related_section(kw, rng));
    }

    // Generate cross-references
    let references_html = generate_references_html_ex(keywords, None, &[], rng);

    // Goblin Verdict
    let verdict = VERDICT_TEMPLATES.choose(rng).unwrap_or(&VERDICT_TEMPLATES[0]);
    let verdict_text = verdict.replace("{}", &primary_keyword);

    let content = format!(
        "<div class='dynamic-generated'>\n\
         <section class='dynamic-section'>\n\
         <p>{}</p>\n\
         <p>{}</p>\n\
         </section>\n\
         {}\n\
         <section class='dynamic-section'>\n\
         <h2>The Goblin Verdict on {}</h2>\n\
         <p>{}</p>\n\
         </section>\n\
         {}\n\
         </div>",
        intro_text,
        body_text,
        related_sections,
        primary_keyword,
        verdict_text,
        references_html,
    );

    DynamicPage {
        path: path.to_string(),
        title,
        content,
        keywords: keywords.to_vec(),
    }
}

/// Generate dynamic page content, deterministic from the URL path.
/// Uses a seeded RNG (derived from the path) — no randomness.
pub fn generate_dynamic_page_content(path: &str, keywords: &[String]) -> DynamicPage {
    let seed = seed_from_string(path);
    let mut rng = StdRng::seed_from_u64(seed);
    generate_dynamic_page_content_with_rng(path, keywords, &mut rng)
}

pub fn parse_path_into_keywords(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.split('-'))
        .flat_map(|s| s.split('_'))
        .map(|s| s.to_lowercase())
        .filter(|s| s.len() > 2 && !is_stop_word(s))
        .collect()
}

fn is_stop_word(word: &str) -> bool {
    matches!(
        word,
        "the" | "a" | "an" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "by" | "with" | "is" | "are" | "was" | "were" | "be" | "been"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_content_is_deterministic() {
        // Generate twice with the same path — must produce identical output
        let path = "mysterious-goblin-tunnel";
        let keywords = vec![
            "mysterious".to_string(),
            "goblin".to_string(),
            "tunnel".to_string(),
        ];

        let first = generate_dynamic_page_content(path, &keywords);
        let second = generate_dynamic_page_content(path, &keywords);

        assert_eq!(first.title, second.title,
            "Titles must be identical for the same path");
        assert_eq!(first.content, second.content,
            "Content must be identical for the same path");
        assert_eq!(first.path, second.path,
            "Paths must be identical");
        assert_eq!(first.keywords, second.keywords,
            "Keywords must be identical");

        println!("✅ test_generated_content_is_deterministic passed: same path => same output");
    }

    #[test]
    fn test_different_paths_produce_different_content() {
        let keywords_a = vec!["goblin".to_string(), "lore".to_string()];
        let keywords_b = vec!["altman".to_string(), "miku".to_string()];

        let page_a = generate_dynamic_page_content("goblin-lore-test", &keywords_a);
        let page_b = generate_dynamic_page_content("altman-miku-test", &keywords_b);

        // Different paths must produce different content
        assert_ne!(page_a.content, page_b.content,
            "Different paths must produce different content");
        assert_ne!(page_a.title, page_b.title,
            "Different paths must produce different titles");

        println!("✅ test_different_paths_produce_different_content passed: different paths => different output");
    }
}
