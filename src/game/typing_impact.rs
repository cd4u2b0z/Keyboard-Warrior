//! Typing Impact System - Fuses typing directly to combat damage
//!
//! Every keystroke builds damage. Rhythm and speed matter.
//! The way you type determines the nature of your attack.
//!
//! Design: Typing should feel tactile, not like a detached UI layer.

use std::time::Instant;
use serde::{Deserialize, Serialize};

/// Tracks typing and translates it to combat impact frame-by-frame
#[derive(Debug, Clone)]
pub struct TypingImpact {
    /// Current attack being typed
    pub current_attack: AttackSequence,
    /// Pending damage to apply (builds with each keystroke)
    pub pending_damage: f32,
    /// Damage multiplier from typing quality
    pub quality_multiplier: f32,
    /// Visual impact intensity (0.0 - 1.0)
    pub impact_intensity: f32,
    /// Attack type based on word completion
    pub attack_type: AttackType,
    /// Whether last keystroke was correct
    pub last_correct: bool,
}

/// Sequence of keystrokes forming an attack
#[derive(Debug, Clone)]
pub struct AttackSequence {
    /// The word being typed
    pub word: String,
    /// What's been typed so far
    pub typed: String,
    /// When typing started
    pub started_at: Instant,
    /// Individual keystroke data
    pub keystrokes: Vec<Keystroke>,
}

impl Default for AttackSequence {
    fn default() -> Self {
        Self {
            word: String::new(),
            typed: String::new(),
            started_at: Instant::now(),
            keystrokes: Vec::new(),
        }
    }
}

/// Data for a single keystroke
#[derive(Debug, Clone)]
pub struct Keystroke {
    /// The character typed
    pub char: char,
    /// Whether it was correct
    pub correct: bool,
    /// When it was typed
    pub timestamp: Instant,
    /// Time since last keystroke (ms)
    pub interval_ms: u32,
}

/// Attack type determined by typing performance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackType {
    /// Slow, methodical — single heavy strike
    Deliberate,
    /// Fast, flowing — rapid combo
    Flurry,
    /// Perfect accuracy — precision strike
    Precision,
    /// Messy but fast — wild swings
    Frantic,
    /// Mixed performance — normal attack
    Standard,
}

impl AttackType {
    /// Damage multiplier for this attack type
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            AttackType::Precision => 1.5,   // Perfect accuracy = big bonus
            AttackType::Flurry => 1.3,      // Fast = combo potential
            AttackType::Deliberate => 1.2,  // Slow but accurate = solid
            AttackType::Frantic => 0.9,     // Fast but sloppy = penalty
            AttackType::Standard => 1.0,
        }
    }
    
    /// Get descriptive name
    pub fn name(&self) -> &'static str {
        match self {
            AttackType::Precision => "PRECISION STRIKE",
            AttackType::Flurry => "FLURRY",
            AttackType::Deliberate => "Heavy Blow",
            AttackType::Frantic => "Wild Swing",
            AttackType::Standard => "Attack",
        }
    }
    
    /// Get icon for UI
    pub fn icon(&self) -> &'static str {
        match self {
            AttackType::Precision => "⚔",
            AttackType::Flurry => "⚡",
            AttackType::Deliberate => "🗡",
            AttackType::Frantic => "💥",
            AttackType::Standard => "→",
        }
    }
}

/// Result from a single keystroke
#[derive(Debug, Clone)]
pub struct KeystrokeResult {
    /// Damage added by this keystroke
    pub damage_this_stroke: f32,
    /// Visual intensity (0.0 - 1.0)
    pub visual_intensity: f32,
    /// Sound pitch modifier (0.5 - 1.5)
    pub sound_pitch: f32,
    /// Screen shake amount
    pub screen_shake: f32,
    /// Rhythm bonus applied
    pub rhythm_bonus: f32,
    /// Was it correct?
    pub correct: bool,
}

/// Result from completing a word
#[derive(Debug, Clone)]
pub struct WordCompletionResult {
    /// Total damage dealt
    pub damage: i32,
    /// Attack type used
    pub attack_type: AttackType,
    /// Words per minute achieved
    pub wpm: f32,
    /// Accuracy percentage
    pub accuracy: f32,
    /// Was it a perfect word?
    pub perfect: bool,
    /// Combat log message
    pub message: String,
}

impl Default for TypingImpact {
    fn default() -> Self {
        Self::new()
    }
}

impl TypingImpact {
    pub fn new() -> Self {
        Self {
            current_attack: AttackSequence::default(),
            pending_damage: 0.0,
            quality_multiplier: 1.0,
            impact_intensity: 0.0,
            attack_type: AttackType::Standard,
            last_correct: true,
        }
    }
    
    /// Start tracking a new word
    pub fn start_word(&mut self, word: String) {
        self.current_attack = AttackSequence {
            word,
            typed: String::new(),
            started_at: Instant::now(),
            keystrokes: Vec::new(),
        };
        self.pending_damage = 0.0;
        self.impact_intensity = 0.0;
        self.attack_type = AttackType::Standard;
    }
    
    /// Process a keystroke during combat
    pub fn on_keystroke(&mut self, ch: char, correct: bool) -> KeystrokeResult {
        let now = Instant::now();
        let interval = self.current_attack.keystrokes.last()
            .map(|k| now.duration_since(k.timestamp).as_millis() as u32)
            .unwrap_or(0);
        
        self.current_attack.keystrokes.push(Keystroke {
            char: ch,
            correct,
            timestamp: now,
            interval_ms: interval,
        });
        
        self.current_attack.typed.push(ch);
        self.last_correct = correct;
        
        // Calculate per-keystroke impact
        let impact = self.calculate_keystroke_impact(correct, interval);
        self.pending_damage += impact.damage_this_stroke;
        self.impact_intensity = impact.visual_intensity;
        
        impact
    }
    
    fn calculate_keystroke_impact(&self, correct: bool, interval_ms: u32) -> KeystrokeResult {
        if !correct {
            return KeystrokeResult {
                damage_this_stroke: 0.0,
                visual_intensity: 0.8,  // Error flash
                sound_pitch: 0.5,       // Low, discordant
                screen_shake: 0.1,
                rhythm_bonus: 0.0,
                correct: false,
            };
        }
        
        // Base damage per correct keystroke
        let base = 1.5;
        
        // Speed bonus: faster = more damage (up to 2x at 50ms intervals)
        let speed_mult = if interval_ms > 0 {
            (200.0 / interval_ms as f32).min(2.0).max(0.5)
        } else {
            1.0
        };
        
        // Rhythm bonus: consistent intervals feel better and do more
        let rhythm_mult = self.calculate_rhythm_bonus(interval_ms);
        
        let damage = base * speed_mult * rhythm_mult;
        
        KeystrokeResult {
            damage_this_stroke: damage,
            visual_intensity: (speed_mult * 0.5).min(1.0),
            sound_pitch: 0.8 + (speed_mult * 0.2),
            screen_shake: damage * 0.03,
            rhythm_bonus: rhythm_mult - 1.0,
            correct: true,
        }
    }
    
    fn calculate_rhythm_bonus(&self, current_interval: u32) -> f32 {
        // Compare to average interval of last 3 keystrokes
        let recent: Vec<u32> = self.current_attack.keystrokes
            .iter()
            .rev()
            .take(3)
            .filter(|k| k.interval_ms > 0)
            .map(|k| k.interval_ms)
            .collect();
        
        if recent.len() < 2 || current_interval == 0 {
            return 1.0;
        }
        
        let avg: u32 = recent.iter().sum::<u32>() / recent.len() as u32;
        let variance = (current_interval as i32 - avg as i32).abs() as u32;
        
        // Low variance (consistent rhythm) = up to 50% bonus
        if variance < 30 {
            1.5
        } else if variance < 60 {
            1.25
        } else if variance < 100 {
            1.1
        } else {
            1.0
        }
    }
    
    /// Complete the current word and calculate final damage
    pub fn complete_word(&mut self, base_damage: i32) -> WordCompletionResult {
        let elapsed = self.current_attack.started_at.elapsed();
        let char_count = self.current_attack.typed.len();
        let correct_count = self.current_attack.keystrokes.iter().filter(|k| k.correct).count();
        
        let accuracy = if char_count > 0 {
            correct_count as f32 / char_count as f32
        } else {
            1.0
        };
        
        let wpm = if elapsed.as_secs_f32() > 0.0 {
            (char_count as f32 / 5.0) / (elapsed.as_secs_f32() / 60.0)
        } else {
            0.0
        };
        
        // Determine attack type
        self.attack_type = self.determine_attack_type(wpm, accuracy);
        
        // Calculate final damage
        // Base from pending keystrokes + base damage + attack type modifier
        let type_mult = self.attack_type.damage_multiplier();
        let accuracy_mult = 0.5 + (accuracy * 0.5); // 50-100% based on accuracy
        
        let final_damage = ((base_damage as f32 + self.pending_damage) * type_mult * accuracy_mult).round() as i32;
        let perfect = accuracy >= 0.99;
        
        WordCompletionResult {
            damage: final_damage.max(1), // Always at least 1 damage
            attack_type: self.attack_type,
            wpm,
            accuracy,
            perfect,
            message: self.generate_attack_message(final_damage, perfect),
        }
    }
    
    fn determine_attack_type(&self, wpm: f32, accuracy: f32) -> AttackType {
        match (wpm, accuracy) {
            (w, a) if a >= 0.99 && w >= 80.0 => AttackType::Precision,
            (w, a) if a >= 0.95 && w >= 100.0 => AttackType::Flurry,
            (w, a) if w < 40.0 && a >= 0.95 => AttackType::Deliberate,
            (w, a) if w >= 70.0 && a < 0.85 => AttackType::Frantic,
            _ => AttackType::Standard,
        }
    }
    
    fn generate_attack_message(&self, damage: i32, perfect: bool) -> String {
        let icon = self.attack_type.icon();
        let name = self.attack_type.name();
        
        if perfect {
            format!("{} PERFECT {}! {} damage!", icon, name, damage)
        } else {
            match self.attack_type {
                AttackType::Precision => format!("{} {}! {} damage!", icon, name, damage),
                AttackType::Flurry => format!("{} {}! {} damage!", icon, name, damage),
                AttackType::Deliberate => format!("{} {}. {} damage.", icon, name, damage),
                AttackType::Frantic => format!("{} {}! {} damage.", icon, name, damage),
                AttackType::Standard => format!("You deal {} damage.", damage),
            }
        }
    }
    
    /// Get current pending damage preview
    pub fn get_pending_damage(&self) -> i32 {
        self.pending_damage.round() as i32
    }
    
    /// Get current attack intensity for visuals
    pub fn get_intensity(&self) -> f32 {
        self.impact_intensity
    }
    
    /// Reset for next combat
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_attack_types() {
        assert_eq!(AttackType::Precision.damage_multiplier(), 1.5);
        assert_eq!(AttackType::Frantic.damage_multiplier(), 0.9);
    }
    
    #[test]
    fn test_keystroke_damage() {
        let mut impact = TypingImpact::new();
        impact.start_word("test".to_string());
        
        let result = impact.on_keystroke('t', true);
        assert!(result.correct);
        assert!(result.damage_this_stroke > 0.0);
    }
}
