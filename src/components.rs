use bevy::prelude::*;
use spectre_core::*;

use crate::constants::TARGET_LOCATIONS;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub attack: BaseAttack,
    pub defence: Defence,
    pub enemy: Enemy,
    pub health: Health,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub stats: Stats,
    pub movement: Movement,
    pub health: Health,
    pub mana: Mana,
    pub player: Player,
}

/// Tag component applied when a unit has its movement easing applied
pub struct HasEaseApplied;

/// Contains data about an enemy unit
pub struct Enemy {
    pub lane: usize,
    pub target: Vec2,
}

impl Enemy {
    pub fn new(lane: usize) -> Self {
        let target_loc = TARGET_LOCATIONS[lane];
        Enemy {
            lane,
            target: Vec2::new(target_loc.0, target_loc.1),
        }
    }
}

pub struct Player {
    pub current_lane: usize,
    pub target_lane: usize,
}

/// The base attack / defence of a unit. Can be enhanced over time
#[derive(Default)]
pub struct BaseAttack {
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
    pub base_armour: i16,
    pub fire_armour: i32,
    pub electricity_armour: i32,
    pub poison_armour: i32,
    pub frost_armour: i32,
}
