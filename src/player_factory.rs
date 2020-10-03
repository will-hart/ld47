use spectre_core::*;

use crate::components::{Player, PlayerBundle};

pub fn get_player(lane: usize) -> PlayerBundle {
    PlayerBundle {
        stats: Stats {
            strength: BuffableStatistic::new(10.),
            agility: BuffableStatistic::new(10.),
            intelligence: BuffableStatistic::new(10.),
            is_changed: true,
        },
        health: Health::new(100.),
        mana: Mana::new(200.),
        movement: Movement {
            movement_speed: BuffableStatistic::new(50.),
        },
        player: Player {
            current_lane: lane,
            target_lane: lane,
        },
    }
}
