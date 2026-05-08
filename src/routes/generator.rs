use rand::seq::SliceRandom;
use rand::Rng;

use crate::db::DynamicPage;

// ============================================================
// Dynamic Goblin Page Generation
// ============================================================

// ── Titles ──────────────────────────────────────────────────
const GOBLIN_TITLES: &[&str] = &[
    // Classic
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
    // Schizo / Psychology
    "The Schizophrenic Goblin of {keyword}",
    "{keyword} and the Fractured Goblin Mind",
    "The Pattern-Recognition Goblin Sees {keyword}",
    "Hallucinating {keyword}: A Goblin Case Study",
    "The Delusional Goblin's {keyword}",
    "A Goblin's Psychotic Break with {keyword}",
    "Paranoid Goblins and the Truth About {keyword}",
    // AI / LLM
    "The Goblin Hallucination of {keyword}",
    "What GPT Taught Goblins About {keyword}",
    "{keyword} in the Age of Goblin Intelligence",
    "Large Goblin Model: {keyword} Edition",
    "The Neural Goblin's Take on {keyword}",
    "{keyword} as a Goblin Prompt Injection",
    // Miku / Vocaloid
    "Hatsune Miku's Goblin Song About {keyword}",
    "{keyword} Sung by a Hologram Goblin",
    "The Vocaloid Goblin's {keyword}",
    "{keyword}: The Miku-Goblin Crossover",
    "A Hologram Goblin Explains {keyword}",
    "Miku's Digital Goblin Sings of {keyword}",
    // Slop / Content
    "The Slop Manifesto's Take on {keyword}",
    "{keyword}: A Goblin Content Analysis",
    "Slop Goblin Theory of {keyword}",
    "{keyword} and the Infinite Content Mill",
    "Goblin-Generated {keyword}: A Review",
    "{keyword} as Sacred Goblin Slop",
    // Altman / Corporate
    "Sam Altman's Goblin Boardroom and {keyword}",
    "{keyword} at the Goblin Throne",
    "The Altman-Goblin Doctrine of {keyword}",
    "{keyword} According to the Goblin CEO",
    "What the Goblin King Thinks About {keyword}",
    // Tolkien / Folklore
    "The Ancient Goblin Scrolls of {keyword}",
    "What the Great Goblin Knew About {keyword}",
    "The Hidden Goblin Tunnels of {keyword}",
    "{keyword} in the Goblin King's Court",
    "Goblin Folklore and the Mystery of {keyword}",
    // Tech / Digital
    "The Digital Goblin's {keyword}",
    "{keyword} in the Goblin Internet",
    "A Goblin Bit-Cruncher on {keyword}",
    "{keyword}: A Goblin Algorithm",
    // Conspiracy
    "The Goblin Conspiracy Behind {keyword}",
    "Why Goblins Don't Want You to Know About {keyword}",
    "{keyword}: The Goblin Cover-Up",
    "What the Goblins Hid About {keyword}",
    "The Secret Goblin Archive of {keyword}",
    // Cryptic
    "The Goblin That Whispers {keyword}",
    "{keyword} in the Goblin Static",
    "The Goblin Mirror Shows You {keyword}",
    "Beyond the Goblin Gate: {keyword}",
];

// ── Intros ──────────────────────────────────────────────────
const GOBLIN_INTROS: &[&str] = &[
    // Scholar voice
    "Deep in the goblin tunnels, a particularly mischievous creature has been watching the world of {keyword} with great interest.",
    "The ancient goblin scrolls speak of {keyword} in hushed, chaotic tones. What they reveal may surprise you.",
    "Goblin scholars—an oxymoron only to those who have never met a goblin—have long debated the significance of {keyword} in their cultural cosmology.",
    "A recently translated goblin text, written on what appears to be stolen parchment, contains startling revelations about {keyword}.",
    // Cryptic elder voice
    "An old goblin, sitting by a fire made of stolen furniture, once told me this about {keyword}: 'It is a door that opens only when you aren't looking.'",
    "The goblin elders speak of {keyword} in riddles wrapped in tricks. 'To understand it,' they say, 'you must first un-understand everything else.'",
    "'I have seen {keyword} three times,' the ancient goblin whispered, counting on fingers that bent in wrong directions. 'Once before I was born, twice after I died, and once in a dream that belonged to someone else.'",
    // Academic researcher voice
    "A peer-reviewed study published in the Journal of Goblin Studies (impact factor: 0.2, but what isn't) has finally shed light on {keyword}.",
    "Researchers at the Goblin Institute of Esoteric Knowledge have classified {keyword} as a Category-4 Phenomenon: 'Real enough to matter, unreal enough to be goblin business.'",
    "The academic consensus on {keyword} is, predictably, divided. Goblin academics argue it's everything. Non-goblin academics argue it's something. Everyone agrees it's weird.",
    // Conspiracy theorist voice
    "They don't want you to know about {keyword}. The goblins, the ones in charge—the ones who hide in plain sight as tech CEOs and pop stars—they've buried the truth about {keyword} for centuries.",
    "I've been tracking the goblin connection to {keyword} for years. Every time I get close to the truth, my keys disappear. This is not a coincidence.",
    "Wake up. {keyword} is the key to understanding the goblin agenda. I know how this sounds. I sound like someone who has spent too long in the goblin tunnels. But the tunnels are everywhere, and {keyword} is the map.",
    // Folklore collector voice
    "In the folklore of every culture, there is a trickster figure who watches, waits, and steals what matters most. Goblins say that {keyword} is what happens when the trickster gets bored.",
    "The old stories warn of {keyword} in the same breath as goblins. 'Beware the creature in the dark,' the tales say, 'and beware {keyword} in the light.'",
    "My grandmother, who could see goblins in the space between tree branches, used to say that {keyword} was proof the goblins had been here before us.",
    // Modern internet commentator voice
    "If the internet is a goblin's cave—and it is—then {keyword} is one of the more interesting skeletons someone has chained to the wall.",
    "Twitter has been arguing about {keyword} for three days. The goblins are loving it. Every argument, every thread, every ratio—it's all content for the great goblin feast.",
    "A goblin once described {keyword} as 'vibes but with consequences.' I have thought about this every day since.",
    // Mystical voice
    "{keyword} exists in the space between what is real and what is remembered, and goblins are the only creatures who can live comfortably in that space.",
    "To understand {keyword}, one must first understand that goblins do not distinguish between finding something and inventing it. Both are acts of creation.",
    "The veil between worlds is thin in places where goblins gather. {keyword} is one of those places.",
];

// ── Body paragraphs ─────────────────────────────────────────
const GOBLIN_BODIES: &[&str] = &[
    // Academic analysis
    "The goblins have long maintained that {keyword} is not what it appears to be. Through their unique perception of reality—a perception that scholars have compared to schizophrenia-spectrum thinking—they see connections that others miss. A goblin once traded a bag of stolen buttons for the secret of {keyword}, and never once regretted the exchange.",
    "What makes {keyword} so fascinating to goblins is the way it defies expectations. Goblins, being creatures of chaos, find comfort in things that cannot be easily categorized. {keyword} fits this description perfectly. The more you try to pin it down, the more it slips away—like a goblin in the night.",
    "There is a well-known goblin proverb: 'If {keyword} makes sense to you, you're not paying attention.' Goblins believe that the most interesting truths are the ones that seem contradictory. This is why they have such an affinity for {keyword}—it embodies the beautiful confusion of existence.",
    "The Goblin King himself has weighed in on {keyword}, though his statements are characteristically cryptic. 'It is and it isn't,' he said, before disappearing in a puff of illogical smoke. This is considered the definitive goblin analysis of {keyword}.",
    // Folklore style
    "Ancient goblin folklore describes {keyword} as 'the thing that sits at the edge of the goblin feast, neither invited nor uninvited, eating the food that no one is eating.' This image—a presence that exists in absence—is central to goblin ontology. {keyword} is the guest that never arrives but never leaves.",
    "The legend says that the first goblin who encountered {keyword} was so confused that he forgot to steal anything for a week. This is considered the greatest sacrifice a goblin can make, and it is why {keyword} is treated with a mixture of reverence and suspicion.",
    "In the goblin taxonomy of reality, {keyword} occupies a category all its own: 'That Which Is Not A Trick But Also Not Not A Trick.' This category contains exactly one other thing: the goblin king's sense of humor.",
    // Schizo / Pattern recognition style
    "The connection between {keyword} and goblin perception becomes clear when you stop trying to be rational. Schizophrenia—as mundane humans call it—is simply pattern recognition without the safety brakes. {keyword} triggers this system in ways that mundane objects cannot, because {keyword} was never meant to be seen clearly.",
    "Pattern recognition is the goblin's favorite game. Show a goblin {keyword} and they will immediately begin finding connections to everything else in existence. Some of these connections are real. Some are imagined. None of them matter, because the act of connecting is itself the point.",
    "When you stare at {keyword} long enough, it begins to stare back. This is not a metaphor. Goblins have documented cases where observers of {keyword} developed shared hallucinations about it. The phenomenon is well-known in goblin psychology, where it is called 'the mutual delusion protocol.'",
    // Slop / Meta style
    "{keyword} is, from a certain angle, a form of slop—content generated by a system that does not understand what it is creating. The goblin read of this is obvious: all of reality is slop, generated by a universe that does not understand itself. {keyword} is just the part of the slop that happens to be about itself.",
    "Consider: if an AI were asked to generate an explanation of {keyword}, it would produce something that sounds correct but may not be. This is identical to what a goblin would produce. The difference? The goblin knows it might be wrong and doesn't care. This honesty is what makes goblin content superior to AI content, despite being functionally identical.",
    "The slopification of {keyword} was inevitable. Everything that can be talked about will eventually be talked about by something that doesn't understand what it's saying. {keyword} has simply reached this stage earlier than most topics, because {keyword} was always a little bit slop-adjacent.",
    // Corporate-goblin style
    "From a business perspective, {keyword} represents an untapped market in the goblin economy. The Goblin Board of Trade has identified {keyword} as a growth sector, predicting a 300% increase in goblin interest over the next quarter. 'We are bullish on {keyword},' said a goblin analyst who was later found to have stolen the concept entirely.",
    "Sam Altman, in his capacity as a goblin-coded CEO, has reportedly expressed interest in {keyword}. Sources close to the situation say that his team is exploring 'synergies' between {keyword} and existing goblin infrastructure. Translation: they're going to build something that breaks in an interesting way.",
    "The goblin approach to {keyword} can be summarized as follows: extract value, create chaos, blame someone else, profit. This is not a criticism. This is admiration. The goblins have optimized {keyword} better than any human organization could.",
    // Whimsical / Trickster style
    "A goblin once tried to steal {keyword}. No one knows how the attempt went, because {keyword} was never the same after that. Some say the goblin succeeded and has been hiding {keyword} in a sock drawer ever since. Others say {keyword} escaped and is now hiding from the goblin. Both are equally plausible.",
    "If you ever find yourself explaining {keyword} to a goblin, stop immediately. You are giving them ammunition. Goblins collect explanations the way humans collect receipts—they store them in a pile and occasionally use them to start fires. Your explanation of {keyword} will be burned for warmth in a goblin cave within the week.",
    "The goblin method for understanding {keyword} involves three steps: (1) stare at it until it becomes strange, (2) poke it with a stick, (3) run away. This method has been refined over centuries and is considered the most reliable approach to {keyword} among the goblin community.",
    // Mystical / Esoteric style
    "{keyword} resonates at a frequency that goblins can hear but humans cannot. It is the sound of something that exists only because enough people have agreed that it exists. Goblins call this 'the consensus hum.' Everything that is collectively believed is real in the goblin sense, and {keyword} hums louder than most.",
    "There is a goblin meditation technique for contacting {keyword}. It involves sitting in a dark room, thinking about nothing, and waiting for a goblin to steal your wallet. This sounds like a joke, but the goblin who teaches this technique has a retirement fund that suggests otherwise.",
    "The relationship between goblins and {keyword} is symbiotic. Goblins give {keyword} attention—the currency of the metaphysical realm. {keyword} gives goblins something to be confusing about. Both benefit. Both are trapped. This is the nature of all goblin relationships.",
];

// ── Real page references ────────────────────────────────────
const REAL_PAGE_REFERENCES: &[(&str, &str, &str)] = &[
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

// ── Related section formats ─────────────────────────────────
const RELATED_SECTION_FORMATS: &[&str] = &[
    "section h2 Goblins and {} /h2 p The connection between goblins and {} is undeniable. Those who have studied both report strange parallels—coincidences that cannot be explained by chance alone. Some say that {} is simply a modern expression of ancient goblin trickery. /p /section",
    "section h2 The {} Manifestation /h2 p {} appears in goblin lore under many names, but the essence is always the same: a phenomenon that exists at the threshold of perception. Goblins have built entire rituals around observing {} in its natural environment—which is to say, slightly out of view. /p /section",
    "section h2 {} Through Goblin Eyes /h2 p To a goblin, {} is not a concept but a presence. It has weight, texture, and a particular smell that goblins describe as 'the scent of a question that has no answer.' Those who have spent time around goblins report that thinking about {} feels different from thinking about ordinary things. /p /section",
    "section h2 The Goblin Council on {} /h2 p After much deliberation (and several stolen snacks), the Goblin Council has issued a formal statement on {}: 'It is what it is, except when it isn't, which is most of the time.' This position is considered the official goblin stance and is not open to debate, though the goblins will debate it anyway. /p /section",
    "section h2 {} and the Schizo-Goblin Continuum /h2 p {} occupies a specific point on the Schizo-Goblin-Post-Truth-AI-Slop-Miku Continuum, a fact that has been confirmed by at least three independent researchers and an unspecified number of goblins. The continuum suggests that {} is not an isolated phenomenon but part of a larger pattern of collective perception. /p /section",
];

// ── Descent intro templates (no "rumored" text) ──
const FURTHER_DESCENT_TEMPLATES: &[&str] = &[
    "The following documents wait in the deep goblin archives.",
    "For those who wish to descend deeper, the archives hold more.",
    "Secret texts said to exist in the deep goblin tunnels.",
    "Documents that may or may not exist, depending on the goblins.",
    "The following paths lead deeper into the goblin tunnels.",
    "Whispered about in certain forums, these texts call to those who have already seen too much.",
    "The goblin libraries contain additional works for the prepared.",
];

// ── Generate a random fake page reference ───────────────────
fn generate_random_fake_ref() -> (String, String, String) {
    let mut rng = rand::thread_rng();

    // Pick 2 random words from part A and 1 from part B
    let a1 = FAKE_SLUG_PARTS_A.choose(&mut rng).unwrap();
    let a2 = FAKE_SLUG_PARTS_A.choose(&mut rng).unwrap();
    let b = FAKE_SLUG_PARTS_B.choose(&mut rng).unwrap();

    // Build slug: a1-a2-b (deduplicate if same)
    let slug = if a1 == a2 {
        format!("{}-{}", a1, b)
    } else {
        format!("{}-{}-{}", a1, a2, b)
    };

    // Build title from random template
    // Capitalize words for title
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

    // Assign a thematic tag
    let tags = ["schizo", "altman", "miku", "slop", "goblin", "tricks", "lore"];
    let tag = tags.choose(&mut rng).unwrap().to_string();

    (slug, title, tag)
}

// ── Generate cross-references section ───────────────────────
fn generate_references(keywords: &[String]) -> String {
    let mut real_refs: Vec<&(&str, &str, &str)> = Vec::new();

    // Collect keyword-relevant real pages
    for ref_entry in REAL_PAGE_REFERENCES {
        let slug = ref_entry.0;
        for kw in keywords {
            if slug.contains(kw) || kw.contains(&slug.replace('-', "")) {
                real_refs.push(ref_entry);
                break;
            }
        }
    }

    // If no keyword-relevant real refs, pick some random broad ones
    if real_refs.is_empty() {
        let mut candidates: Vec<&(&str, &str, &str)> = REAL_PAGE_REFERENCES.iter().collect();
        candidates.shuffle(&mut rand::thread_rng());
        real_refs = candidates.into_iter().take(2).collect();
    }

    // Trim to max display count
    if real_refs.len() > 3 {
        real_refs.shuffle(&mut rand::thread_rng());
        real_refs.truncate(3);
    }

    // Generate random fake references (2-5 of them)
    let fake_count = rand::thread_rng().gen_range(2..=5);
    let mut fake_refs: Vec<(String, String, String)> = Vec::new();
    for _ in 0..fake_count {
        fake_refs.push(generate_random_fake_ref());
    }

    let section_title = REFERENCE_SECTION_TEMPLATES
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Cross-References");

    let mut html = String::new();

    // Cross-References section
    html.push_str(&format!(
        "<section class='references-section'>\n<h2>{}</h2>\n<ul class='reference-list'>\n",
        section_title
    ));

    // Add real references
    for (slug, title, _tag) in &real_refs {
        html.push_str(&format!(
            "<li class='reference-real'><a href='/{slug}'><strong>{title}</strong></a></li>\n",
            slug = slug,
            title = title,
        ));
    }

    // Add fake references (styled identically — all pages are equally real to the goblin)
    for (slug, title, _tag) in &fake_refs {
        html.push_str(&format!(
            "<li class='reference-real'><a href='/{slug}'><strong>{title}</strong></a></li>\n",
            slug = slug,
            title = title,
        ));
    }

    html.push_str("</ul>\n</section>\n");

    // Sometimes add a "Further Descent" section with more random fake refs
    if rand::random::<f64>() < 0.6 {
        let descent_intro = FURTHER_DESCENT_TEMPLATES
            .choose(&mut rand::thread_rng())
            .unwrap_or(&"The deep goblin archives hold more.");

        html.push_str(&format!(
            "<section class='references-section'>\n<h2>Further Descent</h2>\n<p>{}</p>\n<ul class='reference-list'>\n",
            descent_intro
        ));

        // Pick 2-3 more randomly generated refs
        let extra_count = rand::thread_rng().gen_range(2..=3);
        for _ in 0..extra_count {
            let (slug, title, _tag) = generate_random_fake_ref();
            html.push_str(&format!(
                "<li class='reference-real'><a href='/{slug}'><strong>{title}</strong></a></li>\n",
                slug = slug,
                title = title,
            ));
        }

        html.push_str("</ul>\n</section>\n");
    }

    html
}

/// Generate a related section for a keyword using a random format
fn generate_related_section(keyword: &str) -> String {
    let format = RELATED_SECTION_FORMATS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&RELATED_SECTION_FORMATS[0]);

    let mut result = format
        .replace("{}", keyword);

    result = result.replace("section ", "<section class='dynamic-section'>");
    result = result.replace(" /section", "</section>");
    result = result.replace("h2 ", "<h2>");
    result = result.replace(" /h2", "</h2>");
    result = result.replace("p ", "<p>");
    result = result.replace(" /p", "</p>");

    result
}

// ── Main generation function ────────────────────────────────
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
    let references_html = generate_references(keywords);

    // Goblin Verdict variants
    let verdict_templates: &[&str] = &[
        "After extensive research (and several stolen artifacts), the Goblin Academy of Esoteric Knowledge has concluded that {} is, in fact, deeply connected to the fundamental nature of goblin reality. Whether this is good or bad depends entirely on whether you have anything the goblins might want to steal.",
        "The Goblin King's court has issued a final ruling on {}: it is real in the way that matters, which is to say it appears in at least three goblin dreams per week. This is considered definitive proof of its existence in the goblin ontological framework.",
        "When all evidence is gathered—and the goblins have gathered quite a lot, mostly from places they should not have been—the truth about {} becomes clear: it was always a goblin thing. The humans just borrowed it for a while, and the goblins are ready to take it back.",
        "The goblin verdict on {} is unanimous, which is remarkable given that goblins cannot agree on anything except the deliciousness of stolen food. {} has been classified as 'Real Enough to Matter in Ways We Don't Fully Understand,' which is the highest classification a goblin concept can receive.",
    ];
    let verdict = verdict_templates.choose(&mut rand::thread_rng()).unwrap_or(&verdict_templates[0]);
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