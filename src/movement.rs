use bevy::prelude::*;
use spectre_core::Movement;
use spectre_time::GameTime;

use crate::{components::*, constants::*};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_movement.system())
            .add_system(enemy_movement.system());
    }
}

/// moves a player between lanes
/// ignores incapacitated players
pub fn player_movement(
    time: Res<GameTime>,
    mut players: Query<Without<Incapacitated, (&Movement, &mut Player, &mut Transform)>>,
) {
    for (movement, mut player, mut transform) in &mut players.iter() {
        // TODO: handle multiple players in the same lane properly
        let target_pos = TARGET_LOCATIONS[player.target_lane].0
            + (player.player_id as f32) * PLAYER_OFFSET_X
            - PLAYER_OFFSET_X;
        let current_pos = transform.translation().x();

        let delta = target_pos - current_pos;

        // tick over the current lane once the player arrives
        if delta.abs() < 3. {
            player.current_lane = player.target_lane;
            player.is_moving = false;
            continue;
        }

        let max_delta = movement.movement_speed.value * time.delta;

        // use abs as delta may be negative, i.e. -665 from target, max is 1.3
        let mut used_delta = delta.abs().min(max_delta.abs());
        if delta < 0. {
            used_delta = -used_delta;
        }

        // translate minimum of delta and max_delta
        player.is_moving = true;
        transform.translate(Vec3::new(used_delta, 0., 0.));
    }
}

/// moves an enemy towards their target position.
/// TODO: If there is a player in the lane stop at the target position otherwise continue off screen
pub fn enemy_movement(time: Res<GameTime>, enemy: &Enemy, mut transform: Mut<Transform>) {
    let target_pos = enemy.target.y();
    let current_pos = transform.translation().y();

    let delta = target_pos - current_pos;
    let max_delta = ENEMY_SPEED * time.delta;

    // use abs as delta may be negative, i.e. -665 from target, max is 1.3
    let mut used_delta = delta.abs().min(max_delta.abs());

    if delta < 0. {
        used_delta = -used_delta;
    }

    // translate minimum of delta and max_delta
    transform.translate(Vec3::new(0., used_delta, 0.));
}
