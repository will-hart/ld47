use bevy::prelude::*;
use spectre_core::*;

use crate::constants::*;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub attack: BaseAttack,
    pub defence: Defence,
    pub enemy: Enemy,
    pub health: Health,
    pub attack_target: AttackTarget,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub movement: Movement,
    pub health: Health,
    pub mana: Mana,
    pub attack: BaseAttack,
    pub defence: Defence,
    pub player: Player,
    // pub attack_target: AttackTarget, // TODO: post jam
}

/// Contains data about an enemy unit
pub struct Enemy {
    pub lane: usize,
    pub target: Vec2,
    pub xp_reward: usize,
}

impl Enemy {
    pub fn new(lane: usize, xp_reward: usize) -> Self {
        let target_loc = Vec2::from(TARGET_LOCATIONS[lane]);
        Enemy {
            lane,
            target: target_loc,
            xp_reward,
        }
    }
}

pub struct Player {
    pub player_id: u8,
    pub current_lane: usize,
    pub target_lane: usize,
    pub is_moving: bool,
    pub abilities: Vec<u16>,
}

impl Player {
    pub fn get_next_level(&self) -> Option<u16> {
        if self.abilities.len() == 0 {
            return Some(2);
        }

        if self.abilities.contains(&2) {
            return None;
        }

        return Some(3);
    }
}

/// The base attack / defence of a unit. Can be enhanced over time
#[derive(Default)]
pub struct BaseAttack {
    pub attack_range: f32,
    pub attack_speed: BuffableStatistic,
    pub next_attack: f32,
    pub min_attack_damage: i32,
    pub max_attack_damage: i32,
    pub crit_chance: f32,
    pub fire_damage: i32,
    pub electricity_damage: i32,
    pub poison_damage: i32,
    pub frost_damage: i32,
}

#[derive(Default)]
pub struct Defence {
    pub base_armour: BuffableStatistic,
    pub fire_armour: i32,
    pub electricity_armour: i32,
    pub poison_armour: i32,
    pub frost_armour: i32,
}

pub struct HealthBar {
    pub entity: Entity,
}

pub struct AttackTarget {
    pub entity: Option<Entity>,
    pub is_obelisk: bool,
}

impl Default for AttackTarget {
    fn default() -> Self {
        AttackTarget {
            entity: None,
            is_obelisk: false,
        }
    }
}

/// A resource to store what wave is currently active
pub struct CurrentWave {
    // the index of the wave to spawn
    pub wave_idx: usize,

    // the next time a wave is due to be spawned
    pub next_wave_time: f32,
}

impl FromResources for CurrentWave {
    fn from_resources(_: &Resources) -> Self {
        CurrentWave {
            wave_idx: 0,
            next_wave_time: 1.,
        }
    }
}

/// Used to flag a text box that is linked to a player's health
pub struct PlayerHealthLink {
    pub player_id: u8,
    pub entity: Option<Entity>,
}

/// Used to flag a text box that is linked to a player's health
pub struct PlayerManaLink {
    pub player_id: u8,
    pub entity: Option<Entity>,
}

/// Links player lane change button to a change lanes action
pub struct PlayerLaneChangeLink {
    pub player_id: u8,
    pub delta: i8,
}

pub struct ObeliskStatusTextUiLink;
pub struct ObeliskStatusImageUiLink;

pub struct PlayerScore {
    pub xp: usize,
    pub obelisk_health: usize,
    pub last_obelisk_damage: f32,
}

impl FromResources for PlayerScore {
    fn from_resources(_: &Resources) -> Self {
        PlayerScore {
            xp: 0,
            obelisk_health: 1000,
            last_obelisk_damage: 0.,
        }
    }
}

/// Used to flag an item for destruction when the game is despawned
pub struct GameSceneEntity;

#[derive(Default)]
pub struct GameSceneConfigured(pub bool);

pub struct PlayerAbilityButtonInteraction {
    pub player_id: u8,
    pub action_number: u8,
}

/// Flags these components as displayed while the game is running
pub struct GameRunningPlayerUi;

/// Flags the main game sidebar GUI, where the ability menu is swapped in
pub struct MainGameSidebarUi;

/// Flags a button to close ability screen and transition back to the game screen
pub struct CloseAbilitiesButtonLink;

/// Used to trigger a purchase when an ability button is clicked
pub struct AbilityPurchaseInteraction {
    pub player_id: u8,
    pub ability_id: u16,
}

#[derive(Default)]
pub struct Incapacitated {
    /// set to 0 to make indefinite
    pub end_time: f32,
    pub is_revived: bool,
}
