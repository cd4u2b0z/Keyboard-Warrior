//! Player Avatar System - The player's visual presence in combat
//!
//! The player is not just a health bar. They have:
//! - Class-specific ASCII art
//! - Animation states (idle, typing, attacking, hit, victory)
//! - Visual response to combat events
//!
//! Design: The player should FEEL present in the world

use serde::{Deserialize, Serialize};

/// Player avatar with animations
#[derive(Debug, Clone)]
pub struct PlayerAvatar {
    /// Player's class
    pub class: PlayerClass,
    /// Current animation state
    pub state: AvatarState,
    /// Timer for animation (ms remaining)
    pub animation_timer: u32,
    /// Current health percentage (0-100)
    pub health_percent: u32,
}

/// Player class for different visuals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerClass {
    Freelancer,
    Wordsmith,
    Codebreaker,
    Chronicler,
}

impl PlayerClass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Freelancer => "Freelancer",
            Self::Wordsmith => "Wordsmith",
            Self::Codebreaker => "Codebreaker",
            Self::Chronicler => "Chronicler",
        }
    }
}

/// Current animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarState {
    Idle,
    Typing,
    Attacking,
    Hit,
    Victory,
    Wounded,
    Defending,
}

impl Default for PlayerAvatar {
    fn default() -> Self {
        Self::new(PlayerClass::Freelancer)
    }
}

impl PlayerAvatar {
    pub fn new(class: PlayerClass) -> Self {
        Self {
            class,
            state: AvatarState::Idle,
            animation_timer: 0,
            health_percent: 100,
        }
    }
    
    /// Get ASCII art for current state
    pub fn get_art(&self) -> Vec<&'static str> {
        match self.class {
            PlayerClass::Freelancer => self.freelancer_art(),
            PlayerClass::Wordsmith => self.wordsmith_art(),
            PlayerClass::Codebreaker => self.codebreaker_art(),
            PlayerClass::Chronicler => self.chronicler_art(),
        }
    }
    
    fn freelancer_art(&self) -> Vec<&'static str> {
        match self.state {
            AvatarState::Idle => vec![
                "  ,--o--,  ",
                "  | /\\ |  ",
                " /| || |\\",
                " |      | ",
                "  ------  ",
                "   /  \\  ",
                "  /    \\ ",
            ],
            AvatarState::Typing => vec![
                "  ,--o--,  ",
                "  | /\\ |  ",
                " /|=||=|\\",
                " |######| ",
                "  ------  ",
                "   /  \\  ",
                "  /    \\ ",
            ],
            AvatarState::Attacking => vec![
                "  ,--*--,  ",
                "  | /\\ |--",
                " /| || |\\",
                " |######| ",
                "  ======  ",
                "   /  \\  ",
                "  /    \\ ",
            ],
            AvatarState::Hit => vec![
                "  ,--x--,  ",
                " \\| /\\ |  ",
                " /| || |\\",
                " |      | ",
                "  ------  ",
                "    /\\   ",
                "   /  \\  ",
            ],
            AvatarState::Victory => vec![
                "  ,--@--,  ",
                " /| /\\ |\\",
                "/ | || | \\",
                "  ######  ",
                "  ======  ",
                "   /  \\  ",
                "  /    \\ ",
            ],
            AvatarState::Wounded => vec![
                "  ,--X--,  ",
                "  | /\\ |  ",
                " /| || |\\",
                " |......| ",
                "  ------  ",
                "   /\\    ",
                "  /  \\   ",
            ],
            AvatarState::Defending => vec![
                "  ,--o--,  ",
                " [| /\\ |  ",
                " [| || |\\",
                " [|      | ",
                " [ ------  ",
                "   /  \\  ",
                "  /    \\ ",
            ],
        }
    }
    
    fn wordsmith_art(&self) -> Vec<&'static str> {
        match self.state {
            AvatarState::Idle => vec![
                "   ,<>,   ",
                "  ,---,   ",
                " /| * |\\",
                " |.....|  ",
                "  =====   ",
                "   / \\   ",
                "  =/ \\=  ",
            ],
            AvatarState::Typing => vec![
                "   ,<>,   ",
                "  ,---,   ",
                " /|=*=|\\",
                " |#####|  ",
                "  =====   ",
                "   / \\   ",
                "  =/ \\=  ",
            ],
            AvatarState::Attacking => vec![
                "   ,<>,~~~",
                "  ,---,   ",
                " /| * |\\",
                " |#####|  ",
                "  =====   ",
                "   / \\   ",
                "  =/ \\=  ",
            ],
            _ => self.freelancer_art(),
        }
    }
    
    fn codebreaker_art(&self) -> Vec<&'static str> {
        match self.state {
            AvatarState::Idle => vec![
                "  [=*=]   ",
                "  | 01|   ",
                " [=====]  ",
                " |.....|  ",
                " [=====]  ",
                "   | |    ",
                "  [| |]   ",
            ],
            AvatarState::Typing => vec![
                "  [=@=]   ",
                "  |>01|   ",
                " [==#==]  ",
                " |#####|  ",
                " [=====]  ",
                "   | |    ",
                "  [| |]   ",
            ],
            AvatarState::Attacking => vec![
                "  [=*=]>> ",
                "  | 01|   ",
                " [=====]  ",
                " |#####|  ",
                " [=====]  ",
                "   | |    ",
                "  [| |]   ",
            ],
            _ => self.freelancer_art(),
        }
    }
    
    fn chronicler_art(&self) -> Vec<&'static str> {
        match self.state {
            AvatarState::Idle => vec![
                "   ,~,    ",
                "  ,+-+,   ",
                " /| = |\\",
                " |.===.|  ",
                "  -----   ",
                "   | |    ",
                "  [===]   ",
            ],
            AvatarState::Typing => vec![
                "   ,~,    ",
                "  ,+-+,   ",
                " /|===|\\",
                " |#===|   ",
                "  -----   ",
                "   | |    ",
                "  [===]   ",
            ],
            AvatarState::Attacking => vec![
                "   ,~,~~~ ",
                "  ,+-+,   ",
                " /| = |\\",
                " |#===|   ",
                "  -----   ",
                "   | |    ",
                "  [===]   ",
            ],
            _ => self.freelancer_art(),
        }
    }
    
    /// Trigger typing animation
    pub fn on_keystroke(&mut self) {
        self.state = AvatarState::Typing;
        self.animation_timer = 100;
    }
    
    /// Trigger attack animation  
    pub fn on_attack(&mut self) {
        self.state = AvatarState::Attacking;
        self.animation_timer = 300;
    }
    
    /// Trigger hit animation
    pub fn on_hit(&mut self) {
        self.state = AvatarState::Hit;
        self.animation_timer = 400;
    }
    
    /// Trigger victory animation
    pub fn on_victory(&mut self) {
        self.state = AvatarState::Victory;
        self.animation_timer = 1000;
    }
    
    /// Trigger defending animation
    pub fn on_defend(&mut self) {
        self.state = AvatarState::Defending;
        self.animation_timer = 500;
    }
    
    /// Update health and potentially set wounded state
    pub fn update_health(&mut self, percent: u32) {
        self.health_percent = percent;
        if percent < 25 && self.state == AvatarState::Idle {
            self.state = AvatarState::Wounded;
        }
    }
    
    /// Update animation timer
    pub fn update(&mut self, delta_ms: u32) {
        if self.animation_timer > 0 {
            self.animation_timer = self.animation_timer.saturating_sub(delta_ms);
            
            if self.animation_timer == 0 {
                self.state = if self.health_percent < 25 {
                    AvatarState::Wounded
                } else {
                    AvatarState::Idle
                };
            }
        }
    }
    
    /// Check if currently animating
    pub fn is_animating(&self) -> bool {
        self.animation_timer > 0
    }
    
    /// Get a description of current state
    pub fn state_description(&self) -> &'static str {
        match self.state {
            AvatarState::Idle => "Ready",
            AvatarState::Typing => "Typing...",
            AvatarState::Attacking => "ATTACK!",
            AvatarState::Hit => "Damaged!",
            AvatarState::Victory => "Victory!",
            AvatarState::Wounded => "Wounded...",
            AvatarState::Defending => "Defending!",
        }
    }
    
    /// Get health bar string
    pub fn health_bar(&self, width: usize) -> String {
        let filled = (self.health_percent as usize * width) / 100;
        let empty = width - filled;
        
        let fill_char = if self.health_percent > 50 {
            '#'
        } else if self.health_percent > 25 {
            '='
        } else {
            '.'
        };
        
        format!("[{}{}]", 
            fill_char.to_string().repeat(filled),
            " ".repeat(empty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_avatar_creation() {
        let avatar = PlayerAvatar::new(PlayerClass::Freelancer);
        assert_eq!(avatar.state, AvatarState::Idle);
        assert_eq!(avatar.health_percent, 100);
    }
    
    #[test]
    fn test_animation_states() {
        let mut avatar = PlayerAvatar::new(PlayerClass::Wordsmith);
        
        avatar.on_keystroke();
        assert_eq!(avatar.state, AvatarState::Typing);
        assert!(avatar.is_animating());
        
        avatar.on_attack();
        assert_eq!(avatar.state, AvatarState::Attacking);
    }
    
    #[test]
    fn test_wounded_state() {
        let mut avatar = PlayerAvatar::new(PlayerClass::Codebreaker);
        
        avatar.update_health(20);
        avatar.update(0);
        assert!(avatar.health_percent < 25);
    }
    
    #[test]
    fn test_art_exists() {
        for class in [PlayerClass::Freelancer, PlayerClass::Wordsmith, 
                      PlayerClass::Codebreaker, PlayerClass::Chronicler] {
            let avatar = PlayerAvatar::new(class);
            let art = avatar.get_art();
            assert!(!art.is_empty());
        }
    }
}
