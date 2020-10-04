use super::*;
use crate::components::*;
use ability_data::AbilityDatabase;
use bevy::prelude::*;

pub fn ability_purchase_system(
    mut commands: Commands,
    mut abilities: ResMut<AbilityDatabase>,
    mut player_score: ResMut<PlayerScore>,
    mut purchase_requests: Query<(Entity, &AbilityPurchaseRequest)>,
    mut players: Query<&Player>,
) {
    for (ent, request) in &mut purchase_requests.iter() {
        let ability = abilities.get(request.ability_id);

        if ability.xp_cost > player_score.0 {
            println!(
                "Unable to purchase ability {} for player {}, too expensive",
                request.ability_id, request.player_id
            );

            commands.despawn(ent);
            continue;
        }

        let mut found: bool = false;
        for player in &mut players.iter() {
            if player.player_id != request.player_id {
                continue;
            }

            // check the player meets the prerequisites

            // apply the ability
        }

        if !found {
            println!(
                "Unable to find matching player for ability {} on player {}, skipping",
                request.ability_id, request.player_id
            );
        }

        commands.despawn(ent);
    }
}
