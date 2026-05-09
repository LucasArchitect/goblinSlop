use rand::seq::SliceRandom;

use crate::db::DynamicPage;
use super::content_templates::{GOBLIN_TITLES, GOBLIN_INTROS, GOBLIN_BODIES, VERDICT_TEMPLATES, generate_related_section};
use super::references::generate_references_html;

// ============================================================
// Generator — Assembles dynamic page from templates + refs
// ============================================================

pub fn generate_dynamic_page_content(path: &str, keywords: &[String]) -> DynamicPage {
    let title_template = GOBLIN_TITLES.choose(&mut rand::thread_rng()).unwrap_or(&"Goblin Thoughts on {keyword}");
    let intro = GOBLIN_INTROS.choose(&mut rand::thread_rng()).unwrap_or(&"A goblin considers {keyword}.");
    let body = GOBLIN_BODIES.choose(&mut rand::thread_rng()).unwrap_or(&"{keyword} is interesting to goblins.");

    let primary_keyword = keywords.first().cloned().unwrap_or_else(|| "something mysterious".to_string());
    let title = title_template.replace("{keyword}", &primary_keyword);
    let intro_text = intro.replace("{keyword}", &primary_keyword);
    let body_text = body.replace("{keyword}", &primary_keyword);

    // Generate related sections with variety
    let mut related_sections = String::new();
    for kw in keywords.iter().skip(1).take(3) {
        related_sections.push_str(&generate_related_section(kw));
    }

    // Generate cross-references
    let references_html = generate_references_html(keywords);

    // Goblin Verdict
    let verdict = VERDICT_TEMPLATES.choose(&mut rand::thread_rng()).unwrap_or(&VERDICT_TEMPLATES[0]);
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
