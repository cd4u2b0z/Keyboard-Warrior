//! Dialogue Engine - Contextual combat dialogue generation
//!
//! Combat dialogue should feel coherent with the enemy you're fighting.
//! A goblin talks differently than an eldritch horror.
//! Messages respond to the current state of the fight.

use rand::prelude::*;

/// Combat momentum for enemies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatMomentum {
    Fresh,      // Full health, confident
    Bloodied,   // Below 70%, wary
    Desperate,  // Below 30%, panicking
    Dying,      // Below 10%, last stand
}

impl CombatMomentum {
    pub fn from_health_percent(percent: i32) -> Self {
        match percent {
            0..=10 => Self::Dying,
            11..=30 => Self::Desperate,
            31..=70 => Self::Bloodied,
            _ => Self::Fresh,
        }
    }
}

/// Player's momentum in combat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMomentum {
    Dominant,   // High health, fast typing
    Confident,  // Good position
    Struggling, // Taking hits
    Critical,   // Low health
}

impl PlayerMomentum {
    pub fn from_health_and_accuracy(health_percent: i32, accuracy: f32) -> Self {
        if health_percent < 25 {
            return Self::Critical;
        }
        if health_percent < 50 {
            return Self::Struggling;
        }
        if accuracy > 0.95 {
            return Self::Dominant;
        }
        Self::Confident
    }
}

/// Zone/floor context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneContext {
    RuinedKeep,
    DrownedArchives,
    OvergrownSanctum,
    ClockworkDepths,
    VoidBreach,
    Unknown,
}

impl ZoneContext {
    pub fn from_floor(floor: u32) -> Self {
        match floor {
            1..=2 => Self::RuinedKeep,
            3..=4 => Self::DrownedArchives,
            5..=6 => Self::OvergrownSanctum,
            7..=8 => Self::ClockworkDepths,
            9..=10 => Self::VoidBreach,
            _ => Self::Unknown,
        }
    }
}

/// Context for generating dialogue
#[derive(Debug, Clone)]
pub struct DialogueContext {
    pub enemy_name: String,
    pub enemy_theme: String,
    pub enemy_momentum: CombatMomentum,
    pub player_momentum: PlayerMomentum,
    pub zone: ZoneContext,
    pub typing_speed: f32,
    pub accuracy: f32,
}

/// Main dialogue engine
#[derive(Debug, Clone, Default)]
pub struct DialogueEngine {
    rng: ThreadRng,
}

impl DialogueEngine {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
    
    /// Generate a hit message based on context
    pub fn generate_hit_message(&mut self, ctx: &DialogueContext, damage: i32, attack_type: &crate::game::typing_impact::AttackType) -> String {
        let base = self.get_hit_flavor(&ctx.enemy_theme, ctx.enemy_momentum, damage);
        let modifier = self.get_attack_modifier(attack_type, ctx.enemy_momentum);
        format!("{}{}", base, modifier)
    }
    
    /// Generate enemy attack message
    pub fn generate_enemy_attack(&mut self, ctx: &DialogueContext, damage: i32) -> String {
        match ctx.enemy_theme.as_str() {
            "goblin" => self.goblin_attack(ctx.enemy_momentum, damage),
            "undead" => self.undead_attack(ctx.enemy_momentum, damage),
            "spectral" => self.spectral_attack(ctx.enemy_momentum, damage),
            "corrupted" => self.corrupted_attack(ctx.enemy_momentum, damage),
            "mechanical" => self.mechanical_attack(ctx.enemy_momentum, damage),
            "void" => self.void_attack(ctx.enemy_momentum, damage),
            _ => format!("The {} attacks for {} damage!", ctx.enemy_name, damage),
        }
    }
    
    /// Generate death message
    pub fn generate_death_message(&mut self, ctx: &DialogueContext) -> String {
        match ctx.enemy_theme.as_str() {
            "goblin" => self.random_pick(&[
                "The goblin squeals and collapses.".to_string(),
                "With a pathetic whimper, the goblin falls.".to_string(),
                "The goblin crumples, its stolen treasures scattering.".to_string(),
            ]),
            "undead" => self.random_pick(&[
                "The skeleton clatters apart, finally at rest.".to_string(),
                "Ancient bones collapse into dust.".to_string(),
                "The undead falls, its curse finally broken.".to_string(),
            ]),
            "spectral" => self.random_pick(&[
                "The spirit fades with a final, mournful wail.".to_string(),
                "Reality reasserts itself. The phantom is gone.".to_string(),
                "The apparition disperses like morning mist.".to_string(),
            ]),
            "corrupted" => self.random_pick(&[
                "The corruption recedes. What remains is almost peaceful.".to_string(),
                "The twisted form shudders and falls still.".to_string(),
                "Nature, corrupted no more, returns to earth.".to_string(),
            ]),
            "mechanical" => self.random_pick(&[
                "SYSTEM FAILURE. The construct powers down.".to_string(),
                "Gears grind to a halt. Silence returns.".to_string(),
                "The automaton collapses, its purpose ended.".to_string(),
            ]),
            "void" => self.random_pick(&[
                "Reality knits itself back together where the void-touched stood.".to_string(),
                "The darkness recedes, leaving only the memory of wrongness.".to_string(),
                "With a sound like tearing silk reversed, it is unmade.".to_string(),
            ]),
            _ => format!("The {} has been defeated!", ctx.enemy_name),
        }
    }
    
    /// Generate taunt from enemy
    pub fn generate_enemy_taunt(&mut self, ctx: &DialogueContext) -> Option<String> {
        if self.rng.gen::<f32>() > 0.3 { return None; }
        
        Some(match ctx.enemy_theme.as_str() {
            "goblin" => match ctx.enemy_momentum {
                CombatMomentum::Fresh => self.random_pick(&[
                    "Gonna poke you full of holes!".to_string(),
                    "Shinies! Give us the shinies!".to_string(),
                ]),
                CombatMomentum::Bloodied => self.random_pick(&[
                    "Ow! You pay for that!".to_string(),
                    "Not fair! NOT FAIR!".to_string(),
                ]),
                CombatMomentum::Desperate => self.random_pick(&[
                    "No no no! Bad human!".to_string(),
                    "I tells the others! They gets you!".to_string(),
                ]),
                CombatMomentum::Dying => "...mercy?".to_string(),
            },
            "void" => match ctx.enemy_momentum {
                CombatMomentum::Fresh => self.random_pick(&[
                    "W E   S E E   Y O U".to_string(),
                    "Y O U   A R E   A L R E A D Y   E M P T Y".to_string(),
                ]),
                CombatMomentum::Bloodied | CombatMomentum::Desperate => self.random_pick(&[
                    "T H I S   F O R M   I S   N O T H I N G".to_string(),
                    "W E   A R E   E T E R N A L".to_string(),
                ]),
                CombatMomentum::Dying => "W E   W I L L   R E T U R N".to_string(),
            },
            _ => return None,
        })
    }
    
    /// Generate combat intro
    pub fn generate_combat_intro(&mut self, ctx: &DialogueContext) -> String {
        match ctx.enemy_theme.as_str() {
            "goblin" => self.random_pick(&[
                format!("A {} blocks your path, cackling!", ctx.enemy_name),
                format!("The {} leaps from the shadows!", ctx.enemy_name),
            ]),
            "undead" => self.random_pick(&[
                format!("A {} rises from the dust, ancient hatred burning in empty sockets.", ctx.enemy_name),
                format!("The {} shambles forth, bones rattling.", ctx.enemy_name),
            ]),
            "spectral" => self.random_pick(&[
                format!("A {} materializes from the darkness.", ctx.enemy_name),
                format!("The temperature drops. A {} appears.", ctx.enemy_name),
            ]),
            "corrupted" => self.random_pick(&[
                format!("The {} emerges from the overgrowth, twisted and wrong.", ctx.enemy_name),
                format!("Vines part to reveal a {}, pulsing with corruption.", ctx.enemy_name),
            ]),
            "mechanical" => self.random_pick(&[
                format!("INTRUDER DETECTED. A {} activates.", ctx.enemy_name),
                format!("Gears whir to life. A {} bars your way.", ctx.enemy_name),
            ]),
            "void" => self.random_pick(&[
                format!("Reality tears. A {} steps through.", ctx.enemy_name),
                format!("The {} was always here. You just could not see it before.", ctx.enemy_name),
            ]),
            _ => format!("A {} appears!", ctx.enemy_name),
        }
    }
    
    fn get_hit_flavor(&mut self, theme: &str, momentum: CombatMomentum, damage: i32) -> String {
        match theme {
            "goblin" => match momentum {
                CombatMomentum::Fresh => self.random_pick(&[
                    "AIEEE! The goblin clutches the wound.".to_string(),
                    "The goblin yelps in pain!".to_string(),
                ]),
                CombatMomentum::Bloodied => self.random_pick(&[
                    "Ow! Not fair! the goblin whines.".to_string(),
                    "The goblin staggers, looking worried.".to_string(),
                ]),
                CombatMomentum::Desperate | CombatMomentum::Dying => self.random_pick(&[
                    "The goblin whimpers pathetically.".to_string(),
                    "No more! No more!".to_string(),
                ]),
            },
            "undead" => match momentum {
                CombatMomentum::Fresh => self.random_pick(&[
                    "Bones crack under the blow.".to_string(),
                    "The undead feels no pain, but the damage is clear.".to_string(),
                ]),
                _ => self.random_pick(&[
                    "Ancient bones shatter.".to_string(),
                    "The skeleton is falling apart.".to_string(),
                ]),
            },
            "spectral" => self.random_pick(&[
                "The apparition SCREAMS - a sound like tearing silk.".to_string(),
                "Your attack disrupts its form.".to_string(),
                "The ghost flickers violently.".to_string(),
            ]),
            "corrupted" => self.random_pick(&[
                "Sap-like blood oozes from the wound.".to_string(),
                "The corrupted flesh knits wrongly.".to_string(),
                "It does not bleed. It oozes.".to_string(),
            ]),
            "mechanical" => self.random_pick(&[
                "DAMAGE SUSTAINED. Sparks fly.".to_string(),
                "Metal shrieks as gears grind.".to_string(),
                "ERROR: STRUCTURAL INTEGRITY COMPROMISED.".to_string(),
            ]),
            "void" => self.random_pick(&[
                "Reality ripples where you strike.".to_string(),
                "The void-touched recoils from existence.".to_string(),
                "Something that should not be... is hurt.".to_string(),
            ]),
            _ => format!("You deal {} damage!", damage),
        }
    }
    
    fn get_attack_modifier(&mut self, attack_type: &crate::game::typing_impact::AttackType, momentum: CombatMomentum) -> String {
        let base = match attack_type {
            crate::game::typing_impact::AttackType::Precision => "A precise strike!",
            crate::game::typing_impact::AttackType::Flurry => "A rapid flurry of blows!",
            crate::game::typing_impact::AttackType::Deliberate => "A measured, powerful hit.",
            crate::game::typing_impact::AttackType::Frantic => "Wild swings - one connects!",
            crate::game::typing_impact::AttackType::Standard => "",
        };
        
        let momentum_mod = match momentum {
            CombatMomentum::Bloodied => " It is wavering.",
            CombatMomentum::Desperate => " It is faltering!",
            CombatMomentum::Dying => " The killing blow approaches.",
            CombatMomentum::Fresh => "",
        };
        
        format!(" {}{}", base, momentum_mod)
    }
    
    fn goblin_attack(&mut self, momentum: CombatMomentum, damage: i32) -> String {
        match momentum {
            CombatMomentum::Fresh => self.random_pick(&[
                format!("Your shinies! MINE! It slashes at you! {} damage!", damage),
                format!("The goblin stabs wildly! {} damage!", damage),
            ]),
            CombatMomentum::Bloodied => self.random_pick(&[
                format!("The goblin attacks desperately! {} damage!", damage),
                format!("Still gonna getcha! {} damage!", damage),
            ]),
            _ => format!("A feeble attack... but still {} damage.", damage),
        }
    }
    
    fn undead_attack(&mut self, momentum: CombatMomentum, damage: i32) -> String {
        match momentum {
            CombatMomentum::Fresh => self.random_pick(&[
                format!("Bony claws rake across you! {} damage!", damage),
                format!("The undead strikes with ancient malice! {} damage!", damage),
            ]),
            _ => format!("It claws at you weakly. {} damage.", damage),
        }
    }
    
    fn spectral_attack(&mut self, _momentum: CombatMomentum, damage: i32) -> String {
        self.random_pick(&[
            format!("A ghostly touch chills your soul! {} damage!", damage),
            format!("The phantom passes THROUGH you! {} damage!", damage),
            format!("Spectral energy lashes out! {} damage!", damage),
        ])
    }
    
    fn corrupted_attack(&mut self, _momentum: CombatMomentum, damage: i32) -> String {
        self.random_pick(&[
            format!("Thorned vines lash at you! {} damage!", damage),
            format!("Corrupted spores assault you! {} damage!", damage),
            format!("The twisted thing strikes! {} damage!", damage),
        ])
    }
    
    fn mechanical_attack(&mut self, _momentum: CombatMomentum, damage: i32) -> String {
        self.random_pick(&[
            format!("EXECUTING COMBAT PROTOCOL. {} damage!", damage),
            format!("Gears whir. Blades extend. {} damage!", damage),
            format!("The construct attacks with mechanical precision! {} damage!", damage),
        ])
    }
    
    fn void_attack(&mut self, _momentum: CombatMomentum, damage: i32) -> String {
        self.random_pick(&[
            format!("Reality BENDS around you! {} damage!", damage),
            format!("The void reaches into you! {} damage!", damage),
            format!("Y O U   F E E L   E M P T Y. {} damage!", damage),
        ])
    }
    
    fn random_pick<T: Clone>(&mut self, options: &[T]) -> T {
        options.choose(&mut self.rng).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_momentum_calculation() {
        assert_eq!(CombatMomentum::from_health_percent(100), CombatMomentum::Fresh);
        assert_eq!(CombatMomentum::from_health_percent(50), CombatMomentum::Bloodied);
        assert_eq!(CombatMomentum::from_health_percent(20), CombatMomentum::Desperate);
        assert_eq!(CombatMomentum::from_health_percent(5), CombatMomentum::Dying);
    }
    
    #[test]
    fn test_dialogue_generation() {
        let mut engine = DialogueEngine::new();
        let ctx = DialogueContext {
            enemy_name: "Goblin Scout".to_string(),
            enemy_theme: "goblin".to_string(),
            enemy_momentum: CombatMomentum::Fresh,
            player_momentum: PlayerMomentum::Confident,
            zone: ZoneContext::RuinedKeep,
            typing_speed: 5.0,
            accuracy: 0.95,
        };
        
        let intro = engine.generate_combat_intro(&ctx);
        assert!(!intro.is_empty());
        
        let death = engine.generate_death_message(&ctx);
        assert!(!death.is_empty());
    }
}
