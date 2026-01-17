# 󰓥 TypingQuest

**A roguelike typing RPG for the terminal — defeat enemies through the rhythm of your keystrokes.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.2.1-blue?style=flat)](CHANGELOG.md)
[![Status](https://img.shields.io/badge/Status-Active_Development-orange?style=flat)]()
[![TUI](https://img.shields.io/badge/TUI-ratatui-purple?style=flat)](https://github.com/ratatui-org/ratatui)

---

## Vision

TypingQuest is a typing game that *feels* like an RPG. Every keystroke has weight. Combos build momentum. Flow states reward consistency. The dungeon unfolds through your fingers.

Inspired by [ttyper](https://github.com/max-niederman/ttyper), *Undertale*, *Balatro*, and *Hades*.

---

## Quick Start

```bash
# Clone and build
git clone https://github.com/cd4u2b0z/typingquest.git
cd typingquest
cargo build --release

# Run the game
./target/release/typingquest
```

**Requirements:**
- Rust 1.70+ 
- A terminal with Unicode support
- [Nerd Font](https://www.nerdfonts.com/) recommended for icons

---

## How It Plays

```
╭──────────────────────────────────────────────────────────────────╮
│  ◈═══════════════════════════════════════════════════════════◈  │
│    ████████╗██╗   ██╗██████╗ ██╗███╗   ██╗ ██████╗              │
│       ██║    ╚████╔╝ ██████╔╝██║██╔██╗ ██║██║  ███╗             │
│       ██║     ╚██╔╝  ██╔═══╝ ██║██║╚██╗██║██║   ██║             │
│       ╚═╝      ╚═╝   ╚═╝     ╚═╝╚═╝  ╚═══╝ ╚═════╝  QUEST  󰌌   │
│  ◈═══════════════════════════════════════════════════════════◈  │
╰──────────────────────────────────────────────────────────────────╯
```

### Core Loop

1. **Explore** — Navigate a 10-floor dungeon with procedural rooms
2. **Fight** — Type words to attack enemies; accuracy and speed deal damage
3. **Grow** — Level up, find items, learn spells, build your character
4. **Die** — Roguelike permadeath with meta-progression between runs

### Combat

Words appear. You type them. Damage happens.

- **Correct characters** flash green; errors flash red
- **Combos** build with consecutive correct words (up to 3x damage)
- **Flow states** reward consistent typing rhythm
- **Time pressure** adds urgency without being punishing

### Exploration

Each floor contains rooms: combat encounters, elite enemies, shops, rest sites, treasure, and random events. Choose your path. Manage your resources. Reach the boss.

---

## Controls

| Key | Action |
|-----|--------|
| `j/k` or `↑/↓` | Navigate menus |
| `Enter` or `e` | Confirm / Explore |
| `Backspace` | Fix typing errors |
| `Esc` | Back / Flee combat |
| `?` | Toggle help overlay |
| `i` | Inventory |
| `s` | Character stats |

---

## Classes

| Class | Style | Strength |
|-------|-------|----------|
| 󰜁 **Wordsmith** | Balanced | +10% damage, starts with Heal |
| 󰯂 **Scribe** | Spellcaster | +25% MP, faster spell learning |
| 󰺝 **Spellweaver** | Glass cannon | +50% spell damage, -20% HP |
| 󰓥 **Barbarian** | Tank | +30% HP, +15% damage, no spells |
| 󰗎 **Trickster** | Luck-based | Random bonuses, critical hits |

---

## Features

### Implemented (v0.2.1)

**Core Gameplay**
- 5 playable classes with distinct mechanics
- 10-floor dungeon with procedural room generation
- Typing-based combat with real-time feedback
- Item system — equipment, consumables, relics
- Spell system — elemental magic with MP costs
- Shop, rest, treasure, and event encounters

**Game Feel (Phase 3)**
- Combo system with damage multipliers (up to 3x)
- Flow states: Building → Flowing → Transcendent
- WPM and accuracy tracking
- Visual feedback for typing performance

**UI Polish (Phase 4)**
- Consistent visual theme with semantic colors
- 40+ Nerd Font icons throughout
- Contextual help system with tips
- Interactive tutorial for new players

**Narrative Foundation**
- Five factions with distinct philosophies
- Lore fragments discoverable through gameplay
- NPC encounters with authored dialogue
- Mystery progression system (framework)

### In Progress

- Save/load functionality
- Full meta-progression loop
- Sound effects
- Content expansion and balancing

---

## Project Structure

```
typingquest/
├── src/
│   ├── main.rs              # Game loop, input handling (~600 lines)
│   ├── game/
│   │   ├── state.rs         # Core game state
│   │   ├── combat.rs        # Combat mechanics
│   │   ├── combat_engine.rs # Combat calculation
│   │   ├── player.rs        # Player/class definitions
│   │   ├── enemy.rs         # Enemy definitions
│   │   ├── dungeon.rs       # Floor/room generation
│   │   ├── items.rs         # Item system
│   │   ├── spells.rs        # Spell system
│   │   ├── typing_feel.rs   # Combo/flow/feedback (~550 lines)
│   │   ├── tutorial.rs      # Tutorial system (~650 lines)
│   │   ├── help_system.rs   # Help overlay (~750 lines)
│   │   └── ...              # Narrative, factions, events
│   ├── ui/
│   │   ├── render.rs        # All screen rendering (~1300 lines)
│   │   └── theme.rs         # Colors, icons, styles (~350 lines)
│   └── data/
│       └── words.rs         # Word lists
├── Cargo.toml
├── CHANGELOG.md
└── README.md
```

**~21,500 lines of Rust** across 35+ source files.

---

## Roadmap

### v0.3.0 — Persistence
- [ ] Save/load game state
- [ ] Meta-progression currency (Ink) persistence
- [ ] Settings configuration

### v0.4.0 — Content
- [ ] More enemy variety per floor
- [ ] Additional spells and items
- [ ] Expanded event encounters
- [ ] Achievement tracking

### v0.5.0 — Balance
- [ ] Difficulty tuning
- [ ] Class balance pass
- [ ] Pacing adjustments

### v1.0.0 — Release
- [ ] Complete 10-floor campaign
- [ ] Multiple endings
- [ ] Full documentation

---

## Building

```bash
# Development build
cargo build

# Release build (optimized, ~1.5MB binary)
cargo build --release

# Run directly
cargo run --release

# Check for errors without building
cargo check
```

---

## Contributing

TypingQuest is a personal project in active development. Issues and suggestions welcome.

To contribute:
1. Open an issue describing the proposed change
2. Fork and create a feature branch
3. Submit a focused PR with clear description

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Credits

**TypingQuest** — Original work by Dr. Baklava

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [Nerd Fonts](https://www.nerdfonts.com/) — Icons

Inspired by:
- [ttyper](https://github.com/max-niederman/ttyper) — Terminal typing
- *Undertale* — Personality and charm
- *Hades* — Meta-progression
- *Balatro* — Satisfying feedback

---

*󰩛 Dr. Baklava was here*
