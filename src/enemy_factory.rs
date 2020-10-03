use crate::components::*;
/// Do this from data files in a real game
use spectre_core::{BuffableStatistic, Health};

pub fn get_wolf() -> EnemyBundle {
    EnemyBundle {
        enemy: Enemy::new(0),
        health: Health {
            max_health: BuffableStatistic::new(10.),
            current_health: 10.,
            target_health: 10.,
            regeneration: 0.,
        },
        attack: BaseAttack {
            min_attack_damage: 3,
            max_attack_damage: 5,
            ..Default::default()
        },
        defence: Defence {
            base_armour: 0,
            ..Default::default()
        },
    }
}
