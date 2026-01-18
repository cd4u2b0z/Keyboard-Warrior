//! Pacing Controller - The breath between battles
//!
//! Not every moment needs to be combat. This system:
//! - Tracks tension levels
//! - Inserts breather beats after intense sequences
//! - Adds atmospheric moments during exploration
//! - Controls the emotional rhythm of the game
//!
//! Design: Tension must rise and fall to feel meaningful

use serde::{Deserialize, Serialize};
use rand::prelude::*;

/// Controls narrative pacing throughout the run
#[derive(Debug, Clone)]
pub struct PacingController {
    /// Current tension level (0-100)
    pub tension: i32,
    /// Combats since last rest/breather
    pub combats_since_rest: i32,
    /// Current pacing phase
    pub phase: PacingPhase,
    /// Pending beats to display
    pub pending_beats: Vec<PacingBeat>,
    /// Random generator
    rng: ThreadRng,
}

/// Current pacing phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacingPhase {
    /// Slow discovery, low stakes
    Exploration,
    /// Building toward conflict
    RisingTension,
    /// Active combat/danger
    Confrontation,
    /// Recovery after conflict
    Resolution,
    /// Story/character moment
    Interlude,
}

impl PacingPhase {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Exploration => "exploration",
            Self::RisingTension => "rising tension",
            Self::Confrontation => "confrontation",
            Self::Resolution => "resolution",
            Self::Interlude => "interlude",
        }
    }
}

/// A pacing beat - a moment of narrative breath
#[derive(Debug, Clone)]
pub enum PacingBeat {
    /// Atmospheric description (auto-advance)
    Atmosphere {
        text: String,
        duration_ms: u32,
    },
    /// Environmental detail the player can examine
    Environmental {
        text: String,
        examine_prompt: Option<String>,
    },
    /// Internal thought from player character
    InternalThought {
        text: String,
    },
    /// Distant sound or hint of danger
    OminousHint {
        text: String,
    },
    /// Memory fragment (player's hidden past)
    MemoryFlash {
        text: String,
        lore_key: Option<String>,
    },
    /// Brief NPC moment
    NPCGlimpse {
        text: String,
    },
}

impl Default for PacingController {
    fn default() -> Self {
        Self::new()
    }
}

impl PacingController {
    pub fn new() -> Self {
        Self {
            tension: 0,
            combats_since_rest: 0,
            phase: PacingPhase::Exploration,
            pending_beats: Vec::new(),
            rng: thread_rng(),
        }
    }
    
    /// Called when combat starts
    pub fn on_combat_start(&mut self, is_boss: bool) {
        self.phase = PacingPhase::Confrontation;
        self.tension += if is_boss { 30 } else { 15 };
        self.tension = self.tension.min(100);
    }
    
    /// Called when combat ends
    pub fn on_combat_end(&mut self, victory: bool, was_boss: bool) {
        self.combats_since_rest += 1;
        
        if victory {
            // Tension decreases on victory
            self.tension -= if was_boss { 20 } else { 10 };
            self.tension = self.tension.max(0);
            
            // After multiple combats, insert a breather
            if self.combats_since_rest >= 3 {
                self.queue_breather();
            }
        } else {
            // Defeat increases tension
            self.tension += 10;
        }
        
        self.update_phase();
    }
    
    /// Called when entering a new room
    pub fn on_room_enter(&mut self, floor: u32, room_type: &str) {
        // Sometimes add atmospheric moment
        if self.phase == PacingPhase::Exploration && self.rng.gen::<f32>() < 0.3 {
            self.queue_atmospheric(floor);
        }
        
        // Tension naturally rises as we go deeper
        if room_type == "combat" {
            self.tension += 5;
        }
    }
    
    /// Called when player rests
    pub fn on_rest(&mut self) {
        self.combats_since_rest = 0;
        self.tension = (self.tension - 30).max(0);
        self.phase = PacingPhase::Resolution;
        
        // Add a rest beat
        self.pending_beats.push(PacingBeat::InternalThought {
            text: "You rest. The silence is almost peaceful. Almost.".into(),
        });
    }
    
    /// Called when entering shop
    pub fn on_shop_enter(&mut self) {
        self.phase = PacingPhase::Interlude;
        self.tension = (self.tension - 10).max(0);
    }
    
    /// Update pacing phase based on tension
    fn update_phase(&mut self) {
        self.phase = match self.tension {
            0..=20 => PacingPhase::Exploration,
            21..=50 => PacingPhase::RisingTension,
            51..=80 => PacingPhase::Confrontation,
            _ => PacingPhase::Confrontation,
        };
    }
    
    /// Queue a breather beat after intense combat
    fn queue_breather(&mut self) {
        let beats = [
            PacingBeat::InternalThought {
                text: "You pause. Let your breathing slow. The silence after battle is deafening.".into(),
            },
            PacingBeat::InternalThought {
                text: "Your hands are shaking. When did they start?".into(),
            },
            PacingBeat::InternalThought {
                text: "How many more? The question surfaces unbidden.".into(),
            },
            PacingBeat::Environmental {
                text: "Dust settles. The echoes of combat fade into memory.".into(),
                examine_prompt: None,
            },
        ];
        
        if let Some(beat) = beats.choose(&mut self.rng) {
            self.pending_beats.push(beat.clone());
        }
        
        self.combats_since_rest = 0;
        self.phase = PacingPhase::Resolution;
    }
    
    /// Queue an atmospheric beat for exploration
    fn queue_atmospheric(&mut self, floor: u32) {
        let beat = match floor {
            1..=2 => {
                let options = [
                    PacingBeat::Atmosphere {
                        text: "Dust motes drift through shafts of pale light.".into(),
                        duration_ms: 2000,
                    },
                    PacingBeat::Environmental {
                        text: "Faded banners hang from the walls. The heraldry is unfamiliar.".into(),
                        examine_prompt: Some("A crown split by a sword. House Valdris, perhaps?".into()),
                    },
                    PacingBeat::OminousHint {
                        text: "Something scrapes against stone in the distance.".into(),
                    },
                ];
                options.choose(&mut self.rng).cloned()
            }
            3..=4 => {
                let options = [
                    PacingBeat::Atmosphere {
                        text: "Water drips somewhere in the darkness. The Archives remember.".into(),
                        duration_ms: 2500,
                    },
                    PacingBeat::Environmental {
                        text: "Waterlogged books line the shelves. Knowledge, drowning.".into(),
                        examine_prompt: Some("Most are ruined. But here and there, a legible page...".into()),
                    },
                    PacingBeat::MemoryFlash {
                        text: "You've been here before. Haven't you? The feeling fades.".into(),
                        lore_key: Some("archives_memory".into()),
                    },
                ];
                options.choose(&mut self.rng).cloned()
            }
            5..=6 => {
                let options = [
                    PacingBeat::Atmosphere {
                        text: "The air is thick with the smell of rot and strange blooms.".into(),
                        duration_ms: 2000,
                    },
                    PacingBeat::Environmental {
                        text: "Flowers grow from the cracks. Beautiful. Wrong.".into(),
                        examine_prompt: Some("Their petals pulse with a faint, sickly light.".into()),
                    },
                    PacingBeat::OminousHint {
                        text: "Something moves in the undergrowth. Not hostile. Not yet.".into(),
                    },
                ];
                options.choose(&mut self.rng).cloned()
            }
            7..=8 => {
                let options = [
                    PacingBeat::Atmosphere {
                        text: "Gears tick in the walls. The Depths are alive, in their way.".into(),
                        duration_ms: 2000,
                    },
                    PacingBeat::Environmental {
                        text: "A construct lies broken against the wall. Its eyes still glow, faintly.".into(),
                        examine_prompt: Some("MAINTENANCE OVERDUE. 4,327 YEARS. PLEASE WAIT.".into()),
                    },
                    PacingBeat::MemoryFlash {
                        text: "You built this. No. That's impossible. Isn't it?".into(),
                        lore_key: Some("clockwork_memory".into()),
                    },
                ];
                options.choose(&mut self.rng).cloned()
            }
            9..=10 => {
                let options = [
                    PacingBeat::Atmosphere {
                        text: "Reality wavers at the edges. Don't look too closely.".into(),
                        duration_ms: 3000,
                    },
                    PacingBeat::OminousHint {
                        text: "The void watches. It always watches.".into(),
                    },
                    PacingBeat::MemoryFlash {
                        text: "You remember darkness. Endless. Hungry. Home.".into(),
                        lore_key: Some("void_memory".into()),
                    },
                ];
                options.choose(&mut self.rng).cloned()
            }
            _ => {
                let options = [
                    PacingBeat::Atmosphere {
                        text: "This is where it happened. The Sundering. You can feel it.".into(),
                        duration_ms: 3000,
                    },
                    PacingBeat::MemoryFlash {
                        text: "Malachar stood here. No. YOU stood here. The truth approaches.".into(),
                        lore_key: Some("breach_memory".into()),
                    },
                ];
                options.choose(&mut self.rng).cloned()
            }
        };
        
        if let Some(b) = beat {
            self.pending_beats.push(b);
        }
    }
    
    /// Get next pending beat (if any)
    pub fn pop_beat(&mut self) -> Option<PacingBeat> {
        self.pending_beats.pop()
    }
    
    /// Check if there are pending beats
    pub fn has_pending(&self) -> bool {
        !self.pending_beats.is_empty()
    }
    
    /// Get current tension level (0-100)
    pub fn get_tension(&self) -> i32 {
        self.tension
    }
    
    /// Get current phase
    pub fn get_phase(&self) -> PacingPhase {
        self.phase
    }
    
    /// Force a specific beat (for scripted moments)
    pub fn queue_beat(&mut self, beat: PacingBeat) {
        self.pending_beats.push(beat);
    }
    
    /// Reset for new run
    pub fn reset(&mut self) {
        self.tension = 0;
        self.combats_since_rest = 0;
        self.phase = PacingPhase::Exploration;
        self.pending_beats.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tension_management() {
        let mut pacing = PacingController::new();
        
        // Combat increases tension
        pacing.on_combat_start(false);
        assert!(pacing.tension > 0);
        
        // Victory decreases tension
        let before = pacing.tension;
        pacing.on_combat_end(true, false);
        assert!(pacing.tension < before);
    }
    
    #[test]
    fn test_breather_generation() {
        let mut pacing = PacingController::new();
        
        // Simulate 3 combats
        for _ in 0..3 {
            pacing.on_combat_start(false);
            pacing.on_combat_end(true, false);
        }
        
        // Should have a breather beat
        assert!(pacing.has_pending() || pacing.combats_since_rest == 0);
    }
}
