/// Do this from data files in a real game
use crate::{components::*, constants::MELEE_RANGE};
use spectre_core::{BuffableStatistic, Health};

pub enum EnemyType {
    Wolf,
    Bear,
}

pub fn get_enemy_bundle(enemy_type: EnemyType, lane: usize) -> EnemyBundle {
    match enemy_type {
        EnemyType::Wolf => get_wolf(lane),
        EnemyType::Bear => get_bear(lane),
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

fn get_bear(lane: usize) -> EnemyBundle {
    EnemyBundle {
        enemy: Enemy::new(lane),
        health: Health::new(50., 0.1),
        attack: BaseAttack {
            attack_range: MELEE_RANGE,
            attack_speed: BuffableStatistic::new(0.95),
            min_attack_damage: 9,
            max_attack_damage: 12,
            ..Default::default()
        },
        defence: Defence {
            base_armour: 1,
            ..Default::default()
        },
        attack_target: AttackTarget::default(),
    }
}
