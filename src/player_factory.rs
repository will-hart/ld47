use spectre_core::*;

use crate::{components::*, constants::MELEE_RANGE};

pub fn get_player(player_id: u8, lane: usize) -> PlayerBundle {
    PlayerBundle {
        health: Health::new(100., 0.),
        mana: Mana::new(200.),
        movement: Movement {
            movement_speed: BuffableStatistic::new(50.),
        },
        player: Player {
            player_id,
            current_lane: lane,
            target_lane: lane,
        },
        attack: BaseAttack {
            attack_range: MELEE_RANGE,
            attack_speed: BuffableStatistic::new(1.5),
            min_attack_damage: 10,
            max_attack_damage: 15,
            crit_chance: 0.05,
            ..Default::default()
        },
        defence: Defence {
            base_armour: 1,
            ..Default::default()
        },
        // attack_target: AttackTarget::default(),
    }
}
