# Changelog

All notable changes to TypingQuest will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-01-18

### 🎭 Deep Narrative Systems
- **DeepLore System** (`deep_lore.rs`, ~700 lines)
  - Complete cosmology: The Silence, The Syntax, the world's creation
  - Five faction histories with detailed backstories
  - Player mystery: discover why you arrived at the threshold
  - 12 unique endings based on faction allegiance and choices
  - Hidden truth revelation system with 5 tiers

- **Lore Fragments** (`lore_fragments.rs`, ~500 lines)
  - 13+ authored lore fragments discoverable through gameplay
  - Fragment types: Inscription, Prophecy, Memory, Teaching, Warning
  - Rarity system affecting discovery chance
  - Fragments persist in Lore Codex across runs

- **Encounter Writing** (`encounter_writing.rs`, ~600 lines)
  - 8+ fully authored NPC encounters with branching dialogue
  - Characters: Silent Keeper Mira, Void Scholar Kael, etc.
  - Location-specific atmospheric writing
  - Multiple dialogue options with faction consequences

- **Writing Guidelines** (`writing_guidelines.rs`, ~400 lines)
  - Literary standards for consistent tone
  - Location-specific atmosphere rules
  - Character voice patterns

### 🔗 Narrative Integration
- **NarrativeEngine** (`narrative_integration.rs`, ~550 lines)
  - Coordinates all story systems into unified experience
  - Chapter progression: Awakening → Discovery → Revelation → Allegiance → Conflict → Reckoning
  - Weighted encounter selection based on story state
  - Mystery progress tracking with revelation thresholds
  - Faction reputation with consequence events at thresholds

### ⌨️ Typing Feel Engine
- **TypingFeel System** (`typing_feel.rs`, ~450 lines)
  - Flow states: Building → Flowing → Transcendent
  - Combo system with decay timer (up to 3x multiplier)
  - Rhythm detection analyzing keystroke cadence
  - Critical hit chance tied to flow state (30% in Transcendent)
  - Visual effects: screen shake, color flash, text ripple
  - Perfect word tracking for bonus damage

### 🏆 Meta-Progression (Hades-Style)
- **MetaProgress System** (`meta_progression.rs`, ~650 lines)
  - Ink currency earned from every run (persists through death)
  - 7-node unlock tree: HP bonus, gold bonus, time bonus, damage bonus, word preview, map reveal, dialogue memory
  - Lore Codex persists collected fragments across runs
  - NPC Bond levels: Stranger → Acquaintance → Familiar → Friend → Bonded
  - 10+ achievements: speed_demon, perfectionist, pacifist, true_ending, etc.
  - Heat system for increased difficulty with better rewards

### 🏛️ Enhanced Faction System
- **Five Factions** with deep relationships:
  - The Silent Order (monks of careful observation)
  - The Echoing Choir (prophets speaking impossible truths)
  - The Gilded Merchants (traders in dangerous knowledge)
  - The Threshold Wardens (guardians against the void)
  - The Void Touched (those who embraced dissolution)
- Faction reputation affects available encounters and endings
- Cross-faction relationships (allies, rivals, enemies)

### 🎨 Voice System
- **15+ Authored NPCs** (`voice_system.rs`, ~800 lines)
  - Each character has distinct vocabulary, cadence, concerns
  - Dialogue adapts based on faction standing and mystery progress
  - Memorable characters with recurring appearances

### Technical
- **~20,000 lines of Rust** (up from ~8,000 in 0.1.0)
- **35+ source files** with clean module organization
- **9 major interconnected systems**
- All systems compile cleanly with no warnings

## [0.1.0] - 2025-01-17

### Added
- Initial release
- Core game loop with scene-based state management
- 5 playable classes: Wordsmith, Scribe, Spellweaver, Barbarian, Trickster
- Combat system with typing-based attacks
- Dungeon exploration with procedural room generation
- Room types: Combat, Elite, Boss, Treasure, Shop, Rest, Event
- Item system with consumables, equipment, and relics
- Spell system with elemental magic
- Event system with branching choices
- Skill trees and character progression
- Narrative system with factions and dialogue
- Quest system
- TUI rendering with ratatui

### Technical
- Rust 2021 edition
- ratatui 0.28 for terminal UI
- crossterm 0.28 for input/terminal control
- serde for serialization
- Release binary: ~1.4MB (optimized + stripped)
