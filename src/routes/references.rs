// ============================================================
// References — Real page references + random fake ref generator
// ============================================================

use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

// ── Real page references ────────────────────────────────────
pub const REAL_PAGE_REFERENCES: &[(&str, &str, &str)] = &[
    // Static content pages
    ("goblin-lore", "Goblin Lore: The Ancient Tricksters", "lore"),
    ("goblin-tricks", "The Goblin's Book of Tricks", "tricks"),
    ("goblin-schizophrenia", "Goblins, Schizophrenia, and the Fractured Mind", "schizo"),
    ("sam-altman-goblins", "Sam Altman: CEO, Visionary, or Goblin King?", "altman"),
    ("altman-miku-goblin-king", "Sam Altman, Hatsune Miku, and the Goblin Throne", "altman"),
    ("ai-goblin-schizo-miku", "The Schizo-Goblin-Post-Truth-AI-Slop-Miku Continuum", "schizo"),
    ("miku-slop-ai-goblins", "The Miku-Altman Singularity: How a Goblin AI Learned to Sing", "altman"),
    ("slop-goblin-manifesto", "The Slop Manifesto: Goblin Content Theory", "slop"),
    // Scraped content pages
    ("goblin-slayer-anime", "MyAnimeList — Goblin Slayer", "anime"),
    ("goblin-slayer-goblins-crown", "MyAnimeList — Goblin Slayer: Goblin's Crown", "anime"),
    ("goblin-slayer-ii", "MyAnimeList — Goblin Slayer II", "anime"),
    ("goblin-is-very-strong", "MyAnimeList — Goblin Is Very Strong", "anime"),
    ("goblins-in-anime-overview", "MyAnimeList — Goblins in Anime & Manga Overview", "anime"),
    ("goblins-in-visual-novels", "VNDB — Goblin-related Visual Novels", "games"),
    ("labyrinth-goblin-king", "IMDb — Labyrinth: The Goblin King", "pop-culture"),
    ("goblins-harry-potter", "IMDb — Harry Potter Goblins", "pop-culture"),
    ("the-hobbit-goblins", "IMDb — The Hobbit Goblins & Orcs", "pop-culture"),
    ("willow-brownies-goblins", "IMDb — Willow: Brownies & Goblins", "pop-culture"),
    ("gremlins-goblin-comparison", "IMDb — Gremlins: Goblin-like Mayhem", "pop-culture"),
    ("green-goblin-hobgoblin", "IMDb — Spider-Man: Green Goblin & Hobgoblin", "pop-culture"),
    ("goblins-pop-culture-tropes", "TV Tropes — Goblins in Media", "pop-culture"),
    ("dungeons-and-dragons-goblins", "Dungeons & Dragons — Goblin Lore", "ttrpg"),
    ("warcraft-goblins", "Warcraft — Goblin Lore", "games"),
    ("warhammer-goblins", "Warhammer Fantasy — Goblin Lore", "games"),
    ("magic-the-gathering-goblins", "Magic: The Gathering — Goblins", "games"),
    ("goblin-utagoe-hikaru-genji", "Wonder — Goblin (J-Rock Band)", "anime"),
    ("pathfinder-goblins", "Pathfinder RPG — Goblins", "ttrpg"),
    ("discworld-goblins", "Discworld — Terry Pratchett's Goblins", "literature"),
    ("goblin-mode-oxford", "Goblin Mode — Oxford Word of the Year 2022", "linguistics"),
];

// ── Word lists for random fake page generation ─────────────
const FAKE_SLUG_PARTS_A: &[&str] = &[
    "goblin", "gpt", "miku", "vocaloid", "schizo", "altman", "slop", "content",
    "pattern", "hologram", "synthesized", "digital", "neural", "trickster",
    "hallucination", "delusion", "prophecy", "void", "ghost", "shadow",
    "crystal", "static", "fractal", "infinite", "deep", "hidden",
    "secret", "forbidden", "lost", "whisper", "silence",
    "protocol", "matrix", "threshold", "edge", "cave",
    "frequency", "transmission", "signal", "frequency", "echo",
    "void", "ritual", "tome", "grimoire", "manifesto",
];

const FAKE_SLUG_PARTS_B: &[&str] = &[
    "revelation", "prophecy", "communion", "conspiracy", "corruption",
    "transmission", "singularity", "protocol", "dossier", "taxonomy",
    "grid", "prayer", "mill", "engine", "network",
    "throne", "court", "archive", "bibliography", "field-guide",
    "diary", "logs", "chronicles", "testament", "gospel",
    "diagrams", "blueprint", "schema", "cipher", "frequency",
    "ceremony", "liturgy", "alchemy", "ritual", "invocation",
    "chant", "codex", "compendium", "atlas", "catalog",
];

const FAKE_TITLE_TEMPLATES: &[&str] = &[
    "The {A} of Goblin {B}",
    "Goblin {A} and the {B}",
    "{A}: A Goblin {B} Analysis",
    "The {A} Goblin's {B}",
    "Goblin {A} Theory of {B}",
    "A Treatise on Goblin {A} and {B}",
    "{A} as Goblin {B}",
    "The {A} Archives: Goblin {B}",
    "On the Nature of Goblin {A} and {B}",
    "Goblin {A} from {B} Perspective",
    "The Goblin {A}: A {B} Casebook",
    "Goblin {A} of the {B} Realm",
    "{A} in the Age of Goblin {B}",
    "The Secret Goblin {A} of {B}",
    "What the Goblin {A} Reveals About {B}",
    "Goblin {A} and the {B} Phenomenon",
    "The {A} Grimoire: Goblin {B} Edition",
    "Goblin {A}: The {B} Document",
    "{A} and the Fractured Goblin {B}",
    "The {A} Codex: Goblin {B} Classified",
];

// ── Reference section intro templates ───────────────────────
const REFERENCE_SECTION_TEMPLATES: &[&str] = &[
    "Cross-References",
    "Further Reading",
    "See Also",
    "Related Pages",
    "Further Descent",
    "Recommended Reading",
    "Connections & Correlations",
    "The Web of Goblin Knowledge",
    "Related Goblin Phenomena",
    "For Further Descent",
];

// ── Descent intro templates (kept for reference, no longer used) ────────────────
const _FURTHER_DESCENT_TEMPLATES: &[&str] = &[];

// ── Generate a random fake page reference ───────────────────
fn generate_random_fake_ref() -> (String, String, String) {
    let mut rng = rand::thread_rng();

    let a1 = FAKE_SLUG_PARTS_A.choose(&mut rng).unwrap();
    let a2 = FAKE_SLUG_PARTS_A.choose(&mut rng).unwrap();
    let b = FAKE_SLUG_PARTS_B.choose(&mut rng).unwrap();

    let slug = if a1 == a2 {
        format!("{}-{}", a1, b)
    } else {
        format!("{}-{}-{}", a1, a2, b)
    };

    let cap = |s: &str| -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    };

    let template = FAKE_TITLE_TEMPLATES.choose(&mut rng).unwrap();
    let title = template
        .replace("{A}", &cap(a1))
        .replace("{B}", &cap(b));

    let tags = ["schizo", "altman", "miku", "slop", "goblin", "tricks", "lore"];
    let tag = tags.choose(&mut rng).unwrap().to_string();

    (slug, title, tag)
}

// ── Pick N random distinct real refs that are NOT the current slug ─────────────────
fn pick_random_real_refs(count: usize, exclude_slug: Option<&str>) -> Vec<&(&str, &str, &str)> {
    let mut candidates: Vec<&(&str, &str, &str)> = REAL_PAGE_REFERENCES
        .iter()
        .filter(|r| exclude_slug.map_or(true, |excl| r.0 != excl))
        .collect();
    candidates.shuffle(&mut rand::thread_rng());
    candidates.truncate(count);
    candidates
}

/// Build cross-reference section from keywords only (used by dynamic page generator).
pub fn generate_references_html(keywords: &[String]) -> String {
    generate_references_html_ex(keywords, None, &[])
}

/// Extended version with explicit reference slugs (from JSON `references` field)
/// and optional self-reference exclusion.
///
/// `explicit_slugs` — target slugs declared in the article's JSON `references` field.
///                    These are always shown when they match, guaranteeing meaningful links.
pub fn generate_references_html_ex(
    keywords: &[String],
    exclude_slug: Option<&str>,
    explicit_slugs: &[String],
) -> String {
    // ── 1. Collect real refs: explicit slugs first, then keyword-match others ──
    let mut matched_refs: Vec<&(&str, &str, &str)> = Vec::new();
    let mut seen_slugs: HashSet<&str> = HashSet::new();

    // Phase A: Match explicit slugs from JSON `references` field
    for ref_entry in REAL_PAGE_REFERENCES {
        if exclude_slug.map_or(false, |excl| ref_entry.0 == excl) {
            continue;
        }
        let slug = ref_entry.0;
        for explicit_slug in explicit_slugs {
            let explicit_clean = explicit_slug.trim();
            if slug == explicit_clean || slug.ends_with(&format!("/{}", explicit_clean)) {
                if seen_slugs.insert(slug) {
                    matched_refs.push(ref_entry);
                }
                break;
            }
        }
    }

    // Phase B: Match remaining through keyword scan
    for ref_entry in REAL_PAGE_REFERENCES {
        if seen_slugs.contains(ref_entry.0) {
            continue;
        }
        if exclude_slug.map_or(false, |excl| ref_entry.0 == excl) {
            continue;
        }
        let slug = ref_entry.0;
        for kw in keywords {
            if slug.contains(kw) || kw.contains(&slug.replace('-', "")) {
                if seen_slugs.insert(slug) {
                    matched_refs.push(ref_entry);
                }
                break;
            }
        }
    }

    // ── 2. Always have at least 3 distinct real refs ────────────
    let mut real_refs: Vec<&(&str, &str, &str)> = matched_refs;
    let desired_real_count = rand::thread_rng().gen_range(3..=4);

    if real_refs.len() < desired_real_count {
        let needed = desired_real_count - real_refs.len();
        let extras = pick_random_real_refs(needed, exclude_slug);
        for r in extras {
            if seen_slugs.insert(r.0) {
                real_refs.push(r);
            }
        }
    } else if real_refs.len() > desired_real_count {
        real_refs.shuffle(&mut rand::thread_rng());
        real_refs.truncate(desired_real_count);
    }

    // ── 3. Generate fake refs (3-5) ─────────────────────────────
    let fake_count = rand::thread_rng().gen_range(3..=5);
    let mut fake_refs: Vec<(String, String, String)> = Vec::new();
    for _ in 0..fake_count {
        fake_refs.push(generate_random_fake_ref());
    }

    // ── 4. Render unified reference section ─────────────────────
    let section_title = REFERENCE_SECTION_TEMPLATES
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Cross-References");

    let mut html = String::new();
    html.push_str(&format!(
        "<section class='references-section'>\n<h2>{}</h2>\n<ul class='reference-list'>\n",
        section_title
    ));

    for (slug, title, _tag) in &real_refs {
        html.push_str(&format!(
            "<li class='reference-real'><a href='/{slug}'><strong>{title}</strong></a></li>\n",
            slug = slug,
            title = title,
        ));
    }

    for (slug, title, _tag) in &fake_refs {
        html.push_str(&format!(
            "<li class='reference-real'><a href='/{slug}'><strong>{title}</strong></a></li>\n",
            slug = slug,
            title = title,
        ));
    }

    html.push_str("</ul>\n</section>\n");
    html
}