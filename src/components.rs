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
    pub movement: Movement,
    pub health: Health,
    pub mana: Mana,
    pub attack: BaseAttack,
    pub defence: Defence,
    pub player: Player,
}

/// Contains data about an enemy unit
pub struct Enemy {
    pub lane: usize,
    pub target: Vec2,
}

impl Enemy {
    pub fn new(lane: usize) -> Self {
        let target_loc = Vec2::from(TARGET_LOCATIONS[lane]);
        Enemy {
            lane,
            target: target_loc,
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
    pub base_armour: i32,
    pub fire_armour: i32,
    pub electricity_armour: i32,
    pub poison_armour: i32,
    pub frost_armour: i32,
}
