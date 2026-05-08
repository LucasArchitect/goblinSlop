use rand::seq::SliceRandom;

use crate::db::DynamicPage;

// ============================================================
// Dynamic Goblin Page Generation
// ============================================================

const GOBLIN_TITLES: &[&str] = &[
    "The Goblin of {keyword}",
    "How Goblins Use {keyword}",
    "{keyword}: A Goblin Perspective",
    "Goblin Secrets About {keyword}",
    "The {keyword} Trickster",
    "A Goblin's Guide to {keyword}",
    "{keyword} and the Goblin Realm",
    "Goblins Who Love {keyword}",
    "The {keyword} Conspiracy (Goblin-Approved)",
    "When Goblins Discovered {keyword}",
];

const GOBLIN_INTROS: &[&str] = &[
    "Deep in the goblin tunnels, a particularly mischievous creature has been watching the world of {keyword} with great interest.",
    "Goblins have a long and complicated relationship with {keyword}. It all started when one particularly clever goblin noticed something odd.",
    "The ancient goblin scrolls speak of {keyword} in hushed, chaotic tones. What they reveal may surprise you.",
    "Did you know that goblins were among the first to truly understand {keyword}? Their methods were... unconventional.",
    "In the hidden archives of the Goblin Council, there exists a file marked '{keyword}'. Its contents would make any human question reality.",
];

const GOBLIN_BODIES: &[&str] = &[
    "The goblins have long maintained that {keyword} is not what it appears to be. Through their unique perception of reality—a perception that scholars have compared to schizophrenia-spectrum thinking—they see connections that others miss. A goblin once traded a bag of stolen buttons for the secret of {keyword}, and never once regretted the exchange.",
    "What makes {keyword} so fascinating to goblins is the way it defies expectations. Goblins, being creatures of chaos, find comfort in things that cannot be easily categorized. {keyword} fits this description perfectly. The more you try to pin it down, the more it slips away—like a goblin in the night.",
    "There is a well-known goblin proverb: 'If {keyword} makes sense to you, you're not paying attention.' Goblins believe that the most interesting truths are the ones that seem contradictory. This is why they have such an affinity for {keyword}—it embodies the beautiful confusion of existence.",
    "The Goblin King himself has weighed in on {keyword}, though his statements are characteristically cryptic. 'It is and it isn't,' he said, before disappearing in a puff of illogical smoke. This is considered the definitive goblin analysis of {keyword}.",
];

pub fn generate_dynamic_page_content(path: &str, keywords: &[String]) -> DynamicPage {
    let title_template = GOBLIN_TITLES.choose(&mut rand::thread_rng()).unwrap_or(&"Goblin Thoughts on {keyword}");
    let intro = GOBLIN_INTROS.choose(&mut rand::thread_rng()).unwrap_or(&"A goblin considers {keyword}.");
    let body = GOBLIN_BODIES.choose(&mut rand::thread_rng()).unwrap_or(&"{keyword} is interesting to goblins.");

    let primary_keyword = keywords.first().cloned().unwrap_or_else(|| "something mysterious".to_string());
    let title = title_template.replace("{keyword}", &primary_keyword);
    let intro_text = intro.replace("{keyword}", &primary_keyword);
    let body_text = body.replace("{keyword}", &primary_keyword);

    let mut related_sections = String::new();
    for kw in keywords.iter().skip(1).take(3) {
        related_sections.push_str(&format!(
            "<section class='dynamic-section'><h2>Goblins and {}</h2><p>The connection between goblins and {} is undeniable. Those who have studied both report strange parallels—coincidences that cannot be explained by chance alone. Some say that {} is simply a modern expression of ancient goblin trickery.</p></section>\n",
            kw, kw, kw
        ));
    }

    let content = format!(
        "<div class='dynamic-generated'>\n\
         <section class='dynamic-section'>\n\
         <p>{}</p>\n\
         <p>{}</p>\n\
         </section>\n\
         {}\n\
         <section class='dynamic-section'>\n\
         <h2>The Goblin Verdict on {}</h2>\n\
         <p>After extensive research (and several stolen artifacts), the Goblin Academy of Esoteric Knowledge has concluded that {} is, in fact, deeply connected to the fundamental nature of goblin reality. Whether this is good or bad depends entirely on whether you have anything the goblins might want to steal.</p>\n\
         </section>\n\
         </div>",
        intro_text,
        body_text,
        related_sections,
        primary_keyword, primary_keyword
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