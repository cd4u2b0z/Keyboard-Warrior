//! Enemy Visual State - Progressive damage visualization
//!
//! Enemies visually degrade as they take damage:
//! - Wound markers accumulate at hit locations
//! - Posture shifts from confident to dying
//! - Blood/damage effects appear
//!
//! Design: A dying enemy should LOOK dying

use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Visual damage state for enemies
#[derive(Debug, Clone)]
pub struct EnemyVisualState {
    /// Base ASCII art (pristine)
    pub base_art: Vec<String>,
    /// Damage overlay data
    pub damage_overlays: DamageOverlays,
    /// Current animation frame
    pub current_frame: usize,
    /// Current posture
    pub posture: EnemyPosture,
    /// Last rendered art (cached)
    cached_render: Option<Vec<String>>,
}

/// Enemy posture based on damage taken
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyPosture {
    /// Full HP, aggressive stance
    Confident,
    /// 75% HP, defensive
    Wary,
    /// 50% HP, off-balance
    Staggered,
    /// 25% HP, limping/hunched
    Wounded,
    /// <10% HP, barely standing
    Dying,
}

impl EnemyPosture {
    pub fn from_health_pct(pct: f32) -> Self {
        match pct {
            p if p > 0.75 => Self::Confident,
            p if p > 0.50 => Self::Wary,
            p if p > 0.25 => Self::Staggered,
            p if p > 0.10 => Self::Wounded,
            _ => Self::Dying,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::Confident => "confident",
            Self::Wary => "wary",
            Self::Staggered => "staggered",
            Self::Wounded => "wounded",
            Self::Dying => "dying",
        }
    }
}

/// All damage overlay data
#[derive(Debug, Clone, Default)]
pub struct DamageOverlays {
    /// Wound markers
    pub wounds: Vec<WoundMarker>,
    /// Blood/effect particles
    pub particles: Vec<DamageParticle>,
    /// Total damage severity (for posture calculation)
    pub total_severity: u32,
}

/// A wound marker on the enemy
#[derive(Debug, Clone)]
pub struct WoundMarker {
    /// Position in ASCII art (row, col)
    pub position: (usize, usize),
    /// Severity of wound
    pub severity: WoundSeverity,
    /// Character to display
    pub char_override: char,
}

/// Wound severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WoundSeverity {
    Scratch,    // Minor visual change
    Cut,        // Visible line
    Gash,       // Major wound
    Critical,   // Devastating mark
}

impl WoundSeverity {
    pub fn from_damage_pct(pct: f32) -> Self {
        match pct {
            p if p > 0.25 => Self::Critical,
            p if p > 0.15 => Self::Gash,
            p if p > 0.08 => Self::Cut,
            _ => Self::Scratch,
        }
    }
    
    pub fn char(&self) -> char {
        match self {
            Self::Critical => '╳',
            Self::Gash => '/',
            Self::Cut => '─',
            Self::Scratch => '·',
        }
    }
    
    pub fn value(&self) -> u32 {
        match self {
            Self::Critical => 4,
            Self::Gash => 3,
            Self::Cut => 2,
            Self::Scratch => 1,
        }
    }
}

/// A particle effect (blood, sparks, etc.)
#[derive(Debug, Clone)]
pub struct DamageParticle {
    pub position: (usize, usize),
    pub char: char,
}

/// Hit location for damage
#[derive(Debug, Clone, Copy)]
pub enum HitLocation {
    Head,
    Torso,
    LeftArm,
    RightArm,
    Legs,
    Center,
    Random,
}

impl Default for EnemyVisualState {
    fn default() -> Self {
        Self::new(vec![
            "   ?   ".to_string(),
            "  ???  ".to_string(),
            "   ?   ".to_string(),
        ])
    }
}

impl EnemyVisualState {
    pub fn new(base_art: Vec<String>) -> Self {
        Self {
            base_art,
            damage_overlays: DamageOverlays::default(),
            current_frame: 0,
            posture: EnemyPosture::Confident,
            cached_render: None,
        }
    }
    
    /// Create from enemy ASCII art string
    pub fn from_ascii(ascii: &str) -> Self {
        let lines: Vec<String> = ascii.lines().map(|s| s.to_string()).collect();
        Self::new(if lines.is_empty() {
            vec!["???".to_string()]
        } else {
            lines
        })
    }
    
    /// Apply damage to the visual state
    pub fn apply_damage(&mut self, damage_pct: f32, location: HitLocation) {
        let mut rng = thread_rng();
        
        // Determine wound severity
        let severity = WoundSeverity::from_damage_pct(damage_pct);
        
        // Find position for wound
        let pos = self.get_hit_position(location, &mut rng);
        
        // Add wound marker
        self.damage_overlays.wounds.push(WoundMarker {
            position: pos,
            severity,
            char_override: severity.char(),
        });
        
        // Update total severity
        self.damage_overlays.total_severity += severity.value();
        
        // Add blood particles for heavier wounds
        if severity as u8 >= WoundSeverity::Cut as u8 {
            self.add_blood_particles(pos, &mut rng);
        }
        
        // Update posture
        self.update_posture();
        
        // Invalidate cache
        self.cached_render = None;
    }
    
    /// Update posture based on cumulative damage
    fn update_posture(&mut self) {
        self.posture = match self.damage_overlays.total_severity {
            0..=2 => EnemyPosture::Confident,
            3..=5 => EnemyPosture::Wary,
            6..=8 => EnemyPosture::Staggered,
            9..=12 => EnemyPosture::Wounded,
            _ => EnemyPosture::Dying,
        };
    }
    
    /// Get position in ASCII art for a hit location
    fn get_hit_position(&self, location: HitLocation, rng: &mut ThreadRng) -> (usize, usize) {
        let height = self.base_art.len();
        let width = self.base_art.first().map(|s| s.len()).unwrap_or(5);
        
        match location {
            HitLocation::Head => (0.min(height - 1), width / 2),
            HitLocation::Torso => (height / 2, width / 2),
            HitLocation::LeftArm => (height / 2, width / 4),
            HitLocation::RightArm => (height / 2, 3 * width / 4),
            HitLocation::Legs => ((height - 1).min(height - 1), width / 2),
            HitLocation::Center => (height / 2, width / 2),
            HitLocation::Random => {
                let row = rng.gen_range(0..height);
                let col = rng.gen_range(0..width);
                (row, col)
            }
        }
    }
    
    /// Add blood particle effects near a wound
    fn add_blood_particles(&mut self, near: (usize, usize), rng: &mut ThreadRng) {
        let blood_chars = ['·', ':', '.', ',', '•'];
        let count = rng.gen_range(2..=4);
        
        for _ in 0..count {
            let offset_row = rng.gen_range(-1i32..=1);
            let offset_col = rng.gen_range(-2i32..=2);
            let new_row = (near.0 as i32 + offset_row).max(0) as usize;
            let new_col = (near.1 as i32 + offset_col).max(0) as usize;
            let ch = *blood_chars.choose(rng).unwrap();
            
            self.damage_overlays.particles.push(DamageParticle {
                position: (new_row, new_col),
                char: ch,
            });
        }
    }
    
    /// Render the current visual state with all damage applied
    pub fn render(&mut self) -> Vec<String> {
        // Return cached if available
        if let Some(ref cached) = self.cached_render {
            return cached.clone();
        }
        
        let mut art = self.base_art.clone();
        
        // Apply posture shift
        art = self.apply_posture_shift(art);
        
        // Apply wound markers
        for wound in &self.damage_overlays.wounds {
            self.apply_char_at(&mut art, wound.position, wound.char_override);
        }
        
        // Apply blood particles (only on empty spaces)
        for particle in &self.damage_overlays.particles {
            if self.char_at(&art, particle.position) == Some(' ') {
                self.apply_char_at(&mut art, particle.position, particle.char);
            }
        }
        
        // Cache and return
        self.cached_render = Some(art.clone());
        art
    }
    
    /// Get character at position
    fn char_at(&self, art: &[String], pos: (usize, usize)) -> Option<char> {
        art.get(pos.0).and_then(|row| row.chars().nth(pos.1))
    }
    
    /// Set character at position
    fn apply_char_at(&self, art: &mut Vec<String>, pos: (usize, usize), ch: char) {
        if pos.0 < art.len() {
            let row = &mut art[pos.0];
            if pos.1 < row.len() {
                let mut chars: Vec<char> = row.chars().collect();
                chars[pos.1] = ch;
                *row = chars.into_iter().collect();
            }
        }
    }
    
    /// Apply posture-based visual shifts
    fn apply_posture_shift(&self, mut art: Vec<String>) -> Vec<String> {
        match self.posture {
            EnemyPosture::Confident => art,
            EnemyPosture::Wary => {
                // Slight shift
                for line in &mut art {
                    *line = format!(" {}", line.trim_end());
                }
                art
            }
            EnemyPosture::Staggered => {
                // Asymmetric shift
                for (i, line) in art.iter_mut().enumerate() {
                    if i % 2 == 0 {
                        *line = format!(" {}", line.trim_end());
                    }
                }
                art
            }
            EnemyPosture::Wounded => {
                // Compress/hunch
                for line in &mut art {
                    *line = format!("  {}", line.trim_end());
                }
                art
            }
            EnemyPosture::Dying => {
                // Dramatic lean
                let half_len = art.len() / 2;
                for (i, line) in art.iter_mut().enumerate() {
                    let offset = i.saturating_sub(half_len);
                    *line = format!("{}{}", " ".repeat(offset), line.trim_end());
                }
                art
            }
        }
    }
    
    /// Get current posture
    pub fn get_posture(&self) -> EnemyPosture {
        self.posture
    }
    
    /// Check if enemy looks near death
    pub fn is_visually_dying(&self) -> bool {
        self.posture == EnemyPosture::Dying || self.posture == EnemyPosture::Wounded
    }
    
    /// Reset visual state (for new combat)
    pub fn reset(&mut self) {
        self.damage_overlays = DamageOverlays::default();
        self.posture = EnemyPosture::Confident;
        self.cached_render = None;
    }
    
    /// Update from health percentage (alternative to apply_damage)
    pub fn update_from_health(&mut self, health_pct: f32) {
        let new_posture = EnemyPosture::from_health_pct(health_pct);
        if new_posture != self.posture {
            self.posture = new_posture;
            self.cached_render = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_posture_from_health() {
        assert_eq!(EnemyPosture::from_health_pct(1.0), EnemyPosture::Confident);
        assert_eq!(EnemyPosture::from_health_pct(0.6), EnemyPosture::Wary);
        assert_eq!(EnemyPosture::from_health_pct(0.3), EnemyPosture::Staggered);
        assert_eq!(EnemyPosture::from_health_pct(0.15), EnemyPosture::Wounded);
        assert_eq!(EnemyPosture::from_health_pct(0.05), EnemyPosture::Dying);
    }
    
    #[test]
    fn test_damage_application() {
        let mut state = EnemyVisualState::new(vec![
            "  O  ".to_string(),
            " /|\\ ".to_string(),
            " / \\ ".to_string(),
        ]);
        
        state.apply_damage(0.20, HitLocation::Torso);
        assert!(state.damage_overlays.wounds.len() > 0);
        assert!(state.damage_overlays.total_severity > 0);
    }
}
