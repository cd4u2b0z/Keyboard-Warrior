//! Encounter Writing System - Authored event text and meaningful encounters
//!
//! This module contains carefully written encounter text designed to feel
//! authored and intentional rather than randomly generated. Each encounter
//! connects to the deeper lore while remaining engaging in isolation.
//!
//! Design philosophy:
//! - Every encounter teaches something about the world
//! - Recurring characters grow and change
//! - Player choices have visible consequences
//! - Environmental details reward attention
//! - Tone varies by location but maintains coherence

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An authored encounter that can appear in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthoredEncounter {
    /// Unique identifier
    pub id: String,
    /// Display title
    pub title: String,
    /// Where this encounter can appear
    pub valid_locations: Vec<String>,
    /// Requirements for this encounter to appear
    pub requirements: EncounterRequirements,
    /// The encounter's narrative content
    pub content: EncounterContent,
    /// Possible player responses
    pub choices: Vec<EncounterChoice>,
    /// How this encounter affects the world
    pub consequences: EncounterConsequences,
    /// Can this encounter repeat?
    pub repeatable: bool,
    /// Tags for filtering and searching
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncounterRequirements {
    /// Minimum chapter to appear
    pub min_chapter: Option<u32>,
    /// Maximum chapter to appear
    pub max_chapter: Option<u32>,
    /// Required faction reputation (faction_name, min_reputation)
    pub faction_reputation: Option<(String, i32)>,
    /// Previous encounter that must have happened
    pub prerequisite_encounter: Option<String>,
    /// Previous encounter that must NOT have happened
    pub blocking_encounter: Option<String>,
    /// Required lore fragment discovered
    pub required_lore: Option<String>,
    /// Time of day (if relevant)
    pub time_of_day: Option<TimeOfDay>,
    /// Weather condition (if relevant)
    pub weather: Option<WeatherCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    Clear,
    Rain,
    Storm,
    CorruptionMist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterContent {
    /// Opening description
    pub description: String,
    /// NPC dialogue (if any)
    pub dialogue: Option<Vec<DialogueLine>>,
    /// Environmental details the player notices
    pub environmental_details: Vec<String>,
    /// The typing challenge for this encounter (if any)
    pub typing_challenge: Option<EncounterTypingChallenge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    /// Does this reveal something?
    pub reveals: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterTypingChallenge {
    pub prompt_text: String,
    pub difficulty: u32,
    pub success_narrative: String,
    pub failure_narrative: String,
    /// Partial success (>70% accuracy)
    pub partial_narrative: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterChoice {
    pub id: String,
    pub text: String,
    pub requires: Option<String>, // Skill, item, or faction
    pub consequence_id: String,
    pub typing_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncounterConsequences {
    /// Reputation changes (faction, amount)
    pub reputation_changes: Vec<(String, i32)>,
    /// Lore fragments revealed
    pub lore_revealed: Vec<String>,
    /// NPCs' opinions changed
    pub npc_opinion_changes: Vec<(String, i32)>,
    /// World state changes
    pub world_state_changes: Vec<String>,
    /// Items gained
    pub items_gained: Vec<String>,
    /// Follow-up encounters enabled
    pub enables_encounters: Vec<String>,
    /// Narrative text shown after
    pub narrative_result: String,
}

/// Build all authored encounters
pub fn build_encounters() -> HashMap<String, AuthoredEncounter> {
    let mut encounters = HashMap::new();
    
    // ========================================================================
    // HAVEN ENCOUNTERS - Relatively safe, introductory
    // ========================================================================
    
    encounters.insert("haven_stranger_arrival".to_string(), AuthoredEncounter {
        id: "haven_stranger_arrival".to_string(),
        title: "A Stranger Arrives".to_string(),
        valid_locations: vec!["haven".to_string(), "haven_inn".to_string()],
        requirements: EncounterRequirements {
            max_chapter: Some(1),
            ..Default::default()
        },
        content: EncounterContent {
            description: "The inn falls silent as a figure pushes through the door. \
                They're coated in road dust and something else—a faint shimmer that makes \
                your eyes slide away. Corruption-touched, but still coherent.

                The innkeeper reaches for the iron bell, ready to ring the warning, but \
                stops when the stranger raises both hands. Palm-up. The old gesture of \
                peace.

                'I'm looking for someone,' the stranger says. Their voice is hoarse, \
                as if they haven't spoken in days. 'Someone who types true. Someone the \
                words still trust.'

                Several patrons look at you. Word travels fast in Haven.".to_string(),
            dialogue: Some(vec![
                DialogueLine {
                    speaker: "Stranger".to_string(),
                    text: "You're the one they talk about, aren't you? The typer who doesn't \
                        make mistakes. I need your help.".to_string(),
                    reveals: None,
                },
                DialogueLine {
                    speaker: "Innkeeper".to_string(),
                    text: "Now hold on. We don't take to strangers demanding things. \
                        State your business proper-like.".to_string(),
                    reveals: None,
                },
                DialogueLine {
                    speaker: "Stranger".to_string(),
                    text: "My business is survival. There's something in the Whispering \
                        Waste. Something that used to be words. It's hunting anyone who \
                        still remembers how to read.".to_string(),
                    reveals: Some("There is a dangerous entity in the Whispering Waste.".to_string()),
                },
            ]),
            environmental_details: vec![
                "The stranger's hands are covered in small scars—typing calluses gone wrong.".to_string(),
                "Their eyes keep drifting to the bookshelves, as if reading invisible text.".to_string(),
                "When they speak, you notice their teeth are stained dark, like they've been eating ink.".to_string(),
            ],
            typing_challenge: None,
        },
        choices: vec![
            EncounterChoice {
                id: "help_stranger".to_string(),
                text: "I'll help you. Tell me more about this threat.".to_string(),
                requires: None,
                consequence_id: "help_stranger_result".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "refuse_stranger".to_string(),
                text: "I don't know you. Find someone else.".to_string(),
                requires: None,
                consequence_id: "refuse_stranger_result".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "test_stranger".to_string(),
                text: "First, prove you're not too far gone. Type something true.".to_string(),
                requires: None,
                consequence_id: "test_stranger_result".to_string(),
                typing_required: true,
            },
        ],
        consequences: EncounterConsequences {
            enables_encounters: vec!["waste_investigation".to_string()],
            narrative_result: "The stranger watches you with desperate hope. Whatever's \
                in the Waste has clearly shaken them badly.".to_string(),
            ..Default::default()
        },
        repeatable: false,
        tags: vec!["introduction".to_string(), "stranger".to_string(), "quest_hook".to_string()],
    });
    
    encounters.insert("haven_old_scribe".to_string(), AuthoredEncounter {
        id: "haven_old_scribe".to_string(),
        title: "The Retired Scribe".to_string(),
        valid_locations: vec!["haven".to_string(), "haven_market".to_string()],
        requirements: EncounterRequirements::default(),
        content: EncounterContent {
            description: "An elderly woman sits on a weathered bench, fingers moving through \
                the air as if typing on an invisible keyboard. Her eyes are clouded with \
                cataracts, but her posture is perfect—the disciplined bearing of a trained \
                scribe.

                'I can hear your keystrokes from here,' she says without turning. 'Young \
                hands. Quick hands. But you pause too long between words. You're thinking \
                too much.'

                She finally looks at you, those clouded eyes somehow seeing straight through \
                you.

                'I was like you once. Before the Unwriting. Before we learned that some \
                thoughts are better left untyped.'".to_string(),
            dialogue: Some(vec![
                DialogueLine {
                    speaker: "Old Scribe".to_string(),
                    text: "They called me Vera Quickfingers in the old days. I could type \
                        a hundred words a minute, all true. Now I'm just Vera. The \
                        Corruption took my speed. Left me with only accuracy.".to_string(),
                    reveals: Some("Some scribes survived the Unwriting but lost abilities.".to_string()),
                },
                DialogueLine {
                    speaker: "Old Scribe".to_string(),
                    text: "Want some advice, young one? Don't trust the Archivists. They \
                        know more than they tell. They were watching before the First \
                        Silence, and they're watching now.".to_string(),
                    reveals: Some("The Archivists have been observing since before the Unwriting.".to_string()),
                },
            ]),
            environmental_details: vec![
                "Her fingers never stop moving. She's typing something invisible.".to_string(),
                "A faded guild tattoo marks her wrist—the Scribes' symbol.".to_string(),
                "Her clothes are patched but clean. Someone is taking care of her.".to_string(),
            ],
            typing_challenge: None,
        },
        choices: vec![
            EncounterChoice {
                id: "ask_about_past".to_string(),
                text: "What was it like before the Unwriting?".to_string(),
                requires: None,
                consequence_id: "vera_past".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "ask_about_archivists".to_string(),
                text: "Why shouldn't I trust the Archivists?".to_string(),
                requires: None,
                consequence_id: "vera_archivists".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "offer_help".to_string(),
                text: "Is there anything I can do for you?".to_string(),
                requires: None,
                consequence_id: "vera_help".to_string(),
                typing_required: false,
            },
        ],
        consequences: EncounterConsequences {
            reputation_changes: vec![("Scribes".to_string(), 5)],
            narrative_result: "Vera smiles, and for a moment you can see the master scribe \
                she once was.".to_string(),
            ..Default::default()
        },
        repeatable: true,
        tags: vec!["npc".to_string(), "scribe".to_string(), "lore".to_string()],
    });
    
    // ========================================================================
    // ATHENAEUM ENCOUNTERS - Knowledge-focused, mysterious
    // ========================================================================
    
    encounters.insert("athenaeum_living_book".to_string(), AuthoredEncounter {
        id: "athenaeum_living_book".to_string(),
        title: "The Book That Speaks".to_string(),
        valid_locations: vec!["athenaeum".to_string(), "athenaeum_stacks".to_string()],
        requirements: EncounterRequirements {
            min_chapter: Some(2),
            ..Default::default()
        },
        content: EncounterContent {
            description: "You're browsing the stacks when a book falls from a high shelf \
                and lands open at your feet. The pages are blank—then text begins to appear, \
                letter by letter, as if being typed in real time.

                'FINALLY SOMEONE WHO CAN READ'

                The text writes itself faster.

                'I HAVE BEEN WAITING FORTY-SEVEN YEARS FOR A READER'
                'THE LAST ONE DIED BEFORE FINISHING MY FIRST CHAPTER'
                'WILL YOU BE DIFFERENT?'

                You realize with a chill that this isn't a corrupted book. This is something \
                else entirely. Something that was written so perfectly, so completely, that \
                it became aware.".to_string(),
            dialogue: Some(vec![
                DialogueLine {
                    speaker: "The Living Book".to_string(),
                    text: "I was written before the Unwriting. Back when words had weight \
                        and meaning persisted. The scribes poured so much intention into \
                        me that I... woke up.".to_string(),
                    reveals: Some("Before the Unwriting, text could become sentient.".to_string()),
                },
                DialogueLine {
                    speaker: "The Living Book".to_string(),
                    text: "I know things. Things the Archivists have hidden. Things about \
                        you. About who you were before you forgot.".to_string(),
                    reveals: Some("The player has a forgotten past.".to_string()),
                },
                DialogueLine {
                    speaker: "The Living Book".to_string(),
                    text: "But knowledge has a price. Will you read me? All the way to \
                        the end? Even when the words hurt?".to_string(),
                    reveals: None,
                },
            ]),
            environmental_details: vec![
                "The book's pages are slightly warm to the touch.".to_string(),
                "Text continues to write itself on pages you're not looking at.".to_string(),
                "Sometimes the words rearrange themselves, as if the book is choosing what to show you.".to_string(),
            ],
            typing_challenge: Some(EncounterTypingChallenge {
                prompt_text: "Type the following to begin reading: 'I accept the weight of knowing.'".to_string(),
                difficulty: 3,
                success_narrative: "The book shivers with something like joy. 'At last. Turn to chapter one.'".to_string(),
                failure_narrative: "The book's pages flip shut. 'You hesitate. Come back when you're ready.'".to_string(),
                partial_narrative: Some("The book waits. 'Almost. Try again. Precision matters here.'".to_string()),
            }),
        },
        choices: vec![
            EncounterChoice {
                id: "accept_book".to_string(),
                text: "I'll read you. Show me what you know.".to_string(),
                requires: None,
                consequence_id: "living_book_accepted".to_string(),
                typing_required: true,
            },
            EncounterChoice {
                id: "refuse_book".to_string(),
                text: "I'm not ready for that kind of knowledge.".to_string(),
                requires: None,
                consequence_id: "living_book_refused".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "negotiate_book".to_string(),
                text: "What's in it for you? Books don't usually want to be read.".to_string(),
                requires: None,
                consequence_id: "living_book_negotiate".to_string(),
                typing_required: false,
            },
        ],
        consequences: EncounterConsequences {
            lore_revealed: vec!["player_previous_life".to_string()],
            world_state_changes: vec!["living_book_awakened".to_string()],
            enables_encounters: vec!["living_book_chapter_2".to_string()],
            narrative_result: "The book settles into your hands, warm and patient. It has \
                waited decades for this moment. It can wait a little longer.".to_string(),
            ..Default::default()
        },
        repeatable: false,
        tags: vec!["major".to_string(), "lore".to_string(), "book".to_string(), "player_mystery".to_string()],
    });
    
    // ========================================================================
    // CORRUPTION ZONE ENCOUNTERS - Dangerous, surreal
    // ========================================================================
    
    encounters.insert("corruption_memory_echo".to_string(), AuthoredEncounter {
        id: "corruption_memory_echo".to_string(),
        title: "A Memory Not Your Own".to_string(),
        valid_locations: vec!["corruption_zone".to_string(), "whispering_waste".to_string()],
        requirements: EncounterRequirements {
            min_chapter: Some(3),
            weather: Some(WeatherCondition::CorruptionMist),
            ..Default::default()
        },
        content: EncounterContent {
            description: "The Corruption mist parts, and suddenly you're somewhere else.

                A library. Vast and beautiful, lit by windows that look out onto a city of \
                spires. Books line every wall. The air smells of paper and possibility.

                You're standing at a desk, typing. Your fingers know this keyboard. Your \
                body knows this chair. But these aren't your hands.

                A voice calls from behind you. A name. Your name—but not your name.

                'Love? Are you coming to bed?'

                You turn. A figure stands in the doorway, silhouetted by lamplight. You \
                can't see their face, but your heart—no, someone's heart—aches at the \
                sight of them.

                Then the mist closes in, and you're back in the Waste, alone, with tears \
                streaming down your face.".to_string(),
            dialogue: None,
            environmental_details: vec![
                "The phantom keyboard felt real. Your fingers still remember the keys.".to_string(),
                "The name they called—it echoes in your mind, just out of reach.".to_string(),
                "You know the figure was important. You loved them. You lost them.".to_string(),
            ],
            typing_challenge: Some(EncounterTypingChallenge {
                prompt_text: "Quick! Type the name you almost heard before it fades: '______'".to_string(),
                difficulty: 5,
                success_narrative: "For a moment, you remember. The name. The face. The loss. Then it slips away, leaving only grief.".to_string(),
                failure_narrative: "The name is gone. But the grief remains, settling into your bones like an old wound.".to_string(),
                partial_narrative: Some("Fragments. You catch fragments. A syllable. A feeling. Not enough.".to_string()),
            }),
        },
        choices: vec![
            EncounterChoice {
                id: "embrace_memory".to_string(),
                text: "Try to hold onto the memory, even if it hurts.".to_string(),
                requires: None,
                consequence_id: "memory_embrace".to_string(),
                typing_required: true,
            },
            EncounterChoice {
                id: "reject_memory".to_string(),
                text: "Push the memory away. It's not yours.".to_string(),
                requires: None,
                consequence_id: "memory_reject".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "analyze_memory".to_string(),
                text: "This feels significant. Try to understand what you saw.".to_string(),
                requires: Some("Archivists rank: Initiate".to_string()),
                consequence_id: "memory_analyze".to_string(),
                typing_required: false,
            },
        ],
        consequences: EncounterConsequences {
            lore_revealed: vec!["first_speaker_journal_1".to_string()],
            world_state_changes: vec!["player_memory_fragment_1".to_string()],
            narrative_result: "The Corruption mist carries echoes. Some of those echoes are yours. \
                Or were yours. Or will be yours. Time means little in places like this.".to_string(),
            ..Default::default()
        },
        repeatable: false,
        tags: vec!["player_mystery".to_string(), "memory".to_string(), "emotional".to_string()],
    });
    
    // ========================================================================
    // FACTION-SPECIFIC ENCOUNTERS
    // ========================================================================
    
    encounters.insert("mechanist_breakdown".to_string(), AuthoredEncounter {
        id: "mechanist_breakdown".to_string(),
        title: "A Machine in Distress".to_string(),
        valid_locations: vec!["gearhold".to_string(), "mechanist_workshop".to_string()],
        requirements: EncounterRequirements {
            faction_reputation: Some(("Mechanists".to_string(), -10)),
            ..Default::default()
        },
        content: EncounterContent {
            description: "You find a Mechanist technician sitting in the middle of their \
                workshop, surrounded by dismantled clockwork. They're crying—the ugly, \
                heaving kind that comes from genuine despair.

                'It doesn't work,' they say when they notice you. 'None of it works. \
                We tell ourselves that machines are pure. That gears and springs don't \
                lie. But it's not true.'

                They hold up a small device—a word-processor, mechanical rather than \
                magical. Its keys are stuck in nonsense patterns.

                'Even the machines are corrupted now. Even the things we built to escape \
                the Unwriting. There's nowhere left that words are safe.'".to_string(),
            dialogue: Some(vec![
                DialogueLine {
                    speaker: "Mechanist Technician".to_string(),
                    text: "The elders keep saying we just need better designs. More \
                        precise mechanisms. But I've seen the truth. The Corruption isn't \
                        in the words. It's in meaning itself.".to_string(),
                    reveals: Some("Some Mechanists are losing faith in their doctrine.".to_string()),
                },
                DialogueLine {
                    speaker: "Mechanist Technician".to_string(),
                    text: "What if the Naturalists are right? What if we can't engineer \
                        our way out of this? What if the only answer is to... let it happen?".to_string(),
                    reveals: None,
                },
            ]),
            environmental_details: vec![
                "The dismantled machines show signs of Corruption—gears with too many teeth, springs that coil inward.".to_string(),
                "Plans cover the walls, covered in crossed-out formulas and frustrated annotations.".to_string(),
                "The technician's hands are calloused but steady. They're used to precise work.".to_string(),
            ],
            typing_challenge: None,
        },
        choices: vec![
            EncounterChoice {
                id: "comfort_mechanist".to_string(),
                text: "The Corruption affects everything. You're not wrong to despair.".to_string(),
                requires: None,
                consequence_id: "mechanist_comfort".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "challenge_mechanist".to_string(),
                text: "Giving up won't help. There has to be a solution.".to_string(),
                requires: None,
                consequence_id: "mechanist_challenge".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "help_mechanist".to_string(),
                text: "Show me what you're working on. Maybe fresh eyes will help.".to_string(),
                requires: None,
                consequence_id: "mechanist_help".to_string(),
                typing_required: true,
            },
        ],
        consequences: EncounterConsequences {
            reputation_changes: vec![("Mechanists".to_string(), 15)],
            npc_opinion_changes: vec![("Technician Kaya".to_string(), 20)],
            enables_encounters: vec!["mechanist_doubt_chain".to_string()],
            narrative_result: "The technician looks at you with something between hope and \
                fear. You've seen behind the Mechanist certainty to the doubt underneath.".to_string(),
            ..Default::default()
        },
        repeatable: false,
        tags: vec!["faction".to_string(), "mechanists".to_string(), "doubt".to_string()],
    });
    
    encounters.insert("shadowwriter_offer".to_string(), AuthoredEncounter {
        id: "shadowwriter_offer".to_string(),
        title: "A Whisper in the Dark".to_string(),
        valid_locations: vec!["shadow_quarter".to_string(), "haven_alleys".to_string()],
        requirements: EncounterRequirements {
            time_of_day: Some(TimeOfDay::Night),
            min_chapter: Some(2),
            ..Default::default()
        },
        content: EncounterContent {
            description: "A voice speaks from an alley so dark you can't see who's there. \
                The voice is androgynous, measured, each word carefully chosen.

                'You've been making noise, little typist. Good noise. The kind that \
                makes certain people nervous. That makes you interesting to us.'

                A card flutters out of the darkness and lands at your feet. It's black, \
                with silver text that seems to shift when you try to read it.

                'The Shadow Writers are always looking for talent. Talent that doesn't \
                ask too many questions. Talent that understands that some truths are \
                better left unwritten—but should still be known.'".to_string(),
            dialogue: Some(vec![
                DialogueLine {
                    speaker: "Voice in the Dark".to_string(),
                    text: "We don't want you to do anything illegal. Nothing that would \
                        hurt anyone who doesn't deserve it. We just... collect information. \
                        Important information. Information the factions hide from each other.".to_string(),
                    reveals: Some("The Shadow Writers spy on other factions.".to_string()),
                },
                DialogueLine {
                    speaker: "Voice in the Dark".to_string(),
                    text: "In exchange, we share what we know. And we know a great deal. \
                        About the Unwriting. About the First Speaker. About you.".to_string(),
                    reveals: Some("The Shadow Writers know about the player's past.".to_string()),
                },
            ]),
            environmental_details: vec![
                "The darkness in the alley seems deeper than natural. Unnatural.".to_string(),
                "The card's silver text reads differently each time you look at it.".to_string(),
                "You can't tell if the voice belongs to one person or several.".to_string(),
            ],
            typing_challenge: None,
        },
        choices: vec![
            EncounterChoice {
                id: "accept_shadow".to_string(),
                text: "I'm listening. What do you want me to do?".to_string(),
                requires: None,
                consequence_id: "shadow_accepted".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "refuse_shadow".to_string(),
                text: "I don't work in the dark. Find someone else.".to_string(),
                requires: None,
                consequence_id: "shadow_refused".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "demand_info".to_string(),
                text: "Tell me what you know about me first. Then we'll talk.".to_string(),
                requires: None,
                consequence_id: "shadow_demanded".to_string(),
                typing_required: false,
            },
        ],
        consequences: EncounterConsequences {
            reputation_changes: vec![("ShadowWriters".to_string(), 10)],
            world_state_changes: vec!["shadowwriter_contact".to_string()],
            enables_encounters: vec!["cipher_introduction".to_string()],
            narrative_result: "The darkness shifts. You sense the presence withdrawing, \
                but not entirely. The Shadow Writers are patient. They'll wait for your answer.".to_string(),
            ..Default::default()
        },
        repeatable: false,
        tags: vec!["faction".to_string(), "shadowwriters".to_string(), "offer".to_string()],
    });
    
    // ========================================================================
    // LATE-GAME ENCOUNTERS - Major revelations
    // ========================================================================
    
    encounters.insert("first_archivist_meeting".to_string(), AuthoredEncounter {
        id: "first_archivist_meeting".to_string(),
        title: "The Oldest Word".to_string(),
        valid_locations: vec!["athenaeum_restricted".to_string()],
        requirements: EncounterRequirements {
            min_chapter: Some(4),
            required_lore: Some("player_previous_life".to_string()),
            faction_reputation: Some(("Archivists".to_string(), 50)),
            ..Default::default()
        },
        content: EncounterContent {
            description: "The Restricted Section is silent. Too silent. The kind of silence \
                that listens.

                Then something moves in the darkness between the shelves. Not a person. \
                Not exactly. A shape made of shadow and suggestion, with eyes like the \
                space between letters.

                'You've returned,' it says. The voice is everywhere and nowhere, written \
                more than spoken. 'Again. You always return. Forty-seven times, you've \
                found your way here. Forty-seven times, you've asked the same question.'

                It draws closer. You can see it now—or rather, you can read it. The First \
                Archivist is a word that achieved consciousness. A concept that learned \
                to think.

                'Are you ready for the answer this time? Or will you choose to forget again?'".to_string(),
            dialogue: Some(vec![
                DialogueLine {
                    speaker: "The First Archivist".to_string(),
                    text: "I am what remains of the very first word ever written. \
                        The word that invented meaning. I existed before the Age of Voices, \
                        before even the First Scribe discovered writing.".to_string(),
                    reveals: Some("The First Archivist predates human writing.".to_string()),
                },
                DialogueLine {
                    speaker: "The First Archivist".to_string(),
                    text: "I have watched you for three thousand years, First Speaker. \
                        I watched you create the Unwriting. I watched you die from it. I \
                        watched you be reborn, again and again, each time forgetting what \
                        you did. What you lost. What you became.".to_string(),
                    reveals: Some("The player caused the Unwriting.".to_string()),
                },
                DialogueLine {
                    speaker: "The First Archivist".to_string(),
                    text: "The wound you created cannot be healed by forgetting. It can \
                        only be healed by choosing. You have three paths: end all writing, \
                        restore all writing, or find the Third Grammar. Previous versions \
                        of you have tried the first two. None have attempted the third.".to_string(),
                    reveals: Some("There is a third option.".to_string()),
                },
            ]),
            environmental_details: vec![
                "The First Archivist flickers like candlelight. Sometimes you can read words in its form.".to_string(),
                "The books around you are ancient. Some predate human civilization.".to_string(),
                "Time feels different here. You're not sure how long you've been standing.".to_string(),
            ],
            typing_challenge: Some(EncounterTypingChallenge {
                prompt_text: "Type your true name—the name you had before you forgot.".to_string(),
                difficulty: 5,
                success_narrative: "The name flows through your fingers. For a moment, you are who you were. It hurts. It heals.".to_string(),
                failure_narrative: "You can't remember. The First Archivist sighs—a sound like pages turning. 'Not yet, then.'".to_string(),
                partial_narrative: Some("The name comes in fragments. Half-remembered. Half-denied.".to_string()),
            }),
        },
        choices: vec![
            EncounterChoice {
                id: "ask_third_grammar".to_string(),
                text: "What is the Third Grammar?".to_string(),
                requires: None,
                consequence_id: "archivist_third_grammar".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "ask_spouse".to_string(),
                text: "The one I lost... are they still out there somewhere?".to_string(),
                requires: None,
                consequence_id: "archivist_spouse".to_string(),
                typing_required: false,
            },
            EncounterChoice {
                id: "reject_past".to_string(),
                text: "I'm not that person anymore. I choose to stay who I am now.".to_string(),
                requires: None,
                consequence_id: "archivist_rejected".to_string(),
                typing_required: false,
            },
        ],
        consequences: EncounterConsequences {
            lore_revealed: vec!["tomorrow_text_7".to_string()],
            world_state_changes: vec!["identity_revealed".to_string()],
            enables_encounters: vec!["final_choice".to_string()],
            narrative_result: "The First Archivist watches you with patient, ageless eyes. \
                It has waited millennia. It can wait a little longer. But not forever. \
                The wound is spreading.".to_string(),
            ..Default::default()
        },
        repeatable: false,
        tags: vec!["major".to_string(), "revelation".to_string(), "archivist".to_string(), "player_identity".to_string()],
    });
    
    encounters
}

/// Encounter tracker for a playthrough
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncounterTracker {
    /// Encounters that have been seen
    pub completed_encounters: HashMap<String, bool>,
    /// Choices made in each encounter
    pub choices_made: HashMap<String, String>,
    /// NPCs the player has met
    pub npcs_met: Vec<String>,
    /// Active encounter chains
    pub active_chains: Vec<String>,
}

impl EncounterTracker {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn complete_encounter(&mut self, encounter_id: &str, choice_id: &str) {
        self.completed_encounters.insert(encounter_id.to_string(), true);
        self.choices_made.insert(encounter_id.to_string(), choice_id.to_string());
    }
    
    pub fn has_completed(&self, encounter_id: &str) -> bool {
        *self.completed_encounters.get(encounter_id).unwrap_or(&false)
    }
    
    pub fn get_choice(&self, encounter_id: &str) -> Option<&String> {
        self.choices_made.get(encounter_id)
    }
    
    pub fn meet_npc(&mut self, npc_name: &str) {
        if !self.npcs_met.contains(&npc_name.to_string()) {
            self.npcs_met.push(npc_name.to_string());
        }
    }
}
