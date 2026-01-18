//! Lore-Integrated Word System - Words that tell the story
//!
//! Every word typed during combat connects to the world's lore,
//! the current zone, the enemy faced, and the overarching narrative.
//! Typing becomes an act of narrative discovery.

use rand::seq::SliceRandom;
use rand::Rng;

/// Zone-specific word pools that immerse players in each area's atmosphere
pub struct LoreWords;

impl LoreWords {
    // =========================================
    // SHATTERED HALLS - The fallen kingdom
    // =========================================
    pub fn shattered_halls_words() -> Vec<&'static str> {
        vec![
            // The fallen kingdom
            "throne", "crown", "knight", "oath", "honor",
            "fallen", "ruin", "dust", "echo", "ghost",
            "banner", "sigil", "crest", "blade", "shield",
            "castle", "hall", "chamber", "passage", "gate",
            // The king's tragedy
            "valdris", "kingdom", "loyalty", "betrayal", "hubris",
            "archon", "malachar", "sundering", "sacrifice", "memory",
            // Atmosphere
            "silence", "shadow", "whisper", "darkness", "cold",
            "ancient", "broken", "shattered", "hollow", "empty",
        ]
    }
    
    pub fn shattered_halls_sentences() -> Vec<&'static str> {
        vec![
            "The throne sits empty, but the oaths still bind.",
            "Sir Aldric gave his life defending these halls.",
            "The banners of Valdris hang in tatters.",
            "Once, laughter echoed here. Now, only silence.",
            "The king walked toward the light, and never returned.",
            "Loyalty beyond death. Honor beyond memory.",
            "These stones remember what the living have forgotten.",
            "The Hollow Knights still patrol their eternal watch.",
            "In the dust, you find a sigil of the royal guard.",
            "The Sundering took everything, but it could not take our oaths.",
        ]
    }
    
    // =========================================
    // SUNKEN ARCHIVES - Drowned knowledge
    // =========================================
    pub fn sunken_archives_words() -> Vec<&'static str> {
        vec![
            // Knowledge and secrets
            "scroll", "tome", "codex", "grimoire", "scripture",
            "wisdom", "truth", "secret", "forbidden", "ancient",
            "scholar", "scribe", "keeper", "archivist", "sage",
            // The drowned library
            "sunken", "drowned", "water", "depths", "flooded",
            "ink", "pages", "binding", "spine", "text",
            // Malachar's research
            "ritual", "ascension", "veil", "breach", "stones",
            "elder", "power", "knowledge", "obsession", "madness",
            // Discovery
            "fragment", "remnant", "preserved", "lost", "found",
        ]
    }
    
    pub fn sunken_archives_sentences() -> Vec<&'static str> {
        vec![
            "Malachar studied here before his fall from grace.",
            "These texts survived the flood. Their secrets endure.",
            "The scholars drowned protecting their books.",
            "Some knowledge is dangerous. Some truths should stay buried.",
            "The ink runs, but the meaning remains.",
            "In the deepest archives, forbidden texts still whisper.",
            "The Elder Stones were first described in these halls.",
            "Water cannot wash away what is written in the soul.",
            "The archivists gave their lives to preserve the truth.",
            "Even now, the drowned scholars guard their wisdom.",
        ]
    }
    
    // =========================================
    // BLIGHTED GARDENS - Nature corrupted
    // =========================================
    pub fn blighted_gardens_words() -> Vec<&'static str> {
        vec![
            // Nature corrupted
            "blight", "rot", "decay", "wither", "corrupt",
            "thorn", "vine", "root", "bark", "branch",
            "poison", "spore", "fungus", "mold", "growth",
            // Once beautiful
            "garden", "bloom", "flower", "petal", "seed",
            "verdant", "lush", "green", "life", "nature",
            // The spread
            "spread", "consume", "infect", "twist", "change",
            "mutation", "aberration", "transformation", "horror",
            // Hope
            "purify", "cleanse", "heal", "restore", "save",
        ]
    }
    
    pub fn blighted_gardens_sentences() -> Vec<&'static str> {
        vec![
            "The royal gardens were the jewel of Valdris. Now they weep poison.",
            "Even the trees scream in these corrupted groves.",
            "The blight spreads with each passing day.",
            "Somewhere beneath the corruption, life still struggles.",
            "The gardeners became the first victims of the spreading rot.",
            "Nature itself has been turned against the natural order.",
            "The flowers still bloom, but their beauty is a lie.",
            "Touch nothing. Trust nothing. The blight is patient.",
            "The roots dig deep. The corruption goes deeper.",
            "Perhaps some part of this place can still be saved.",
        ]
    }
    
    // =========================================
    // CLOCKWORK DEPTHS - Ancient mechanisms
    // =========================================
    pub fn clockwork_depths_words() -> Vec<&'static str> {
        vec![
            // Mechanical
            "gear", "cog", "wheel", "spring", "lever",
            "steam", "brass", "copper", "iron", "steel",
            "mechanism", "automaton", "construct", "machine",
            // Purpose
            "guardian", "sentinel", "warden", "protector", "keeper",
            "purpose", "function", "directive", "protocol", "order",
            // Ancient tech
            "ancient", "forgotten", "dormant", "awakened", "eternal",
            "precision", "calibrate", "maintain", "preserve", "endure",
            // The depths
            "depths", "below", "beneath", "underground", "buried",
        ]
    }
    
    pub fn clockwork_depths_sentences() -> Vec<&'static str> {
        vec![
            "The ancients built machines that outlasted their makers.",
            "These guardians know only one command: protect.",
            "Gears turn in patterns older than the kingdom itself.",
            "The clockwork sentinels do not know their masters are gone.",
            "Steam hisses through pipes that have run for millennia.",
            "Somewhere in these depths, the Binding Stones still hold.",
            "The mechanisms serve a purpose we no longer understand.",
            "Even machines can dream. Their dreams are patient.",
            "The old masters built well. Perhaps too well.",
            "Time has no meaning to things that do not age.",
        ]
    }
    
    // =========================================
    // VOID'S EDGE - Where reality thins
    // =========================================
    pub fn voids_edge_words() -> Vec<&'static str> {
        vec![
            // The Void
            "void", "nothing", "emptiness", "null", "absence",
            "darkness", "shadow", "black", "endless", "infinite",
            // Reality breaking
            "tear", "rift", "breach", "crack", "fracture",
            "reality", "existence", "meaning", "truth", "being",
            // The Sundering
            "sundering", "archon", "malachar", "ascension", "fall",
            "hubris", "price", "sacrifice", "power", "cost",
            // The end
            "end", "final", "last", "ultimate", "absolute",
            "herald", "harbinger", "omen", "prophecy", "doom",
        ]
    }
    
    pub fn voids_edge_sentences() -> Vec<&'static str> {
        vec![
            "Here, at the edge of everything, meaning starts to fade.",
            "The Void Herald speaks with voices that were never born.",
            "Malachar sought godhood. He found something else entirely.",
            "Beyond this point, the rules of reality no longer apply.",
            "The Sundering tore a wound in the world that will not heal.",
            "Some doors, once opened, cannot be closed.",
            "The Archon's ambition doomed us all. Can you succeed where he failed?",
            "In the Void, there is no past, no future. Only the eternal now.",
            "The Elder Stones pulse with power that predates creation.",
            "What walks in the spaces between worlds? You are about to find out.",
        ]
    }
    
    // =========================================
    // THE BREACH - Final confrontation
    // =========================================
    pub fn the_breach_words() -> Vec<&'static str> {
        vec![
            // Ultimate power
            "seal", "bind", "close", "restore", "save",
            "hero", "champion", "chosen", "destiny", "fate",
            // The cosmic
            "cosmos", "creation", "existence", "reality", "world",
            "god", "divine", "mortal", "eternal", "infinite",
            // Victory or defeat
            "victory", "triumph", "salvation", "redemption", "hope",
            "sacrifice", "courage", "determination", "will", "spirit",
            // The final words
            "end", "beginning", "cycle", "renewal", "continuation",
        ]
    }
    
    pub fn the_breach_sentences() -> Vec<&'static str> {
        vec![
            "This is where the Archon fell. This is where you must not.",
            "The Elder Stones recognize a worthy soul. Are you ready?",
            "Forty-seven years of suffering end here, one way or another.",
            "The Void Herald guards the breach. It will not yield easily.",
            "You carry the hopes of everyone who fell before you.",
            "Seal the breach. End the Sundering. Save what remains.",
            "The world watches, even if it does not know your name.",
            "Every word you type is a blow against oblivion.",
            "This is not just a battle. It is a reclamation of meaning.",
            "The Archon sought to become a god. You seek only to save your home.",
        ]
    }
    
    // =========================================
    // ENEMY-SPECIFIC PHRASES
    // =========================================
    
    /// Words for goblin-type enemies (greedy, crude)
    pub fn goblin_words() -> Vec<&'static str> {
        vec![
            "shiny", "mine", "steal", "grab", "hoard",
            "sneak", "hide", "ambush", "trap", "trick",
            "gold", "loot", "treasure", "coin", "gem",
        ]
    }
    
    /// Words for undead enemies (hollow, eternal)
    pub fn undead_words() -> Vec<&'static str> {
        vec![
            "hollow", "empty", "eternal", "bound", "cursed",
            "duty", "oath", "service", "guard", "watch",
            "death", "grave", "tomb", "rest", "peace",
            "memory", "forgotten", "lost", "wandering", "endless",
        ]
    }
    
    /// Words for spectral enemies (ethereal, mysterious)
    pub fn spectral_words() -> Vec<&'static str> {
        vec![
            "wisp", "glow", "fade", "shimmer", "flicker",
            "spirit", "soul", "essence", "echo", "remnant",
            "whisper", "wail", "moan", "cry", "sigh",
            "memory", "regret", "sorrow", "longing", "loss",
        ]
    }
    
    /// Words for corrupted enemies (twisted, wrong)
    pub fn corrupted_words() -> Vec<&'static str> {
        vec![
            "twist", "warp", "corrupt", "taint", "blight",
            "wrong", "broken", "shattered", "ruined", "lost",
            "pain", "agony", "torment", "suffering", "anguish",
            "cure", "save", "heal", "purify", "restore",
        ]
    }
    
    /// Words for mechanical enemies (precise, cold)
    pub fn mechanical_words() -> Vec<&'static str> {
        vec![
            "gear", "cog", "spring", "mechanism", "function",
            "directive", "protocol", "execute", "process", "command",
            "target", "threat", "eliminate", "protect", "guard",
            "ancient", "eternal", "patient", "waiting", "watching",
        ]
    }
    
    /// Words for void enemies (cosmic horror)
    pub fn void_words() -> Vec<&'static str> {
        vec![
            "void", "nothing", "empty", "absent", "null",
            "beyond", "between", "outside", "other", "wrong",
            "meaning", "purpose", "existence", "reality", "truth",
            "end", "unmaking", "erasure", "oblivion", "silence",
        ]
    }
    
    // =========================================
    // BOSS-SPECIFIC CONTENT
    // =========================================
    
    pub fn hollow_knight_sentences() -> Vec<&'static str> {
        vec![
            "I am the last defender of a kingdom that no longer exists.",
            "My oath binds me still, even in death.",
            "The king I served walked into the light and never returned.",
            "You seek to pass? Then prove your worth through combat.",
            "I have guarded these halls for forty-seven years.",
            "Perhaps you are the one the prophecies spoke of.",
            "My blade remembers every battle. It will remember you.",
            "Honor demands that I test you. Do not disappoint me.",
        ]
    }
    
    pub fn void_herald_sentences() -> Vec<&'static str> {
        vec![
            "I speak with the voice of endings. Listen, and despair.",
            "The Sundering was not a disaster. It was an awakening.",
            "Your words are meaningless noise in the face of eternity.",
            "The Archon understood, in the end. You will too.",
            "I am what waits in the spaces between thoughts.",
            "Every reality ends. Yours simply ends sooner.",
            "The Elder Stones cannot save you. Nothing can.",
            "Type your final words, hero. Make them count.",
        ]
    }
    
    // =========================================
    // NARRATIVE PROGRESSION PHRASES
    // =========================================
    
    /// Early game - establishing the world
    pub fn early_narrative() -> Vec<&'static str> {
        vec![
            "The kingdom of Valdris fell forty-seven years ago.",
            "The Sundering changed everything.",
            "You are not the first to venture into these depths.",
            "The Blight spreads with each passing season.",
            "Somewhere below, the breach still bleeds darkness.",
        ]
    }
    
    /// Mid game - revealing the truth
    pub fn mid_narrative() -> Vec<&'static str> {
        vec![
            "Malachar was not a villain. He was trying to save us all.",
            "The Elder Stones hold power beyond mortal comprehension.",
            "The Archon's ritual failed. Or did it succeed too well?",
            "The factions war while the true enemy grows stronger.",
            "You begin to understand what you must do.",
        ]
    }
    
    /// Late game - final revelation
    pub fn late_narrative() -> Vec<&'static str> {
        vec![
            "The breach can be sealed. But the cost may be everything.",
            "You carry the hopes of a dying world.",
            "The Void Herald guards the way. It must be overcome.",
            "The Elder Stones pulse with ancient recognition.",
            "This is the moment everything has been building toward.",
        ]
    }
    
    // =========================================
    // UTILITY FUNCTIONS
    // =========================================
    
    /// Get words appropriate for the current floor zone
    pub fn get_zone_words(floor: u32) -> Vec<&'static str> {
        match floor {
            1..=2 => Self::shattered_halls_words(),
            3..=4 => Self::sunken_archives_words(),
            5..=6 => Self::blighted_gardens_words(),
            7..=8 => Self::clockwork_depths_words(),
            9..=10 => Self::voids_edge_words(),
            _ => Self::the_breach_words(),
        }
    }
    
    /// Get sentences appropriate for the current floor zone
    pub fn get_zone_sentences(floor: u32) -> Vec<&'static str> {
        match floor {
            1..=2 => Self::shattered_halls_sentences(),
            3..=4 => Self::sunken_archives_sentences(),
            5..=6 => Self::blighted_gardens_sentences(),
            7..=8 => Self::clockwork_depths_sentences(),
            9..=10 => Self::voids_edge_sentences(),
            _ => Self::the_breach_sentences(),
        }
    }
    
    /// Get words based on enemy type (from typing_theme)
    pub fn get_enemy_words(typing_theme: &str) -> Vec<&'static str> {
        match typing_theme {
            "fantasy" => Self::undead_words(),
            "dark" => Self::spectral_words(),
            "arcane" => Self::sunken_archives_words(),
            "nature" => Self::corrupted_words(),
            "technology" => Self::mechanical_words(),
            _ => Self::shattered_halls_words(),
        }
    }
    
    /// Get narrative sentences based on progression
    pub fn get_narrative_sentences(floor: u32) -> Vec<&'static str> {
        match floor {
            1..=3 => Self::early_narrative(),
            4..=7 => Self::mid_narrative(),
            _ => Self::late_narrative(),
        }
    }
    
    /// Get a random word from the appropriate pool
    pub fn random_word(floor: u32, enemy_theme: Option<&str>) -> String {
        let mut rng = rand::thread_rng();
        
        // Mix zone words with enemy-specific words
        let mut pool = Self::get_zone_words(floor);
        
        if let Some(theme) = enemy_theme {
            pool.extend(Self::get_enemy_words(theme));
        }
        
        pool.choose(&mut rng)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "honor".to_string())
    }
    
    /// Get a random sentence from the appropriate pool
    pub fn random_sentence(floor: u32, is_boss: bool, boss_name: Option<&str>) -> String {
        let mut rng = rand::thread_rng();
        
        // Boss-specific sentences take priority
        if is_boss {
            if let Some(name) = boss_name {
                let boss_sentences: Vec<&str> = match name {
                    n if n.contains("Hollow Knight") => Self::hollow_knight_sentences(),
                    n if n.contains("Void Herald") => Self::void_herald_sentences(),
                    _ => Self::get_zone_sentences(floor),
                };
                return boss_sentences.choose(&mut rng)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "Face your destiny.".to_string());
            }
        }
        
        // Mix zone sentences with narrative sentences
        let mut pool = Self::get_zone_sentences(floor);
        pool.extend(Self::get_narrative_sentences(floor));
        
        pool.choose(&mut rng)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "The battle continues.".to_string())
    }
}
