use spectre_core::*;

use crate::{components::*, constants::MELEE_RANGE};

pub fn get_player(lane: usize) -> PlayerBundle {
    PlayerBundle {
        health: Health::new(100.),
        mana: Mana::new(200.),
        movement: Movement {
            movement_speed: BuffableStatistic::new(50.),
        },
        player: Player {
            current_lane: lane,
            target_lane: lane,
        },
        attack: BaseAttack {
            attack_range: MELEE_RANGE,
            attack_speed: BuffableStatistic::new(1500.),
            min_attack_damage: 10,
            max_attack_damage: 15,
            ..Default::default()
        },
        defence: Defence {
            base_armour: 1,
            ..Default::default()
        },
    }
}
