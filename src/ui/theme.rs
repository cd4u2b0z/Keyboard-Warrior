//! Visual Theme System - Consistent styling across the game
//!
//! This module provides:
//! - Color palette with semantic meanings
//! - Border styles (box drawing characters)
//! - Nerd Font icons for UI elements
//! - Style presets for common patterns

use ratatui::style::{Color, Modifier, Style};

/// Color palette - consistent across all UI
pub struct Palette;

impl Palette {
    // Primary colors
    pub const PRIMARY: Color = Color::Rgb(0, 200, 200);      // Cyan-ish
    pub const SECONDARY: Color = Color::Rgb(200, 150, 50);   // Gold
    pub const ACCENT: Color = Color::Rgb(200, 50, 200);      // Magenta
    
    // Semantic colors
    pub const SUCCESS: Color = Color::Rgb(50, 200, 100);     // Green
    pub const WARNING: Color = Color::Rgb(230, 180, 50);     // Yellow-orange
    pub const DANGER: Color = Color::Rgb(220, 60, 60);       // Red
    pub const INFO: Color = Color::Rgb(100, 150, 255);       // Light blue
    
    // Combat colors
    pub const PLAYER_HP: Color = Color::Rgb(80, 200, 80);    // Bright green
    pub const ENEMY_HP: Color = Color::Rgb(220, 80, 80);     // Red
    pub const MP: Color = Color::Rgb(80, 130, 230);          // Blue
    pub const COMBO: Color = Color::Rgb(255, 200, 50);       // Bright gold
    
    // Rarity colors
    pub const COMMON: Color = Color::Rgb(180, 180, 180);     // Gray
    pub const UNCOMMON: Color = Color::Rgb(80, 200, 80);     // Green
    pub const RARE: Color = Color::Rgb(80, 150, 255);        // Blue
    pub const EPIC: Color = Color::Rgb(180, 80, 255);        // Purple
    pub const LEGENDARY: Color = Color::Rgb(255, 180, 50);   // Orange
    
    // UI colors
    pub const BG_DARK: Color = Color::Rgb(20, 20, 25);       // Near black
    pub const BG_PANEL: Color = Color::Rgb(30, 30, 40);      // Panel bg
    pub const TEXT: Color = Color::Rgb(220, 220, 220);       // White text
    pub const TEXT_DIM: Color = Color::Rgb(120, 120, 130);   // Muted text
    pub const BORDER: Color = Color::Rgb(80, 80, 100);       // Border default
    pub const BORDER_FOCUS: Color = Color::Rgb(100, 180, 200); // Focused border
    
    // Typing feedback colors
    pub const TYPED_CORRECT: Color = Color::Rgb(80, 230, 80);   // Bright green
    pub const TYPED_WRONG: Color = Color::Rgb(255, 80, 80);     // Bright red
    pub const UNTYPED: Color = Color::Rgb(100, 100, 110);       // Gray
    pub const CURSOR: Color = Color::Rgb(100, 200, 255);        // Cyan cursor
    
    // Flow state colors
    pub const FLOW_BUILDING: Color = Color::Rgb(200, 200, 100);    // Yellow
    pub const FLOW_FLOWING: Color = Color::Rgb(100, 200, 255);     // Cyan
    pub const FLOW_TRANSCENDENT: Color = Color::Rgb(255, 100, 255); // Magenta
    pub const FLOW_RECOVERING: Color = Color::Rgb(200, 100, 100);  // Faded red
    
    // Zone-specific colors
    pub const ZONE_SHATTERED_HALLS: Color = Color::Rgb(140, 140, 160);   // Stone gray
    pub const ZONE_SUNKEN_ARCHIVES: Color = Color::Rgb(80, 180, 200);    // Deep cyan
    pub const ZONE_BLIGHTED_GARDENS: Color = Color::Rgb(100, 180, 80);   // Sickly green
    pub const ZONE_CLOCKWORK_DEPTHS: Color = Color::Rgb(220, 180, 60);   // Brass yellow
    pub const ZONE_VOIDS_EDGE: Color = Color::Rgb(180, 80, 220);         // Void purple
    pub const ZONE_THE_BREACH: Color = Color::Rgb(220, 60, 60);          // Blood red
}

/// Nerd Font icons used throughout the UI
pub struct Icons;

impl Icons {
    // Navigation & UI
    pub const ARROW_RIGHT: &'static str = "󰁕";
    pub const ARROW_LEFT: &'static str = "󰁍";
    pub const ARROW_UP: &'static str = "󰁝";
    pub const ARROW_DOWN: &'static str = "󰁅";
    pub const HELP: &'static str = "󰋗";
    pub const MENU: &'static str = "󰍜";
    pub const CLOSE: &'static str = "󰅖";
    pub const CHECK: &'static str = "󰄬";
    pub const CROSS: &'static str = "󰅙";
    pub const INFO: &'static str = "󰋽";
    pub const WARNING: &'static str = "󰀪";
    pub const ERROR: &'static str = "󰅚";
    
    // Game elements
    pub const SWORD: &'static str = "󰓥";
    pub const SHIELD: &'static str = "󰒘";
    pub const HEART: &'static str = "󰣐";
    pub const MANA: &'static str = "󱠇";
    pub const GOLD: &'static str = "󰆼";
    pub const XP: &'static str = "󰓎";
    pub const LEVEL: &'static str = "󰞋";
    pub const SKULL: &'static str = "󰯈";
    pub const CROWN: &'static str = "󰔰";
    pub const FIRE: &'static str = "󰈸";
    pub const MAGIC: &'static str = "󱡃";
    pub const POTION: &'static str = "󱂓";
    pub const KEY: &'static str = "󰌆";
    pub const CHEST: &'static str = "󱋣";
    pub const MAP: &'static str = "󰍋";
    pub const DUNGEON: &'static str = "󰘛";
    pub const DOOR: &'static str = "󰞔";
    
    // Classes
    pub const WORDSMITH: &'static str = "󰜁";
    pub const SCRIBE: &'static str = "󰯂";
    pub const SPELLWEAVER: &'static str = "󰺝";
    pub const BARBARIAN: &'static str = "󰓥";
    pub const TRICKSTER: &'static str = "󰗎";
    
    // Typing & Combat
    pub const KEYBOARD: &'static str = "󰌌";
    pub const COMBO: &'static str = "󱋊";
    pub const TIMER: &'static str = "󰔟";
    pub const SPEED: &'static str = "󰓅";
    pub const ACCURACY: &'static str = "󰇄";
    pub const TARGET: &'static str = "󰓾";
    pub const BURST: &'static str = "󰛨";
    pub const CRITICAL: &'static str = "󱐋";
    
    // Status & Effects
    pub const BUFF: &'static str = "󰁝";
    pub const DEBUFF: &'static str = "󰁅";
    pub const HEAL: &'static str = "󰣐";
    pub const DAMAGE: &'static str = "󱐌";
    pub const DEFEND: &'static str = "󰒘";
    pub const STUN: &'static str = "󰒖";
    pub const POISON: &'static str = "󱂓";
    pub const BURN: &'static str = "󰈸";
    
    // Rooms/Encounters
    pub const COMBAT: &'static str = "󰓥";
    pub const SHOP: &'static str = "󰆼";
    pub const REST: &'static str = "󰈸";
    pub const EVENT: &'static str = "󰗀";
    pub const BOSS: &'static str = "󰯈";
    pub const TREASURE: &'static str = "󱋣";
    pub const MYSTERY: &'static str = "󰛓";
    
    // Misc
    pub const STAR: &'static str = "󰓎";
    pub const SPARK: &'static str = "󱐋";
    pub const WAVE: &'static str = "󱗿";
    pub const QUOTE: &'static str = "󰗡";
    pub const BOOK: &'static str = "󰂽";
    pub const SCROLL: &'static str = "󱪙";
    pub const BAKLAVA: &'static str = "󰩛";
}

/// Styled border characters for different UI contexts
pub struct Borders;

impl Borders {
    // Standard box drawing
    pub const SINGLE: BorderSet = BorderSet {
        top_left: "┌", top: "─", top_right: "┐",
        left: "│", right: "│",
        bottom_left: "└", bottom: "─", bottom_right: "┘",
    };
    
    // Double line borders
    pub const DOUBLE: BorderSet = BorderSet {
        top_left: "╔", top: "═", top_right: "╗",
        left: "║", right: "║",
        bottom_left: "╚", bottom: "═", bottom_right: "╝",
    };
    
    // Rounded borders
    pub const ROUNDED: BorderSet = BorderSet {
        top_left: "╭", top: "─", top_right: "╮",
        left: "│", right: "│",
        bottom_left: "╰", bottom: "─", bottom_right: "╯",
    };
    
    // Heavy borders
    pub const HEAVY: BorderSet = BorderSet {
        top_left: "┏", top: "━", top_right: "┓",
        left: "┃", right: "┃",
        bottom_left: "┗", bottom: "━", bottom_right: "┛",
    };
    
    // Decorative / mystical borders
    pub const MYSTICAL: BorderSet = BorderSet {
        top_left: "◈", top: "◇", top_right: "◈",
        left: "◆", right: "◆",
        bottom_left: "◈", bottom: "◇", bottom_right: "◈",
    };
    
    // ASCII only (fallback)
    pub const ASCII: BorderSet = BorderSet {
        top_left: "+", top: "-", top_right: "+",
        left: "|", right: "|",
        bottom_left: "+", bottom: "-", bottom_right: "+",
    };
}

#[derive(Clone, Copy)]
pub struct BorderSet {
    pub top_left: &'static str,
    pub top: &'static str,
    pub top_right: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub bottom_left: &'static str,
    pub bottom: &'static str,
    pub bottom_right: &'static str,
}

impl BorderSet {
    /// Create a horizontal line of this border style
    pub fn h_line(&self, width: usize) -> String {
        self.top.repeat(width)
    }
    
    /// Create a title line with embedded text
    pub fn title_line(&self, text: &str, width: usize) -> String {
        let text_len = text.chars().count() + 2; // +2 for spaces
        let remaining = width.saturating_sub(text_len);
        let left_pad = remaining / 2;
        let right_pad = remaining - left_pad;
        
        format!("{}{} {} {}{}",
            self.top_left,
            self.top.repeat(left_pad),
            text,
            self.top.repeat(right_pad),
            self.top_right
        )
    }
}

/// Style presets for common UI patterns
pub struct Styles;

impl Styles {
    // Text styles
    pub fn title() -> Style {
        Style::default()
            .fg(Palette::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn subtitle() -> Style {
        Style::default()
            .fg(Palette::SECONDARY)
            .add_modifier(Modifier::ITALIC)
    }
    
    pub fn normal() -> Style {
        Style::default().fg(Palette::TEXT)
    }
    
    pub fn dim() -> Style {
        Style::default().fg(Palette::TEXT_DIM)
    }
    
    pub fn accent() -> Style {
        Style::default()
            .fg(Palette::ACCENT)
            .add_modifier(Modifier::BOLD)
    }
    
    // Status styles
    pub fn success() -> Style {
        Style::default()
            .fg(Palette::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn warning() -> Style {
        Style::default().fg(Palette::WARNING)
    }
    
    pub fn danger() -> Style {
        Style::default()
            .fg(Palette::DANGER)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn info() -> Style {
        Style::default().fg(Palette::INFO)
    }
    
    // Interactive styles
    pub fn selected() -> Style {
        Style::default()
            .fg(Palette::SECONDARY)
            .add_modifier(Modifier::BOLD | Modifier::REVERSED)
    }
    
    pub fn focused() -> Style {
        Style::default()
            .fg(Palette::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn keybind() -> Style {
        Style::default()
            .fg(Palette::SECONDARY)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn hint() -> Style {
        Style::default()
            .fg(Palette::TEXT_DIM)
            .add_modifier(Modifier::ITALIC)
    }
    
    // Typing styles
    pub fn typed_correct() -> Style {
        Style::default()
            .fg(Palette::TYPED_CORRECT)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn typed_wrong() -> Style {
        Style::default()
            .fg(Palette::TYPED_WRONG)
            .add_modifier(Modifier::UNDERLINED)
    }
    
    pub fn untyped() -> Style {
        Style::default().fg(Palette::UNTYPED)
    }
    
    pub fn cursor() -> Style {
        Style::default()
            .fg(Palette::CURSOR)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }
    
    // Block/Panel styles
    pub fn block_default() -> Style {
        Style::default()
            .bg(Palette::BG_PANEL)
    }
    
    pub fn border_default() -> Style {
        Style::default().fg(Palette::BORDER)
    }
    
    pub fn border_focus() -> Style {
        Style::default().fg(Palette::BORDER_FOCUS)
    }
    
    // Combat styles
    pub fn player_hp() -> Style {
        Style::default().fg(Palette::PLAYER_HP)
    }
    
    pub fn enemy_hp() -> Style {
        Style::default().fg(Palette::ENEMY_HP)
    }
    
    pub fn mp() -> Style {
        Style::default().fg(Palette::MP)
    }
    
    pub fn combo() -> Style {
        Style::default()
            .fg(Palette::COMBO)
            .add_modifier(Modifier::BOLD)
    }
}

/// Helper for creating styled spans quickly
pub fn icon_span(icon: &str, color: Color) -> ratatui::text::Span<'static> {
    ratatui::text::Span::styled(
        format!("{} ", icon),
        Style::default().fg(color),
    )
}

/// Format a stat display with icon
pub fn stat_display(icon: &str, value: impl std::fmt::Display, color: Color) -> String {
    format!("{} {}", icon, value)
}

/// Get the appropriate color for HP percentage
pub fn hp_color(percent: u16) -> Color {
    if percent > 66 {
        Palette::SUCCESS
    } else if percent > 33 {
        Palette::WARNING
    } else {
        Palette::DANGER
    }
}

/// Get color for combo level
pub fn combo_color(combo: i32) -> Color {
    if combo >= 25 {
        Palette::FLOW_TRANSCENDENT
    } else if combo >= 15 {
        Palette::DANGER
    } else if combo >= 8 {
        Palette::WARNING
    } else if combo >= 3 {
        Palette::INFO
    } else {
        Palette::TEXT_DIM
    }
}

/// Get color for WPM display
pub fn wpm_color(wpm: f32) -> Color {
    if wpm >= 100.0 {
        Palette::FLOW_TRANSCENDENT
    } else if wpm >= 80.0 {
        Palette::ACCENT
    } else if wpm >= 60.0 {
        Palette::WARNING
    } else if wpm >= 40.0 {
        Palette::INFO
    } else {
        Palette::TEXT
    }
}

/// Get color for accuracy display
pub fn accuracy_color(accuracy: f32) -> Color {
    if accuracy >= 98.0 {
        Palette::FLOW_TRANSCENDENT
    } else if accuracy >= 95.0 {
        Palette::SUCCESS
    } else if accuracy >= 85.0 {
        Palette::WARNING
    } else {
        Palette::DANGER
    }
}

/// Get color for a zone based on its name
pub fn zone_color(zone_name: &str) -> Color {
    match zone_name {
        "Shattered Halls" => Palette::ZONE_SHATTERED_HALLS,
        "Sunken Archives" => Palette::ZONE_SUNKEN_ARCHIVES,
        "Blighted Gardens" => Palette::ZONE_BLIGHTED_GARDENS,
        "Clockwork Depths" => Palette::ZONE_CLOCKWORK_DEPTHS,
        "Void's Edge" => Palette::ZONE_VOIDS_EDGE,
        "The Breach" => Palette::ZONE_THE_BREACH,
        _ => Palette::PRIMARY, // Default fallback
    }
}
