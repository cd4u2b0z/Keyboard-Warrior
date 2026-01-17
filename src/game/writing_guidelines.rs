//! Writing Guidelines and Tone System
//!
//! This module establishes the literary voice, tone standards, and writing
//! principles for all game text. It serves as both documentation and
//! programmatic rules for text generation.
//!
//! Our narrative models:
//! - Gene Wolfe's unreliable narrators and layered meaning
//! - Ursula K. Le Guin's economy of language and emotional precision
//! - Dark Souls' environmental storytelling through items
//! - Disco Elysium's internal voice and psychological depth
//! - Planescape Torment's philosophical weight and consequence

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CORE PRINCIPLES
// ============================================================================

/// The fundamental writing principles that guide all text
#[derive(Debug, Clone)]
pub struct WritingPrinciples {
    /// Show don't tell - let readers discover meaning
    pub indirect_revelation: IndirectRevelation,
    /// Every word earns its place
    pub economy_of_language: EconomyOfLanguage,
    /// Reward attention and re-reading
    pub layered_meaning: LayeredMeaning,
    /// Characters have psychological reality
    pub psychological_depth: PsychologicalDepth,
    /// Actions have visible consequences
    pub consequence_visibility: ConsequenceVisibility,
}

impl WritingPrinciples {
    pub fn canonical() -> Self {
        Self {
            indirect_revelation: IndirectRevelation::canonical(),
            economy_of_language: EconomyOfLanguage::canonical(),
            layered_meaning: LayeredMeaning::canonical(),
            psychological_depth: PsychologicalDepth::canonical(),
            consequence_visibility: ConsequenceVisibility::canonical(),
        }
    }
}

/// Indirect revelation - Gene Wolfe principle
/// 
/// Never state something directly that can be implied.
/// Trust the reader to connect dots.
/// Let mystery enhance rather than frustrate.
#[derive(Debug, Clone)]
pub struct IndirectRevelation {
    /// Things we never state directly
    pub always_imply: Vec<String>,
    /// Things we can state when appropriate
    pub sometimes_state: Vec<String>,
    /// Things we always state clearly (game mechanics, etc.)
    pub always_state: Vec<String>,
}

impl IndirectRevelation {
    pub fn canonical() -> Self {
        Self {
            always_imply: vec![
                "The player's true identity (until late game)".to_string(),
                "The full truth of the First Silence".to_string(),
                "Character's deepest motivations".to_string(),
                "The nature of the Corruption's origin".to_string(),
                "What happened to the First Speaker's spouse".to_string(),
                "The connection between player's typing and reality".to_string(),
            ],
            sometimes_state: vec![
                "Historical facts (through documents)".to_string(),
                "NPC backstories (through dialogue, earned)".to_string(),
                "Faction philosophies (through demonstration)".to_string(),
                "The consequences of player choices".to_string(),
            ],
            always_state: vec![
                "Game mechanics and controls".to_string(),
                "Immediate dangers and threats".to_string(),
                "Quest objectives (when accepted)".to_string(),
                "Item effects and stats".to_string(),
                "What typing is required in combat".to_string(),
            ],
        }
    }
}

/// Economy of language - Le Guin principle
///
/// Every word must earn its place.
/// Cut adjectives ruthlessly.
/// Prefer Anglo-Saxon words over Latinate.
/// One strong verb beats three weak ones.
#[derive(Debug, Clone)]
pub struct EconomyOfLanguage {
    /// Maximum sentence length guidelines by context
    pub max_sentence_length: HashMap<String, usize>,
    /// Words to avoid (overused, vague)
    pub banned_words: Vec<String>,
    /// Preferred replacements
    pub preferred_alternatives: HashMap<String, String>,
}

impl EconomyOfLanguage {
    pub fn canonical() -> Self {
        let mut max_length = HashMap::new();
        max_length.insert("combat".to_string(), 12);
        max_length.insert("dialogue".to_string(), 20);
        max_length.insert("description".to_string(), 25);
        max_length.insert("lore".to_string(), 30);
        max_length.insert("internal_thought".to_string(), 15);
        
        let mut alternatives = HashMap::new();
        alternatives.insert("utilize".to_string(), "use".to_string());
        alternatives.insert("implement".to_string(), "do/make".to_string());
        alternatives.insert("commence".to_string(), "begin/start".to_string());
        alternatives.insert("terminate".to_string(), "end/stop".to_string());
        alternatives.insert("subsequently".to_string(), "then/later".to_string());
        alternatives.insert("approximately".to_string(), "about".to_string());
        alternatives.insert("instantaneously".to_string(), "at once".to_string());
        alternatives.insert("facilitate".to_string(), "help/ease".to_string());
        
        Self {
            max_sentence_length: max_length,
            banned_words: vec![
                "very".to_string(),
                "really".to_string(),
                "quite".to_string(),
                "rather".to_string(),
                "somewhat".to_string(),
                "basically".to_string(),
                "literally".to_string(),
                "actually".to_string(),
                "definitely".to_string(),
                "absolutely".to_string(),
                // Generic video game language to avoid
                "epic".to_string(),
                "awesome".to_string(),
                "legendary".to_string(),
                "ultimate".to_string(),
            ],
            preferred_alternatives: alternatives,
        }
    }
}

/// Layered meaning - Dark Souls principle
///
/// First read gives plot.
/// Second read gives character.
/// Third read gives theme.
/// Every piece of text rewards attention.
#[derive(Debug, Clone)]
pub struct LayeredMeaning {
    /// Surface layer - immediate plot information
    pub surface_layer: LayerDescription,
    /// Character layer - reveals personality/motivation
    pub character_layer: LayerDescription,
    /// Thematic layer - connects to larger ideas
    pub thematic_layer: LayerDescription,
}

#[derive(Debug, Clone)]
pub struct LayerDescription {
    pub description: String,
    pub examples: Vec<String>,
}

impl LayeredMeaning {
    pub fn canonical() -> Self {
        Self {
            surface_layer: LayerDescription {
                description: "What is literally happening. The plot.".to_string(),
                examples: vec![
                    "Example: 'The scribe's hands shook as she typed.'".to_string(),
                    "Surface meaning: A scribe is typing nervously.".to_string(),
                ],
            },
            character_layer: LayerDescription {
                description: "What this reveals about people and relationships.".to_string(),
                examples: vec![
                    "Same text: 'The scribe's hands shook as she typed.'".to_string(),
                    "Character layer: She's not confident. Why? Fear? Guilt? Age?".to_string(),
                ],
            },
            thematic_layer: LayerDescription {
                description: "How this connects to the game's central ideas.".to_string(),
                examples: vec![
                    "Same text: 'The scribe's hands shook as she typed.'".to_string(),
                    "Thematic layer: Even masters can't fully control the word. \
                    Language has power over its users.".to_string(),
                ],
            },
        }
    }
}

/// Psychological depth - Disco Elysium principle
///
/// Characters have inner lives.
/// Contradictions are human.
/// People change (or resist change).
/// No one sees themselves as the villain.
#[derive(Debug, Clone)]
pub struct PsychologicalDepth {
    /// Every NPC should have:
    pub npc_requirements: Vec<String>,
    /// Every faction should have:
    pub faction_requirements: Vec<String>,
    /// The player should feel:
    pub player_feelings: Vec<String>,
}

impl PsychologicalDepth {
    pub fn canonical() -> Self {
        Self {
            npc_requirements: vec![
                "A fear (stated or implied)".to_string(),
                "A hope (stated or implied)".to_string(),
                "A contradiction (stated or implied)".to_string(),
                "A reason they believe they're right".to_string(),
                "Something they're hiding".to_string(),
            ],
            faction_requirements: vec![
                "A correct observation about the world".to_string(),
                "An incorrect conclusion from that observation".to_string(),
                "A justified grievance with another faction".to_string(),
                "A blind spot they refuse to examine".to_string(),
                "Something valuable they protect".to_string(),
            ],
            player_feelings: vec![
                "Curious about the mystery".to_string(),
                "Conflicted about faction choices".to_string(),
                "Growing unease about their own identity".to_string(),
                "Genuine emotional connection to NPCs".to_string(),
                "Weight of their choices".to_string(),
            ],
        }
    }
}

/// Consequence visibility - Planescape Torment principle
///
/// Choices should matter.
/// Effects should be visible.
/// Not all consequences are immediate.
/// Some consequences contradict expectations.
#[derive(Debug, Clone)]
pub struct ConsequenceVisibility {
    /// Immediate feedback (same scene)
    pub immediate_effects: Vec<String>,
    /// Short-term effects (within the run)
    pub short_term_effects: Vec<String>,
    /// Long-term effects (across runs)
    pub long_term_effects: Vec<String>,
}

impl ConsequenceVisibility {
    pub fn canonical() -> Self {
        Self {
            immediate_effects: vec![
                "NPC dialogue changes".to_string(),
                "Reputation visible shift".to_string(),
                "Environmental reaction".to_string(),
                "Item or resource change".to_string(),
            ],
            short_term_effects: vec![
                "NPC remembers and references choice".to_string(),
                "New encounter becomes available".to_string(),
                "Faction response changes".to_string(),
                "Story branch opens or closes".to_string(),
            ],
            long_term_effects: vec![
                "Meta-progression unlocks".to_string(),
                "Character's fate in future runs".to_string(),
                "World state changes persist".to_string(),
                "New dialogue options unlock".to_string(),
            ],
        }
    }
}

// ============================================================================
// TONE BY LOCATION
// ============================================================================

/// Tone guidelines for each major location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTone {
    pub location: String,
    pub primary_mood: String,
    pub secondary_moods: Vec<String>,
    pub vocabulary_style: String,
    pub sentence_rhythm: String,
    pub example_description: String,
}

pub fn location_tones() -> HashMap<String, LocationTone> {
    let mut tones = HashMap::new();
    
    tones.insert("haven".to_string(), LocationTone {
        location: "Haven".to_string(),
        primary_mood: "Cautious hope".to_string(),
        secondary_moods: vec![
            "Wary community".to_string(),
            "Hard-won peace".to_string(),
            "Fragile safety".to_string(),
        ],
        vocabulary_style: "Plain, warm, practical. Working-class diction. \
            Characters say 'ain't' and 'reckon.' Comfortable.".to_string(),
        sentence_rhythm: "Medium length. Conversational. Like talking to a neighbor.".to_string(),
        example_description: "The inn smells of woodsmoke and something almost like coffee. \
            Mismatched chairs ring a fireplace that's seen better centuries. People nod \
            as you enter—not friendly exactly, but not hostile either. Earned trust \
            is the currency here.".to_string(),
    });
    
    tones.insert("athenaeum".to_string(), LocationTone {
        location: "Athenaeum".to_string(),
        primary_mood: "Reverent mystery".to_string(),
        secondary_moods: vec![
            "Hushed wonder".to_string(),
            "Dangerous knowledge".to_string(),
            "Hidden threats".to_string(),
        ],
        vocabulary_style: "Elevated but not pretentious. Academic without jargon. \
            Characters are precise. Words are chosen carefully.".to_string(),
        sentence_rhythm: "Longer, more complex. Like reading a well-written article. \
            Comfortable with subclauses.".to_string(),
        example_description: "The shelves rise beyond sight, their heights lost in a \
            darkness that might be shadow or might be something more. Somewhere, a \
            book falls. Somewhere else, one answers. The Archivists move silently \
            between the stacks, their robes whispering secrets you're not meant to hear.".to_string(),
    });
    
    tones.insert("corruption_zone".to_string(), LocationTone {
        location: "Corruption Zone".to_string(),
        primary_mood: "Creeping wrongness".to_string(),
        secondary_moods: vec![
            "Reality sickness".to_string(),
            "Surreal danger".to_string(),
            "Distorted memory".to_string(),
        ],
        vocabulary_style: "Fragmented. Words that don't quite fit. Unexpected \
            combinations. Almost poetry, but broken poetry.".to_string(),
        sentence_rhythm: "Short. Staccato. Then suddenly long, too long, running on \
            like something that forgot how to stop, forgot what stopping means, forgot \
            forgetting—".to_string(),
        example_description: "The ground remembers being a ceiling. Words float in the \
            air, unattached to mouths or meaning. You see a tree that might have been \
            a library. A library that might have been a memory. Your shadow walks \
            ahead of you, looking back.".to_string(),
    });
    
    tones.insert("gearhold".to_string(), LocationTone {
        location: "Gearhold".to_string(),
        primary_mood: "Industrious precision".to_string(),
        secondary_moods: vec![
            "Controlled chaos".to_string(),
            "Faith in mechanism".to_string(),
            "Underlying doubt".to_string(),
        ],
        vocabulary_style: "Technical but accessible. Mechanical metaphors. \
            Clear cause-and-effect thinking in speech patterns.".to_string(),
        sentence_rhythm: "Regular. Rhythmic. Like clockwork. Each sentence a \
            gear turning.".to_string(),
        example_description: "Steam hisses from pipes that run like arteries through \
            brass walls. Somewhere, gears click through their eternal count. The \
            Mechanists work in synchronized teams, their typewriters beating time \
            like mechanical hearts. Every word has a function. Every function has \
            a word.".to_string(),
    });
    
    tones.insert("shadow_quarter".to_string(), LocationTone {
        location: "Shadow Quarter".to_string(),
        primary_mood: "Conspiratorial tension".to_string(),
        secondary_moods: vec![
            "Hidden knowledge".to_string(),
            "Necessary secrecy".to_string(),
            "Questionable means".to_string(),
        ],
        vocabulary_style: "Indirect. Euphemistic. No one says exactly what they mean. \
            Double meanings in everything.".to_string(),
        sentence_rhythm: "Varied. Unpredictable. Keeping you off balance. Like a \
            conversation where the subtext is the real text.".to_string(),
        example_description: "The alley shouldn't exist according to any map. The door \
            has no handle from the outside. The person who greets you has a name they \
            weren't born with and a smile that promises nothing. 'We've been expecting \
            you,' they say, and you're not sure if that's good or very, very bad.".to_string(),
    });
    
    tones.insert("grove".to_string(), LocationTone {
        location: "The Living Grove".to_string(),
        primary_mood: "Bittersweet acceptance".to_string(),
        secondary_moods: vec![
            "Natural grief".to_string(),
            "Resilient growth".to_string(),
            "Quiet wisdom".to_string(),
        ],
        vocabulary_style: "Organic. Flowing. Nature metaphors that feel earned. \
            Speech patterns like water—taking the path of least resistance.".to_string(),
        sentence_rhythm: "Long, unhurried. Like watching seasons change. \
            Comfortable with silence.".to_string(),
        example_description: "The trees here grow words instead of leaves. Most are \
            fragmentary—syllables that rustle in wind that doesn't exist. The Naturalists \
            move among them like gardeners, pruning meaning, composting syntax. 'The \
            Corruption isn't the enemy,' they tell you. 'It's the compost. The old \
            words must rot so new ones can grow.'".to_string(),
    });
    
    tones
}

// ============================================================================
// RECURRING MOTIFS
// ============================================================================

/// Motifs that should recur throughout the game
#[derive(Debug, Clone)]
pub struct RecurringMotif {
    pub name: String,
    pub description: String,
    pub variations: Vec<String>,
    pub first_appearance: String,
    pub revelation_moment: String,
}

pub fn narrative_motifs() -> Vec<RecurringMotif> {
    vec![
        RecurringMotif {
            name: "The Unspoken Name".to_string(),
            description: "The First Speaker's spouse's name has been erased. \
                Characters reference them indirectly.".to_string(),
            variations: vec![
                "NPCs trailing off mid-sentence when they almost say the name".to_string(),
                "Documents with the name cut out or corrupted".to_string(),
                "The player's own memories stopping short".to_string(),
                "Other characters feeling a 'shape' where the name should be".to_string(),
            ],
            first_appearance: "Chapter 1 - A character references 'the one who—' and stops.".to_string(),
            revelation_moment: "Chapter 4 - The player types the name for the first time.".to_string(),
        },
        RecurringMotif {
            name: "The Weight of Words".to_string(),
            description: "Language has physical presence in this world. Words leave marks.".to_string(),
            variations: vec![
                "Heavy words that slow you down".to_string(),
                "Light words that lift you up".to_string(),
                "Words that leave visible traces in the air".to_string(),
                "The physical exhaustion of typing truth".to_string(),
            ],
            first_appearance: "Tutorial - 'Your fingers are heavy. Each keystroke matters.'".to_string(),
            revelation_moment: "Throughout - escalating as player learns their role.".to_string(),
        },
        RecurringMotif {
            name: "The Recurring Dream".to_string(),
            description: "A dream of typing at a keyboard while someone calls your name.".to_string(),
            variations: vec![
                "Sometimes the keyboard is familiar".to_string(),
                "Sometimes the voice is close, sometimes far".to_string(),
                "The words you type keep changing".to_string(),
                "You can never quite turn to see who's calling".to_string(),
            ],
            first_appearance: "Run start - brief flash of the dream.".to_string(),
            revelation_moment: "Chapter 5 - The dream completes. You see the face. You remember.".to_string(),
        },
        RecurringMotif {
            name: "Forty-Seven".to_string(),
            description: "The number of the player's previous incarnations. Appears subtly.".to_string(),
            variations: vec![
                "'47th shelf from the left'".to_string(),
                "'Founded in the year 2947'".to_string(),
                "'The forty-seven principles'".to_string(),
                "'They've been at this for forty-seven years'".to_string(),
            ],
            first_appearance: "Background detail in multiple locations.".to_string(),
            revelation_moment: "Chapter 4 - The player learns what the number means.".to_string(),
        },
        RecurringMotif {
            name: "The Third Option".to_string(),
            description: "Whenever presented with binary choices, there's always a third.".to_string(),
            variations: vec![
                "NPCs mentioning 'another way'".to_string(),
                "Puzzles with hidden solutions".to_string(),
                "Faction philosophies hinting at synthesis".to_string(),
                "The prophecy mentioning 'the door that is not a door'".to_string(),
            ],
            first_appearance: "Chapter 1 - First binary choice has a hidden option.".to_string(),
            revelation_moment: "Ending - The Third Grammar is the synthesis of opposites.".to_string(),
        },
    ]
}

// ============================================================================
// DIALOGUE GUIDELINES
// ============================================================================

/// Guidelines for writing dialogue
#[derive(Debug, Clone)]
pub struct DialogueGuidelines {
    /// General principles
    pub principles: Vec<String>,
    /// Format rules
    pub format_rules: Vec<String>,
    /// What to avoid
    pub avoid: Vec<String>,
}

impl DialogueGuidelines {
    pub fn canonical() -> Self {
        Self {
            principles: vec![
                "Every line should do at least two things (advance plot + reveal character, etc.)".to_string(),
                "People interrupt each other. They don't wait politely.".to_string(),
                "People don't say what they mean. Subtext is king.".to_string(),
                "Different characters have different vocabularies and rhythms.".to_string(),
                "Silence and pauses are dialogue too.".to_string(),
                "People repeat themselves. They have verbal tics.".to_string(),
                "Questions often go unanswered. Or answered with other questions.".to_string(),
            ],
            format_rules: vec![
                "No dialogue tags except 'said' and 'asked' unless absolutely necessary.".to_string(),
                "Action beats instead of adverbs: 'She looked away. \"Fine.\"' not 'she said coldly.'".to_string(),
                "Keep exchanges short. No monologues unless the character would actually monologue.".to_string(),
                "When in doubt, cut it.".to_string(),
            ],
            avoid: vec![
                "'As you know, Bob' exposition.".to_string(),
                "Characters explaining their feelings instead of showing them.".to_string(),
                "Everyone speaking in the same voice.".to_string(),
                "Perfect grammar for characters who wouldn't use it.".to_string(),
                "Witty banter that doesn't fit the moment.".to_string(),
                "Characters who never make mistakes or say the wrong thing.".to_string(),
            ],
        }
    }
}
