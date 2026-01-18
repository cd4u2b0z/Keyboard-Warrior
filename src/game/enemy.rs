//! Enemy definitions - Data-driven with Undertale/Earthbound flair!

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::sync::Arc;
use crate::data::{GameData, enemies::EnemyTemplate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub max_hp: i32,
    pub current_hp: i32,
    pub attack_power: i32,
    pub defense: i32,
    pub xp_reward: i32,
    pub gold_reward: i32,
    pub enemy_type: EnemyType,
    pub ascii_art: String,
    pub battle_cry: String,
    pub defeat_message: String,
    pub spare_condition: Option<String>,
    pub is_boss: bool,
    pub typing_theme: String,
    pub attack_messages: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyType {
    Normal,
    Elite,
    Boss,
}

impl Enemy {
    /// Create an enemy from a data template, scaled for floor
    pub fn from_template(template: &EnemyTemplate, floor: i32) -> Self {
        let scale = 1.0 + (floor as f32 - 1.0) * 0.1;
        Self {
            name: template.name.clone(),
            max_hp: (template.base_hp as f32 * scale) as i32,
            current_hp: (template.base_hp as f32 * scale) as i32,
            attack_power: (template.base_damage as f32 * scale) as i32,
            defense: (template.base_defense as f32 * scale) as i32,
            xp_reward: (template.xp_reward as f32 * scale) as i32,
            gold_reward: (template.gold_reward as f32 * scale) as i32,
            enemy_type: EnemyType::Normal,
            ascii_art: template.ascii_art.clone(),
            battle_cry: format!("* {} blocks your path!", template.name),
            defeat_message: template.death_message.clone(),
            spare_condition: None,
            is_boss: false,
            typing_theme: template.typing_theme.clone(),
            attack_messages: template.attack_messages.clone(),
        }
    }

    /// Spawn a random enemy appropriate for the floor using GameData
    pub fn random_for_floor_data(game_data: &GameData, floor: i32) -> Self {
        let tier = ((floor - 1) / 2 + 1).clamp(1, 7) as u32;
        let enemies = game_data.enemies.get_enemies_by_tier(tier);
        
        if enemies.is_empty() {
            // Fallback to hardcoded if no data
            return Self::random_for_floor(floor);
        }
        
        let mut rng = rand::thread_rng();
        let template = enemies.choose(&mut rng).unwrap();
        Self::from_template(template, floor)
    }

    /// Spawn an elite enemy using GameData
    pub fn random_elite_data(game_data: &GameData, floor: i32) -> Self {
        let mut enemy = Self::random_for_floor_data(game_data, floor);
        enemy.name = format!("Elite {}", enemy.name);
        enemy.max_hp = (enemy.max_hp as f32 * 1.5) as i32;
        enemy.current_hp = enemy.max_hp;
        enemy.attack_power = (enemy.attack_power as f32 * 1.3) as i32;
        enemy.xp_reward = (enemy.xp_reward as f32 * 2.0) as i32;
        enemy.gold_reward = (enemy.gold_reward as f32 * 2.0) as i32;
        enemy.enemy_type = EnemyType::Elite;
        enemy
    }

    /// Spawn a boss using GameData
    pub fn random_boss_data(game_data: &GameData, floor: i32) -> Self {
        let bosses: Vec<_> = game_data.enemies.bosses.values().collect();
        
        if bosses.is_empty() {
            return Self::random_boss(floor);
        }
        
        let mut rng = rand::thread_rng();
        let boss = bosses.choose(&mut rng).unwrap();
        let scale = 1.0 + (floor as f32 - 1.0) * 0.15;
        
        Self {
            name: boss.name.clone(),
            max_hp: (boss.base_hp as f32 * scale) as i32,
            current_hp: (boss.base_hp as f32 * scale) as i32,
            attack_power: (boss.base_damage as f32 * scale) as i32,
            defense: (boss.base_defense as f32 * scale) as i32,
            xp_reward: (boss.xp_reward as f32 * scale) as i32,
            gold_reward: (boss.gold_reward as f32 * scale) as i32,
            enemy_type: EnemyType::Boss,
            ascii_art: boss.ascii_art.clone(),
            battle_cry: boss.intro_dialogue.first()
                .cloned()
                .unwrap_or_else(|| format!("* {} awakens!", boss.name)),
            defeat_message: boss.death_dialogue.last()
                .cloned()
                .unwrap_or_else(|| format!("* {} has been defeated!", boss.name)),
            spare_condition: None,
            is_boss: true,
            typing_theme: "corruption".to_string(),
            attack_messages: boss.phase_transition_dialogue.clone(),
        }
    }

    // === Legacy methods for backwards compatibility ===
    
    pub fn random_for_floor(floor: i32) -> Self {
        let mut rng = rand::thread_rng();
        let pool = Self::get_enemy_pool(floor);
        pool.choose(&mut rng).unwrap().clone()
    }

    pub fn random_elite(floor: i32) -> Self {
        let mut enemy = Self::random_for_floor(floor);
        enemy.name = format!("Elite {}", enemy.name);
        enemy.max_hp = (enemy.max_hp as f32 * 1.5) as i32;
        enemy.current_hp = enemy.max_hp;
        enemy.attack_power = (enemy.attack_power as f32 * 1.3) as i32;
        enemy.xp_reward = (enemy.xp_reward as f32 * 2.0) as i32;
        enemy.gold_reward = (enemy.gold_reward as f32 * 2.0) as i32;
        enemy.enemy_type = EnemyType::Elite;
        enemy
    }

    pub fn random_boss(floor: i32) -> Self {
        let mut rng = rand::thread_rng();
        let pool = Self::get_boss_pool(floor);
        pool.choose(&mut rng).unwrap().clone()
    }

    pub fn get_attack_message(&self) -> &str {
        if !self.attack_messages.is_empty() {
            let mut rng = rand::thread_rng();
            return self.attack_messages.choose(&mut rng)
                .map(|s| s.as_str())
                .unwrap_or("attacks");
        }
        
        let messages = [
            "attacks",
            "strikes",
            "hits you",
            "lunges at you",
        ];
        let mut rng = rand::thread_rng();
        messages.choose(&mut rng).unwrap()
    }

    fn get_enemy_pool(floor: i32) -> Vec<Self> {
        // Zone-appropriate fantasy enemies with balanced stats
        let shattered_halls_enemies = vec![
            Enemy {
                name: "Goblin Lurker".to_string(),
                max_hp: 25 + (floor * 3),
                current_hp: 25 + (floor * 3),
                attack_power: 4 + floor,
                defense: 1,
                xp_reward: 12 + (floor * 2) as i32,
                gold_reward: 8 + (floor * 2),
                enemy_type: EnemyType::Normal,
                ascii_art: "  ,--.\n  (o.o)\n  /|░|\\".to_string(),
                battle_cry: "* Shiny things! Give them!".to_string(),
                defeat_message: "* The goblin falls with a pitiful screech.".to_string(),
                spare_condition: Some("Offer gold to flee".to_string()),
                is_boss: false,
                typing_theme: "fantasy".to_string(),
                attack_messages: vec!["lunges with a rusty dagger".to_string(), "throws a rock".to_string()],
            },
            Enemy {
                name: "Hollow Knight".to_string(),
                max_hp: 35 + (floor * 4),
                current_hp: 35 + (floor * 4),
                attack_power: 5 + floor,
                defense: 3,
                xp_reward: 15 + (floor * 2) as i32,
                gold_reward: 12 + (floor * 2),
                enemy_type: EnemyType::Normal,
                ascii_art: "  [╦╦]\n  |██|\n  /  \\".to_string(),
                battle_cry: "* For the fallen kingdom...".to_string(),
                defeat_message: "* The armor clatters empty to the floor.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "fantasy".to_string(),
                attack_messages: vec!["swings a notched blade".to_string(), "charges shield-first".to_string()],
            },
            Enemy {
                name: "Wailing Wraith".to_string(),
                max_hp: 20 + (floor * 2),
                current_hp: 20 + (floor * 2),
                attack_power: 6 + floor,
                defense: 0,
                xp_reward: 14 + (floor * 2) as i32,
                gold_reward: 6 + floor,
                enemy_type: EnemyType::Normal,
                ascii_art: " ~░░░~\n  (○○)\n  ~~~~".to_string(),
                battle_cry: "* Whyyyyy...".to_string(),
                defeat_message: "* The wraith fades with a final mournful wail.".to_string(),
                spare_condition: Some("Listen to its sorrows".to_string()),
                is_boss: false,
                typing_theme: "dark".to_string(),
                attack_messages: vec!["wails despairingly".to_string(), "reaches with spectral claws".to_string()],
            },
        ];

        let sunken_archives_enemies = vec![
            Enemy {
                name: "Spectral Wisp".to_string(),
                max_hp: 22 + (floor * 3),
                current_hp: 22 + (floor * 3),
                attack_power: 5 + floor,
                defense: 1,
                xp_reward: 14 + (floor * 2) as i32,
                gold_reward: 10 + floor,
                enemy_type: EnemyType::Normal,
                ascii_art: "   *\n  ░█░\n   *".to_string(),
                battle_cry: "* Knowledge... must be... protected...".to_string(),
                defeat_message: "* The wisp dissipates into ethereal mist.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "arcane".to_string(),
                attack_messages: vec!["hurls arcane sparks".to_string(), "pulses with cold light".to_string()],
            },
            Enemy {
                name: "Drowned Scholar".to_string(),
                max_hp: 38 + (floor * 4),
                current_hp: 38 + (floor * 4),
                attack_power: 6 + floor,
                defense: 2,
                xp_reward: 18 + (floor * 2) as i32,
                gold_reward: 15 + (floor * 2),
                enemy_type: EnemyType::Normal,
                ascii_art: "  [○○]\n  ╔══╗\n  ║~~║".to_string(),
                battle_cry: "* The texts... I must finish reading...".to_string(),
                defeat_message: "* Finally... rest...".to_string(),
                spare_condition: Some("Return its lost tome".to_string()),
                is_boss: false,
                typing_theme: "arcane".to_string(),
                attack_messages: vec!["casts a waterlogged spell".to_string(), "throws a soggy book".to_string()],
            },
            Enemy {
                name: "Stone Golem".to_string(),
                max_hp: 55 + (floor * 5),
                current_hp: 55 + (floor * 5),
                attack_power: 4 + floor,
                defense: 6,
                xp_reward: 22 + (floor * 3) as i32,
                gold_reward: 20 + (floor * 2),
                enemy_type: EnemyType::Normal,
                ascii_art: "  ╔█╗\n  ███\n  █ █".to_string(),
                battle_cry: "* PROTECT... ARCHIVES...".to_string(),
                defeat_message: "* The golem crumbles into inert rubble.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "fantasy".to_string(),
                attack_messages: vec!["swings a massive fist".to_string(), "stomps the ground".to_string()],
            },
        ];

        let blighted_gardens_enemies = vec![
            Enemy {
                name: "Venomous Spider".to_string(),
                max_hp: 30 + (floor * 3),
                current_hp: 30 + (floor * 3),
                attack_power: 7 + floor,
                defense: 1,
                xp_reward: 16 + (floor * 2) as i32,
                gold_reward: 8 + floor,
                enemy_type: EnemyType::Normal,
                ascii_art: " /\\○/\\\n  ████\n /    \\".to_string(),
                battle_cry: "* Skkkkktttt...".to_string(),
                defeat_message: "* The spider curls and goes still.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "nature".to_string(),
                attack_messages: vec!["spits venom".to_string(), "lunges with fangs bared".to_string()],
            },
            Enemy {
                name: "Blighted Thrall".to_string(),
                max_hp: 40 + (floor * 4),
                current_hp: 40 + (floor * 4),
                attack_power: 6 + floor,
                defense: 2,
                xp_reward: 18 + (floor * 2) as i32,
                gold_reward: 12 + (floor * 2),
                enemy_type: EnemyType::Normal,
                ascii_art: "  ░█░\n  ╠█╣\n  ╨ ╨".to_string(),
                battle_cry: "* Join... us... in the... blight...".to_string(),
                defeat_message: "* The thrall crumbles, finally at peace.".to_string(),
                spare_condition: Some("Cure the corruption".to_string()),
                is_boss: false,
                typing_theme: "dark".to_string(),
                attack_messages: vec!["claws with corrupted hands".to_string(), "exhales toxic spores".to_string()],
            },
            Enemy {
                name: "Twisted Treant".to_string(),
                max_hp: 50 + (floor * 5),
                current_hp: 50 + (floor * 5),
                attack_power: 5 + floor,
                defense: 4,
                xp_reward: 20 + (floor * 3) as i32,
                gold_reward: 18 + (floor * 2),
                enemy_type: EnemyType::Normal,
                ascii_art: " ╔░░╗\n ║██║\n ╠╬╬╣".to_string(),
                battle_cry: "* The corruption... it BURNS...".to_string(),
                defeat_message: "* The twisted bark splits, releasing a sigh of relief.".to_string(),
                spare_condition: Some("Purify its roots".to_string()),
                is_boss: false,
                typing_theme: "nature".to_string(),
                attack_messages: vec!["lashes with thorned vines".to_string(), "drops corrupted sap".to_string()],
            },
        ];

        let clockwork_depths_enemies = vec![
            Enemy {
                name: "Clockwork Sentinel".to_string(),
                max_hp: 45 + (floor * 4),
                current_hp: 45 + (floor * 4),
                attack_power: 7 + floor,
                defense: 5,
                xp_reward: 22 + (floor * 3) as i32,
                gold_reward: 20 + (floor * 3),
                enemy_type: EnemyType::Normal,
                ascii_art: " ╔═⚙═╗\n ║ ◊ ║\n ╚═╬═╝".to_string(),
                battle_cry: "* INTRUDER DETECTED. ELIMINATING.".to_string(),
                defeat_message: "* Gears grind to a halt. Steam hisses.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "technology".to_string(),
                attack_messages: vec!["fires a steam bolt".to_string(), "swings a mechanical arm".to_string()],
            },
            Enemy {
                name: "Void Walker".to_string(),
                max_hp: 35 + (floor * 4),
                current_hp: 35 + (floor * 4),
                attack_power: 9 + floor,
                defense: 2,
                xp_reward: 25 + (floor * 3) as i32,
                gold_reward: 22 + (floor * 3),
                enemy_type: EnemyType::Normal,
                ascii_art: "  ◇◇◇\n  ░█░\n  ▼ ▼".to_string(),
                battle_cry: "* The void... calls...".to_string(),
                defeat_message: "* The walker fades back into the darkness.".to_string(),
                spare_condition: Some("Show it the light".to_string()),
                is_boss: false,
                typing_theme: "dark".to_string(),
                attack_messages: vec!["strikes from the shadows".to_string(), "drains your essence".to_string()],
            },
        ];

        let voids_edge_enemies = vec![
            Enemy {
                name: "Shadow Weaver".to_string(),
                max_hp: 42 + (floor * 5),
                current_hp: 42 + (floor * 5),
                attack_power: 10 + floor,
                defense: 3,
                xp_reward: 28 + (floor * 3) as i32,
                gold_reward: 25 + (floor * 3),
                enemy_type: EnemyType::Normal,
                ascii_art: " ∿∿∿∿\n (◆◆)\n ~~~~".to_string(),
                battle_cry: "* Your fate is already woven...".to_string(),
                defeat_message: "* The weaver's shadows disperse into nothing.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "dark".to_string(),
                attack_messages: vec!["entangles you in shadow threads".to_string(), "whispers doom".to_string()],
            },
            Enemy {
                name: "Soul Devourer".to_string(),
                max_hp: 50 + (floor * 5),
                current_hp: 50 + (floor * 5),
                attack_power: 11 + floor,
                defense: 4,
                xp_reward: 32 + (floor * 4) as i32,
                gold_reward: 28 + (floor * 4),
                enemy_type: EnemyType::Normal,
                ascii_art: "  ╔▓▓╗\n  ║◊◊║\n  ╚▼▼╝".to_string(),
                battle_cry: "* Your soul... smells... delicious...".to_string(),
                defeat_message: "* The devourer releases its stolen souls in a blinding flash.".to_string(),
                spare_condition: Some("Offer a fragment of your soul".to_string()),
                is_boss: false,
                typing_theme: "dark".to_string(),
                attack_messages: vec!["tears at your essence".to_string(), "feeds on your fear".to_string()],
            },
            Enemy {
                name: "Death Knight".to_string(),
                max_hp: 60 + (floor * 6),
                current_hp: 60 + (floor * 6),
                attack_power: 12 + floor,
                defense: 6,
                xp_reward: 35 + (floor * 4) as i32,
                gold_reward: 30 + (floor * 4),
                enemy_type: EnemyType::Normal,
                ascii_art: " ╔═╦═╗\n ║▓█▓║\n ║ ▼ ║".to_string(),
                battle_cry: "* In death, I serve still.".to_string(),
                defeat_message: "* The knight kneels, finally released from duty.".to_string(),
                spare_condition: Some("Speak its true name".to_string()),
                is_boss: false,
                typing_theme: "dark".to_string(),
                attack_messages: vec!["cleaves with a cursed blade".to_string(), "summons dark fire".to_string()],
            },
        ];

        // Match enemies to zones based on floor
        match floor {
            1..=2 => shattered_halls_enemies,
            3..=4 => sunken_archives_enemies,
            5..=6 => blighted_gardens_enemies,
            7..=8 => clockwork_depths_enemies,
            _ => voids_edge_enemies,
        }
    }

    fn get_boss_pool(floor: i32) -> Vec<Self> {
        match floor {
            1..=5 => vec![
                Enemy {
                    name: "The Hollow Knight".to_string(),
                    max_hp: 120 + (floor * 10),
                    current_hp: 120 + (floor * 10),
                    attack_power: 8 + floor,
                    defense: 5,
                    xp_reward: 100,
                    gold_reward: 75,
                    enemy_type: EnemyType::Boss,
                    ascii_art: "    ╔═══╗\n    ║ ◆ ║\n   ╔╩═══╩╗\n   ║ ███ ║\n   ╚══╬══╝\n      █\n     ╱ ╲".to_string(),
                    battle_cry: "* I am the last defender of this fallen kingdom.".to_string(),
                    defeat_message: "* At last... my watch... ends...".to_string(),
                    spare_condition: Some("Prove your worth through honor".to_string()),
                    is_boss: true,
                    typing_theme: "fantasy".to_string(),
                    attack_messages: vec![
                        "charges with spectral lance".to_string(),
                        "unleashes a devastating combo".to_string(),
                        "calls upon ancient oaths".to_string(),
                    ],
                },
            ],
            _ => vec![
                Enemy {
                    name: "The Void Herald".to_string(),
                    max_hp: 200 + (floor * 15),
                    current_hp: 200 + (floor * 15),
                    attack_power: 12 + floor,
                    defense: 8,
                    xp_reward: 250,
                    gold_reward: 150,
                    enemy_type: EnemyType::Boss,
                    ascii_art: "      ████████\n    ██░░░░░░░░██\n   ██░░◆░░░░◆░░██\n  ██░░░░░▼░░░░░██\n   ██░░~~~~~░░██\n    ██░░░░░░░░██\n      ████████".to_string(),
                    battle_cry: "* I am the herald of the end. The Sundering continues through me.".to_string(),
                    defeat_message: "* The void... recedes... but it will... return...".to_string(),
                    spare_condition: None,
                    is_boss: true,
                    typing_theme: "dark".to_string(),
                    attack_messages: vec![
                        "tears reality asunder".to_string(),
                        "speaks in the language of endings".to_string(),
                        "channels the power of the Sundering".to_string(),
                        "erases meaning from existence".to_string(),
                    ],
                },
            ],
        }
    }
}
