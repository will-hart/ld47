use bevy::prelude::*;
use spectre_animations::AnimationState;
use spectre_core::Health;

use crate::components::*;

/// incapcitates players and shows incap animation when they die
pub fn player_incapacitation_system(
    mut commands: Commands,
    mut players: Query<Without<Incapacitated, (Entity, &Player, &mut AnimationState, &Health)>>,
) {
    for (ent, _, mut anim_state, health) in &mut players.iter() {
        if health.target_health > 0. {
            continue;
        }

        commands.insert_one(ent, Incapacitated::default());
        anim_state.set_animation(1); // incapacitated animation
    }
}

pub fn player_revival_system(
    mut commands: Commands,
    entity: Entity,
    incap: &Incapacitated,
    mut anim_state: Mut<AnimationState>,
    mut health: Mut<Health>,
) {
    if !incap.is_revived {
        return;
    }

    health.target_health = 0.5 * health.max_health.value;
    health.current_health = health.target_health;

    anim_state.set_animation(0);

    commands.remove_one::<Incapacitated>(entity);
}
