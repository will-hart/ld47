use spectre_core::*;

use crate::{components::*, constants::MELEE_RANGE};

pub fn get_player(player_id: u8, lane: usize) -> PlayerBundle {
    PlayerBundle {
        health: Health::new(100., 0.),
        mana: Mana::new(200.),
        movement: Movement {
            movement_speed: BuffableStatistic::new(75.),
        },
        player: Player {
            player_id,
            current_lane: lane,
            target_lane: lane,
            is_moving: false,
            abilities: vec![],
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
            base_armour: BuffableStatistic::new(1.),
            ..Default::default()
        },
        actions: PlayerAbilityActions {
            actions: vec![
                AbilityActionDetails {
                    action: None,
                    next_available: f32::MAX,
                },
                AbilityActionDetails {
                    action: None,
                    next_available: f32::MAX,
                },
            ],
        }, // attack_target: AttackTarget::default(),
    }
}
