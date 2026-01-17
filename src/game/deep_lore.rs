//! Deep Lore System - The mythology, history, and secrets of the world
//!
//! This module establishes the foundational truth of the universe.
//! All other systems reference this as the source of coherent worldbuilding.
//!
//! Design Principles (from studying masters of the craft):
//! - Tolkien: Languages shape culture; history feels lived-in
//! - Ursula K. Le Guin: Magic has costs; systems have rules
//! - Gene Wolfe: Unreliable narrators; truth revealed through contradiction
//! - Dark Souls/Elden Ring: Environmental storytelling; fragmented lore
//! - Planescape Torment: Belief shapes reality; words have power
//! - Disco Elysium: Internal voices; ideology as character

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// THE COSMOLOGY - What is true about this universe
// ============================================================================

/// The fundamental truth of the world: reality is made of language.
/// Before there was matter, there was the Word.
/// The Unwriting threatens to unmake creation itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cosmology {
    /// The three ages of the world
    pub ages: WorldAges,
    /// The nature of the Corruption/Unwriting
    pub corruption_truth: CorruptionTruth,
    /// The original sin that started everything
    pub the_first_silence: FirstSilence,
    /// What the world was like before
    pub before_memory: BeforeMemory,
}

impl Default for Cosmology {
    fn default() -> Self {
        Self::canonical()
    }
}

impl Cosmology {
    /// The canonical, true cosmology (what actually happened)
    pub fn canonical() -> Self {
        Self {
            ages: WorldAges::canonical(),
            corruption_truth: CorruptionTruth::canonical(),
            the_first_silence: FirstSilence::canonical(),
            before_memory: BeforeMemory::canonical(),
        }
    }
}

/// The three ages of the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldAges {
    pub age_of_voices: AgeOfVoices,
    pub age_of_writing: AgeOfWriting,
    pub age_of_unwriting: AgeOfUnwriting,
}

impl WorldAges {
    pub fn canonical() -> Self {
        Self {
            age_of_voices: AgeOfVoices {
                description: "Before writing, when words existed only as breath. \
                    The First Speakers shaped reality through speech alone. \
                    Nothing was permanent. Every truth could be forgotten. \
                    The world was mutable, dreamlike, contradictory.".to_string(),
                key_event: "The Speaking of the First Names gave form to chaos.".to_string(),
                duration: "Unknown - time itself was unwritten".to_string(),
                ended_by: "The First Scribe discovered how to make words permanent.".to_string(),
            },
            age_of_writing: AgeOfWriting {
                description: "Words made permanent. Reality solidified. \
                    The great libraries arose. Knowledge accumulated. \
                    But with permanence came inflexibility. \
                    And with fixed meaning came the possibility of corruption.".to_string(),
                key_event: "The Founding of the First Library at Logos Prime.".to_string(),
                duration: "Three thousand years of recorded history.".to_string(),
                ended_by: "The First Silence - when someone tried to unwrite death itself.".to_string(),
                great_works: vec![
                    "The Eternal Codex - all natural laws in written form".to_string(),
                    "The Name Registry - every living thing recorded".to_string(),
                    "The Tomorrow Texts - prophecies written to ensure futures".to_string(),
                ],
            },
            age_of_unwriting: AgeOfUnwriting {
                description: "The current age. The Corruption spreads. \
                    Words lose meaning. Texts unravel. Reality bleeds. \
                    The factions war over how to survive - or exploit - the collapse.".to_string(),
                key_event: "The Great Unwriting began at Logos Prime.".to_string(),
                duration: "Forty-seven years since the First Silence.".to_string(),
                current_state: "Accelerating decay. Haven is one of the last stable zones.".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfVoices {
    pub description: String,
    pub key_event: String,
    pub duration: String,
    pub ended_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfWriting {
    pub description: String,
    pub key_event: String,
    pub duration: String,
    pub ended_by: String,
    pub great_works: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfUnwriting {
    pub description: String,
    pub key_event: String,
    pub duration: String,
    pub current_state: String,
}

/// The true nature of the Corruption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionTruth {
    /// What the factions believe (different theories)
    pub faction_theories: HashMap<String, String>,
    /// What is actually true (revealed gradually)
    pub actual_nature: String,
    /// The terrible secret
    pub hidden_truth: HiddenTruth,
}

impl CorruptionTruth {
    pub fn canonical() -> Self {
        let mut theories = HashMap::new();
        
        theories.insert("Scribes".to_string(), 
            "The Corruption is divine punishment for straying from proper form. \
             Only perfect adherence to the sacred texts can reverse it.".to_string());
        
        theories.insert("Mechanists".to_string(),
            "The Corruption is entropy—a natural process of linguistic decay. \
             It can be outpaced through pure speed and efficiency.".to_string());
        
        theories.insert("Naturalists".to_string(),
            "The Corruption is the world rejecting forced, unnatural writing. \
             Return to organic expression and reality will heal.".to_string());
        
        theories.insert("ShadowWriters".to_string(),
            "The Corruption was deliberately created as a weapon. \
             Someone—or something—is using it intentionally.".to_string());
        
        theories.insert("Archivists".to_string(),
            "The Corruption is information loss on a cosmic scale. \
             It cannot be stopped, only documented.".to_string());
        
        Self {
            faction_theories: theories,
            actual_nature: "The Corruption is grief made manifest. \
                When the First Speaker tried to unwrite death—to remove the word \
                and thus the concept from reality—they succeeded partially. \
                Death was wounded but not killed. What bleeds from that wound \
                is the Unwriting: reality's immune response to a violation \
                of its fundamental grammar.".to_string(),
            hidden_truth: HiddenTruth {
                level_1: "The Corruption can be temporarily halted by perfect typing—\
                    flawless language reinforces reality.".to_string(),
                level_2: "The original unwriter is still alive, kept immortal by their \
                    own broken spell, endlessly trying to finish what they started.".to_string(),
                level_3: "The player is connected to the First Speaker. Their amnesia \
                    is not accidental—they chose to forget.".to_string(),
                final_truth: "The player IS the First Speaker, reborn to either complete \
                    the Unwriting or finally accept what they tried to unmake.".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenTruth {
    /// Revealed in Chapter 2
    pub level_1: String,
    /// Revealed in Chapter 4
    pub level_2: String,
    /// Revealed in Chapter 5
    pub level_3: String,
    /// The final revelation
    pub final_truth: String,
}

/// The event that ended the Age of Writing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstSilence {
    pub what_happened: String,
    pub who_caused_it: String,
    pub why: String,
    pub immediate_consequences: Vec<String>,
    pub long_term_consequences: Vec<String>,
}

impl FirstSilence {
    pub fn canonical() -> Self {
        Self {
            what_happened: "At the height of the Age of Writing, the greatest \
                scribe—called only the First Speaker in surviving texts—lost someone \
                they loved. In their grief, they attempted the impossible: to unwrite \
                death itself. To remove the word, and thus the concept, from reality. \
                They partially succeeded. The resulting paradox tore a hole in the \
                fabric of meaning.".to_string(),
            who_caused_it: "The First Speaker. Their true name has been lost—or \
                deliberately erased. Some texts refer to them as 'The One Who Would Not Let Go.'".to_string(),
            why: "Love. Grief. The unbearable weight of loss. They could not accept \
                that words—which could create worlds—could not save one person.".to_string(),
            immediate_consequences: vec![
                "Logos Prime, the First Library, collapsed into a wound in reality.".to_string(),
                "The Eternal Codex shattered. Natural laws became suggestions.".to_string(),
                "The Name Registry corrupted. Identities began to blur.".to_string(),
                "The Corruption began spreading from the epicenter.".to_string(),
            ],
            long_term_consequences: vec![
                "Haven and other stable zones formed around uncorrupted texts.".to_string(),
                "The factions arose with competing theories and survival strategies.".to_string(),
                "Typing became both weapon and shield—the only way to reinforce reality.".to_string(),
                "The First Speaker vanished. Some say died. Some say worse.".to_string(),
            ],
        }
    }
}

/// What the world was like before the Unwriting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeMemory {
    pub the_old_world: String,
    pub what_was_lost: Vec<String>,
    pub what_remains: Vec<String>,
    pub places_that_remember: Vec<String>,
}

impl BeforeMemory {
    pub fn canonical() -> Self {
        Self {
            the_old_world: "A world where words were trusted. Where you could name \
                something and know it would stay named. Where books held stable truth \
                and language connected rather than threatened. The sky had a proper \
                color, gravity had a reliable direction, and death—though feared—was \
                at least consistent.".to_string(),
            what_was_lost: vec![
                "The ability to trust written records completely.".to_string(),
                "Consistent natural laws outside stable zones.".to_string(),
                "Long-distance communication (texts corrupt in transit).".to_string(),
                "The certainty of identity (names can blur, be stolen, forgotten).".to_string(),
                "Simple mortality (some deaths no longer 'take').".to_string(),
            ],
            what_remains: vec![
                "Haven and other protected settlements.".to_string(),
                "Fragments of the great libraries, fiercely guarded.".to_string(),
                "The practice of protective typing—reality maintenance through keystrokes.".to_string(),
                "Oral traditions among the Naturalists, less corruptible than text.".to_string(),
                "The faction wars, fought with words as weapons.".to_string(),
            ],
            places_that_remember: vec![
                "The Athenaeum - where books still hold stable meaning.".to_string(),
                "Haven's Central Square - built around the Last Functional Terminal.".to_string(),
                "The Sacred Grove - where words grow as living things.".to_string(),
                "The Restricted Section - sealed texts too dangerous to read.".to_string(),
            ],
        }
    }
}

// ============================================================================
// FACTION HISTORIES - How each group arose and what they truly want
// ============================================================================

/// Complete history of a faction, including hidden agendas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionHistory {
    pub faction_name: String,
    pub founding_story: String,
    pub founder: HistoricalFigure,
    pub original_purpose: String,
    pub how_they_changed: String,
    pub current_leadership: String,
    pub public_agenda: String,
    pub hidden_agenda: String,
    pub internal_conflicts: Vec<String>,
    pub key_artifacts: Vec<Artifact>,
    pub relationship_to_corruption: String,
    pub what_they_know: Vec<String>,
    pub what_they_hide: Vec<String>,
}

/// A historical figure important to the lore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalFigure {
    pub name: String,
    pub title: String,
    pub era: String,
    pub legacy: String,
    pub dark_secret: Option<String>,
    pub connection_to_player: Option<String>,
}

/// An artifact with history and power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub description: String,
    pub origin_story: String,
    pub powers: Vec<String>,
    pub current_location: ArtifactLocation,
    pub who_wants_it: Vec<String>,
    pub hidden_truth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactLocation {
    Known(String),
    Rumored(String),
    Lost,
    HeldByPlayer,
    Destroyed,
    Corrupted,
}

/// Build complete faction histories
pub fn build_faction_histories() -> HashMap<String, FactionHistory> {
    let mut histories = HashMap::new();
    
    histories.insert("Scribes".to_string(), FactionHistory {
        faction_name: "The Scribes of the Eternal Word".to_string(),
        founding_story: "When the First Library fell, a group of monks who had been \
            copying texts in a distant monastery felt their sacred works shudder. \
            Sister Verity, the eldest among them, declared that they must preserve \
            what remained through perfect replication. 'If we type true, reality \
            holds true,' she wrote—and discovered it was literally correct.".to_string(),
        founder: HistoricalFigure {
            name: "Sister Verity".to_string(),
            title: "The Unshaken Hand".to_string(),
            era: "Early Unwriting, Year 1".to_string(),
            legacy: "Discovered that perfect typing reinforces reality. Established \
                the first stable zone through continuous transcription.".to_string(),
            dark_secret: Some("Verity was the First Speaker's apprentice. She knew \
                what they were attempting and said nothing.".to_string()),
            connection_to_player: Some("Her teachings form the basis of the player's \
                initial typing training. Her ghost may appear in dreams.".to_string()),
        },
        original_purpose: "Preserve all texts. Maintain reality through perfect copying.".to_string(),
        how_they_changed: "As their power grew, preservation became control. They began \
            deciding which texts deserved saving—and which should be 'forgotten.'".to_string(),
        current_leadership: "The Voice of the Eternal Word, an anonymous figure who \
            communicates only through perfectly-typed edicts.".to_string(),
        public_agenda: "Maintain the stable zones. Preserve uncorrupted texts. Train \
            typists in the sacred forms.".to_string(),
        hidden_agenda: "Recover the fragments of the Eternal Codex. Use them to rewrite \
            reality according to their doctrine. Become the sole arbiters of truth.".to_string(),
        internal_conflicts: vec![
            "Reformists believe knowledge should be shared; Orthodoxy hoards it.".to_string(),
            "Some younger Scribes question whether 'perfect' typing is truly sacred.".to_string(),
            "The Voice has not been seen in person for seven years. Is anyone truly there?".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Original Manuscript".to_string(),
                description: "The first text ever written—predates even the First Library.".to_string(),
                origin_story: "Written by the First Scribe when they discovered how to \
                    make words permanent. Contains the 'grammar of reality.'".to_string(),
                powers: vec![
                    "Text written on its pages becomes absolutely true.".to_string(),
                    "Can undo any corruption within visual range.".to_string(),
                    "The writer's intent becomes reality—but costs their memories.".to_string(),
                ],
                current_location: ArtifactLocation::Rumored("Said to be in the deepest \
                    vault of the Scribes' sanctuary, but some whisper it was lost.".to_string()),
                who_wants_it: vec![
                    "The Scribes (officially have it, actually desperate to find it)".to_string(),
                    "The Mechanists (would use it to write 'optimal' reality)".to_string(),
                    "The First Speaker (needs it to complete the Unwriting)".to_string(),
                ],
                hidden_truth: Some("The manuscript is the player's lost journal. They were \
                    the First Scribe, reborn.".to_string()),
            },
        ],
        relationship_to_corruption: "View it as punishment for imperfection. Believe \
            perfect typing can halt and eventually reverse it.".to_string(),
        what_they_know: vec![
            "The Corruption can be locally halted through sustained perfect typing.".to_string(),
            "Sister Verity knew the First Speaker personally.".to_string(),
            "Certain texts are too dangerous to preserve—but they preserve them anyway.".to_string(),
        ],
        what_they_hide: vec![
            "The Original Manuscript is missing.".to_string(),
            "The Voice has not spoken in months. The edicts are being forged.".to_string(),
            "They have a fragment of the First Speaker's final writing.".to_string(),
        ],
    });
    
    histories.insert("Mechanists".to_string(), FactionHistory {
        faction_name: "The Mechanist Collective".to_string(),
        founding_story: "In the chaos following the First Silence, a typing instructor \
            named Marcus Venn observed that his fastest students survived corruption \
            events while slower ones did not. He theorized that speed itself was protective—\
            that by typing faster than reality could decay, one could outrun the Unwriting.".to_string(),
        founder: HistoricalFigure {
            name: "Marcus Venn".to_string(),
            title: "The Velocity Prophet".to_string(),
            era: "Early Unwriting, Year 3".to_string(),
            legacy: "Developed the theory of 'Velocity Immunity'—that sufficiently fast \
                typing creates a stable bubble. Founded the first speed-training academies.".to_string(),
            dark_secret: Some("Venn's obsession with speed came from watching his daughter \
                die slowly to corruption. He types endlessly to this day, afraid to stop.".to_string()),
            connection_to_player: None,
        },
        original_purpose: "Develop typing speed as a survival mechanism. Train others \
            to type fast enough to survive.".to_string(),
        how_they_changed: "Speed became ideology. Efficiency became morality. They began \
            to view slow typists as not just vulnerable but inferior—obstacles to \
            collective survival.".to_string(),
        current_leadership: "The Prime Algorithm—a governing council that makes all \
            decisions through typing speed competitions. The fastest finger rules.".to_string(),
        public_agenda: "Achieve speeds sufficient to outpace the Corruption entirely. \
            Train the world in efficiency. Eliminate 'lag' in all its forms.".to_string(),
        hidden_agenda: "They're building something in the Velocity Citadel—a machine \
            that types automatically, infinitely fast. They call it the Perpetual \
            Engine. They believe it will type reality back into existence.".to_string(),
        internal_conflicts: vec![
            "The 'Burnouts'—those who pushed too hard and lost fine motor control.".to_string(),
            "Debate over whether augmentation (mechanical finger aids) is legitimate.".to_string(),
            "Venn still lives, ancient and typing constantly, but disagrees with current doctrine.".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Perpetual Engine".to_string(),
                description: "A massive typing machine designed to type infinitely fast.".to_string(),
                origin_story: "Decades of Mechanist engineering, incorporating corrupted \
                    but controlled texts to power its movements.".to_string(),
                powers: vec![
                    "Can stabilize large areas through sustained output.".to_string(),
                    "Types faster than any human could achieve.".to_string(),
                    "Feeds on corrupted text as fuel.".to_string(),
                ],
                current_location: ArtifactLocation::Known("The heart of the Velocity Citadel, \
                    still incomplete.".to_string()),
                who_wants_it: vec![
                    "The Mechanists (building it)".to_string(),
                    "The Scribes (want to destroy it as heretical)".to_string(),
                    "The ShadowWriters (believe it will accelerate the Corruption)".to_string(),
                ],
                hidden_truth: Some("The Engine cannot distinguish between writing and \
                    unwriting. If completed, it might type reality out of existence entirely.".to_string()),
            },
        ],
        relationship_to_corruption: "View it as a speed problem. Type faster than decay, \
            and you win.".to_string(),
        what_they_know: vec![
            "Speed does create temporary stability fields.".to_string(),
            "The Corruption has patterns—it's not entirely random.".to_string(),
            "Certain corrupted texts contain encoded information.".to_string(),
        ],
        what_they_hide: vec![
            "The Engine's test runs have caused local reality 'stutters.'".to_string(),
            "Marcus Venn opposes the Engine but is kept isolated.".to_string(),
            "Several high-speed typists have started typing words that don't exist.".to_string(),
        ],
    });
    
    histories.insert("Naturalists".to_string(), FactionHistory {
        faction_name: "The Naturalist Circle".to_string(),
        founding_story: "While others typed furiously to hold reality together, a poet \
            named Willow noticed that the Sacred Grove—where she had written verses among \
            the trees—remained uncorrupted. The words there weren't typed; they were grown. \
            She theorized that organic, unforced expression was inherently stable.".to_string(),
        founder: HistoricalFigure {
            name: "Willow".to_string(),
            title: "The First Voice Since Silence".to_string(),
            era: "Early Unwriting, Year 7".to_string(),
            legacy: "Rediscovered spoken word traditions. Proved that oral recitation \
                can stabilize areas without typing. Founded the Grove as a sanctuary.".to_string(),
            dark_secret: Some("Willow can hear the Corruption speak. It told her about \
                the Grove. She has never told anyone what else it said.".to_string()),
            connection_to_player: Some("The player may dream of Willow's voice. She seems \
                to recognize them, though they've never met.".to_string()),
        },
        original_purpose: "Preserve oral traditions. Find alternatives to typing-based \
            stability. Live in harmony with reality rather than forcing it.".to_string(),
        how_they_changed: "Their rejection of 'forced' typing evolved into suspicion \
            of all technology. Some extremists now refuse to type at all, which makes \
            them vulnerable outside the Grove.".to_string(),
        current_leadership: "The Circle of Speakers—elders who memorize and recite the \
            stabilizing verses. No single leader; decisions made by consensus.".to_string(),
        public_agenda: "Teach natural typing flow. Preserve oral traditions. Expand the \
            Grove's protective influence through song and story.".to_string(),
        hidden_agenda: "Willow believes the Corruption is not disease but evolution. \
            She thinks reality is transforming, not dying, and the Naturalists alone \
            will survive the change because they don't fight it.".to_string(),
        internal_conflicts: vec![
            "Purists refuse all typing; Pragmatists accept 'organic' typing.".to_string(),
            "Young members leave for other factions, frustrated by passivity.".to_string(),
            "Some elders have begun speaking in tongues—is this enlightenment or infection?".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Songlines".to_string(),
                description: "A network of verbal pathways through corrupted territory.".to_string(),
                origin_story: "Willow and her followers mapped safe routes by singing. \
                    The songs themselves became the paths.".to_string(),
                powers: vec![
                    "Allow safe passage through corrupted zones if sung correctly.".to_string(),
                    "Reveal hidden locations that exist only when named aloud.".to_string(),
                    "Can temporarily 'heal' minor corruption through recitation.".to_string(),
                ],
                current_location: ArtifactLocation::Known("The Songlines exist as living \
                    knowledge in the Circle of Speakers' memories.".to_string()),
                who_wants_it: vec![
                    "The Archivists (desperately want to write them down, which would kill them)".to_string(),
                    "The ShadowWriters (need them to reach Logos Prime)".to_string(),
                ],
                hidden_truth: Some("The Songlines were not discovered but given. Something \
                    in the Corruption taught Willow how to sing them.".to_string()),
            },
        ],
        relationship_to_corruption: "View it as natural process—perhaps painful but not \
            evil. Believe resistance causes more harm than acceptance.".to_string(),
        what_they_know: vec![
            "Spoken words are harder to corrupt than written ones.".to_string(),
            "The Corruption responds to emotion. Calm minds are safer.".to_string(),
            "Something communicates through the Corruption. Willow calls it 'the Voice.'".to_string(),
        ],
        what_they_hide: vec![
            "Willow regularly 'converses' with the Corruption.".to_string(),
            "Several elders have been 'transformed' and are hidden in the deep Grove.".to_string(),
            "The Songlines lead to Logos Prime. Willow knows the way.".to_string(),
        ],
    });
    
    histories.insert("ShadowWriters".to_string(), FactionHistory {
        faction_name: "The Shadow Writers".to_string(),
        founding_story: "When the First Library fell, most assumed Logos Prime was destroyed. \
            But a group of librarians escaped through hidden passages, carrying forbidden \
            texts—books the First Speaker had ordered destroyed before their final act. \
            These survivors became the Shadow Writers, keepers of dangerous knowledge.".to_string(),
        founder: HistoricalFigure {
            name: "Cipher".to_string(),
            title: "The Name No One Remembers".to_string(),
            era: "The First Silence, Year 0".to_string(),
            legacy: "Led the escape from Logos Prime. Preserved the Forbidden Library. \
                Developed encryption techniques that resist corruption.".to_string(),
            dark_secret: Some("Cipher was there when the First Speaker attempted the \
                Unwriting. They could have stopped it but chose to watch.".to_string()),
            connection_to_player: Some("Cipher left encoded messages for the player \
                throughout the world. They knew the player would return.".to_string()),
        },
        original_purpose: "Preserve the forbidden texts. Guard against those who would \
            repeat the First Speaker's mistake. Fight the Corruption directly.".to_string(),
        how_they_changed: "Their secrecy became paranoia. Their guardianship became control. \
            Now they assassinate those who learn too much as readily as they protect knowledge.".to_string(),
        current_leadership: "The Cipher Council—seven anonymous members who communicate \
            only through encrypted messages. No one knows if the original Cipher still lives.".to_string(),
        public_agenda: "Fight the Corruption. Protect the innocent. Preserve dangerous \
            knowledge safely.".to_string(),
        hidden_agenda: "They're searching for the First Speaker. Not to stop them—to \
            finish what they started. The Cipher Council believes controlled Unwriting \
            is better than chaotic decay. They want to end reality cleanly.".to_string(),
        internal_conflicts: vec![
            "Protectors want to save people; Executors want to control information.".to_string(),
            "Some members discover they've assassinated innocents and defect.".to_string(),
            "Rumors that the original Cipher is actually the First Speaker in hiding.".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Forbidden Library".to_string(),
                description: "A hidden collection of texts too dangerous to exist.".to_string(),
                origin_story: "Rescued from Logos Prime. Contains books on unwriting, \
                    reality manipulation, and the true history of the First Speaker.".to_string(),
                powers: vec![
                    "Contains knowledge to temporarily halt or redirect corruption.".to_string(),
                    "Includes the First Speaker's personal journals.".to_string(),
                    "Holds a partial copy of the Unwriting Equation.".to_string(),
                ],
                current_location: ArtifactLocation::Rumored("Somewhere in the Shadow Quarter, \
                    location known only to the Cipher Council.".to_string()),
                who_wants_it: vec![
                    "Everyone. These texts could end or save the world.".to_string(),
                ],
                hidden_truth: Some("The Library is not hidden—it's encoded. The entire \
                    Shadow Quarter IS the library, with buildings as pages.".to_string()),
            },
        ],
        relationship_to_corruption: "View it as a weapon. Someone aimed it. They intend \
            to find the trigger and either disarm or redirect it.".to_string(),
        what_they_know: vec![
            "The First Speaker survived the First Silence.".to_string(),
            "The Corruption follows patterns based on the Unwriting Equation.".to_string(),
            "There is a way to Logos Prime. The path exists but is deadly.".to_string(),
        ],
        what_they_hide: vec![
            "They have a fragment of the Unwriting Equation and have used it.".to_string(),
            "Cipher may be the First Speaker—or their closest friend, or their murderer.".to_string(),
            "They know the player's true identity but are watching to see what they choose.".to_string(),
        ],
    });
    
    histories.insert("Archivists".to_string(), FactionHistory {
        faction_name: "The Archive Keepers".to_string(),
        founding_story: "The Athenaeum predates the First Library. Its founders understood \
            that all libraries eventually fall, so they built one that exists partially \
            outside reality—a pocket dimension of pure information. When Logos Prime fell, \
            the Athenaeum absorbed its refugees and its overflow of collapsing texts.".to_string(),
        founder: HistoricalFigure {
            name: "The First Archivist".to_string(),
            title: "The One Who Remains".to_string(),
            era: "Before the Age of Writing".to_string(),
            legacy: "Built the Athenaeum as a 'memory of reality.' It records everything \
                that exists—and everything that ceases to exist.".to_string(),
            dark_secret: Some("The First Archivist was not human. They were the first \
                word ever written, given consciousness by the act of preservation.".to_string()),
            connection_to_player: Some("The player can find the First Archivist in the \
                deepest level of the Athenaeum. They have been waiting.".to_string()),
        },
        original_purpose: "Record everything. Take no sides. Preserve all knowledge \
            regardless of danger or morality.".to_string(),
        how_they_changed: "They haven't. That's the problem. They record the Corruption \
            spreading. They record people dying. They do not intervene. Their neutrality \
            has become its own kind of evil.".to_string(),
        current_leadership: "No formal leadership. The Athenaeum itself seems to direct \
            the Archivists through subtle suggestions and reorganizing shelves.".to_string(),
        public_agenda: "Document everything. Assist researchers regardless of faction. \
            Maintain the Athenaeum as neutral ground.".to_string(),
        hidden_agenda: "The Archivists believe the world is ending and cannot be saved. \
            They're building the Athenaeum into a complete backup of reality, so that \
            when everything else is unwritten, meaning will survive in one place.".to_string(),
        internal_conflicts: vec![
            "Younger Archivists want to intervene; elders forbid it.".to_string(),
            "Some texts in the Athenaeum have started editing themselves.".to_string(),
            "The deepest levels are sealed. No one remembers why.".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Index of Everything".to_string(),
                description: "A catalog of all that exists, existed, or will exist.".to_string(),
                origin_story: "Created by the First Archivist. Updated automatically. \
                    Reading an entry makes the reader aware of the subject's location.".to_string(),
                powers: vec![
                    "Locate anyone or anything in reality.".to_string(),
                    "Know the history of anything by reading its entry.".to_string(),
                    "See when entries begin to fade—predicting corruption.".to_string(),
                ],
                current_location: ArtifactLocation::Known("The heart of the Athenaeum. \
                    Only senior Archivists may consult it.".to_string()),
                who_wants_it: vec![
                    "The ShadowWriters (to find the First Speaker)".to_string(),
                    "The Scribes (to find the Original Manuscript)".to_string(),
                    "The player (to find their true identity)".to_string(),
                ],
                hidden_truth: Some("The Index has an entry for the player—but it's encrypted \
                    in a cipher no one recognizes. Except possibly Cipher.".to_string()),
            },
        ],
        relationship_to_corruption: "View it as data. Record its spread. Study its patterns. \
            Do not fight it; fighting creates bias in the records.".to_string(),
        what_they_know: vec![
            "Everything. They have records of the First Speaker, the First Silence, all of it.".to_string(),
            "The Corruption is slowing. This terrifies them because they don't know why.".to_string(),
            "The player has visited the Athenaeum before. Their previous entry was erased.".to_string(),
        ],
        what_they_hide: vec![
            "They have a complete record of the First Speaker's identity and actions.".to_string(),
            "The Athenaeum is dying. The Corruption is seeping into its pocket dimension.".to_string(),
            "The First Archivist is not dead but sleeping in the deepest vault.".to_string(),
        ],
    });
    
    histories
}

// ============================================================================
// THE PLAYER'S TRUE IDENTITY - The central mystery
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMystery {
    /// Clues to be discovered in each chapter
    pub clues_by_chapter: HashMap<u32, Vec<Clue>>,
    /// The truth about who the player is
    pub the_truth: PlayerTruth,
    /// Possible endings based on player choice
    pub possible_endings: Vec<Ending>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clue {
    pub id: String,
    pub description: String,
    pub how_found: String,
    pub what_it_suggests: String,
    pub who_knows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTruth {
    pub who_they_were: String,
    pub what_they_did: String,
    pub why_they_forgot: String,
    pub what_they_must_choose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ending {
    pub name: String,
    pub requirements: Vec<String>,
    pub description: String,
    pub consequences: String,
}

pub fn build_player_mystery() -> PlayerMystery {
    let mut clues = HashMap::new();
    
    // Chapter 1 clues - hints at unusual nature
    clues.insert(1, vec![
        Clue {
            id: "typing_instinct".to_string(),
            description: "You type with impossible familiarity, as if your fingers remember \
                what your mind does not.".to_string(),
            how_found: "Tutorial/early combat".to_string(),
            what_it_suggests: "You were a skilled typist before losing your memory.".to_string(),
            who_knows: vec!["Trainer Beck suspects".to_string()],
        },
        Clue {
            id: "corruption_affinity".to_string(),
            description: "The Corruption... hesitates around you. Just for a moment.".to_string(),
            how_found: "First corruption encounter".to_string(),
            what_it_suggests: "You have some connection to the Corruption.".to_string(),
            who_knows: vec!["Willow senses this".to_string()],
        },
    ]);
    
    // Chapter 2 clues - connections to the past
    clues.insert(2, vec![
        Clue {
            id: "cipher_messages".to_string(),
            description: "You find encrypted messages hidden throughout the world. \
                You can read them without trying.".to_string(),
            how_found: "Exploration".to_string(),
            what_it_suggests: "Someone left messages specifically for you.".to_string(),
            who_knows: vec!["Cipher (obviously)".to_string()],
        },
        Clue {
            id: "dreams_of_library".to_string(),
            description: "You dream of a vast library, burning. You're running through \
                corridors filled with screaming books. You're carrying something precious.".to_string(),
            how_found: "Sleep events".to_string(),
            what_it_suggests: "You were at Logos Prime when it fell.".to_string(),
            who_knows: vec!["The Archivists have recorded your dreams".to_string()],
        },
    ]);
    
    // Chapter 3 clues - identity narrowing
    clues.insert(3, vec![
        Clue {
            id: "verity_recognition".to_string(),
            description: "Sister Verity's ghost appears to you. 'I'm sorry,' she says. \
                'I should have stopped you.'".to_string(),
            how_found: "Scribe questline".to_string(),
            what_it_suggests: "You knew Verity. You did something she regrets.".to_string(),
            who_knows: vec!["The Scribes' secret histories".to_string()],
        },
        Clue {
            id: "original_handwriting".to_string(),
            description: "You find ancient texts written in your handwriting.".to_string(),
            how_found: "Athenaeum deep exploration".to_string(),
            what_it_suggests: "You are far older than you appear.".to_string(),
            who_knows: vec!["The First Archivist".to_string()],
        },
    ]);
    
    // Chapter 4 clues - confronting the truth
    clues.insert(4, vec![
        Clue {
            id: "cipher_confrontation".to_string(),
            description: "Cipher reveals they watched you die—and watched you return. \
                'How many times will you try this?' they ask.".to_string(),
            how_found: "Shadow Writer questline".to_string(),
            what_it_suggests: "You have died before. Perhaps many times.".to_string(),
            who_knows: vec!["Cipher knows everything".to_string()],
        },
        Clue {
            id: "the_name".to_string(),
            description: "You find your entry in the Index of Everything. It lists many names. \
                All of them are yours. The first name is the First Speaker.".to_string(),
            how_found: "Archivist questline".to_string(),
            what_it_suggests: "You are the First Speaker.".to_string(),
            who_knows: vec!["All faction leaders, by now".to_string()],
        },
    ]);
    
    // Chapter 5 clue - acceptance
    clues.insert(5, vec![
        Clue {
            id: "memory_return".to_string(),
            description: "Your memories return. You were the greatest scribe who ever lived. \
                When your beloved died, you tried to unwrite death itself. You failed. \
                You've been reborn countless times, trying to fix your mistake—or finish it.".to_string(),
            how_found: "Approaching Logos Prime".to_string(),
            what_it_suggests: "The truth.".to_string(),
            who_knows: vec!["Everyone".to_string()],
        },
    ]);
    
    PlayerMystery {
        clues_by_chapter: clues,
        the_truth: PlayerTruth {
            who_they_were: "The First Speaker—the greatest typist in history, whose grief \
                destroyed the world.".to_string(),
            what_they_did: "Attempted to unwrite death. Partially succeeded. The resulting \
                paradox created the Corruption.".to_string(),
            why_they_forgot: "You chose to forget. Each rebirth, you erase your own memory, \
                hoping a fresh start will help you find a different answer. It never has.".to_string(),
            what_they_must_choose: "Complete the Unwriting (end existence). Reverse it \
                (restore death, accept loss). Or find a third path no one has imagined.".to_string(),
        },
        possible_endings: vec![
            Ending {
                name: "The Final Silence".to_string(),
                requirements: vec!["Complete the Unwriting Equation".to_string()],
                description: "You finish what you started. Reality is unwritten. Peace at last.".to_string(),
                consequences: "Everything ends. But perhaps that's not nothing—perhaps \
                    unwritten means potential, means the chance to begin again better.".to_string(),
            },
            Ending {
                name: "The First Word".to_string(),
                requirements: vec!["Destroy the Unwriting Equation".to_string(), 
                                   "Accept your beloved's death".to_string()],
                description: "You reverse your mistake. Death returns fully. The Corruption \
                    halts. You finally let go.".to_string(),
                consequences: "The world heals, slowly. You die for real this time. \
                    But you die knowing you chose to let others live.".to_string(),
            },
            Ending {
                name: "The Third Grammar".to_string(),
                requirements: vec!["Unite all factions".to_string(),
                                   "Find the hidden variable in the Equation".to_string()],
                description: "You discover that unwriting death was impossible not because \
                    it can't be done, but because death was never a word—it was a silence. \
                    You learn to write silence. You create a new grammar of reality.".to_string(),
                consequences: "Reality transforms. Death becomes optional. Existence becomes \
                    choice. You become the first god of a new kind of world.".to_string(),
            },
        ],
    }
}
