/// Do this from data files in a real game
use crate::{components::*, constants::MELEE_RANGE};
use spectre_core::{BuffableStatistic, Health};

pub enum EnemyType {
    Wolf,
}

pub fn get_enemy_bundle(enemy_type: EnemyType, lane: usize) -> EnemyBundle {
    match enemy_type {
        EnemyType::Wolf => get_wolf(lane),
    }
}

fn get_wolf(lane: usize) -> EnemyBundle {
    EnemyBundle {
        enemy: Enemy::new(lane),
        health: Health::new(30., 0.),
        attack: BaseAttack {
            attack_range: MELEE_RANGE,
            attack_speed: BuffableStatistic::new(1.),
            min_attack_damage: 3,
            max_attack_damage: 5,
            ..Default::default()
        },
        defence: Defence {
            base_armour: 0,
            ..Default::default()
        },
        attack_target: AttackTarget::default(),
    }
}
