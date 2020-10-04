use super::*;
use crate::components::*;
use ability_data::AbilityDatabase;
use bevy::prelude::*;
use spectre_core::{Health, Mana, Movement};

pub fn ability_purchase_system(
    mut commands: Commands,
    mut abilities: ResMut<AbilityDatabase>,
    mut player_score: ResMut<PlayerScore>,
    mut purchase_requests: Query<(Entity, &AbilityPurchaseRequest)>,
    mut players: Query<(
        &mut Player,
        &mut Defence,
        &mut Health,
        &mut Mana,
        &mut Movement,
    )>,
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
        for (mut player, mut defence, mut health, mut mana, mut movement) in &mut players.iter() {
            if player.player_id != request.player_id {
                continue;
            }

            // check the player meets the prerequisites
            let can_unlock = ability
                .prerequisites
                .iter()
                .all(|pre| player.abilities.contains(pre));

            if !can_unlock {
                println!(
                    "Player {} does not have the correct prerequisite abilities to unlock {}",
                    request.player_id, request.ability_id
                );
                break;
            }

            // register the ability
            player.abilities.push(ability.id);

            // apply the effects
            for effect in ability.effects.iter() {
                match effect {
                    AbilityDetail::Attack(_) => {
                        panic!("Not implemented");
                    }
                    AbilityDetail::Buff(detail) => {
                        match detail.buff_type {
                            BuffType::Armour => {
                                defence.base_armour.buffs.push(detail.buff.clone());
                            }
                            BuffType::Health => {
                                health.max_health.buffs.push(detail.buff.clone());
                            }
                            BuffType::Mana => {
                                mana.max_mana.buffs.push(detail.buff.clone());
                            }
                            BuffType::Regeneration => {
                                health.regeneration.buffs.push(detail.buff.clone());
                            }
                            BuffType::MovementSpeed => {
                                movement.movement_speed.buffs.push(detail.buff.clone());
                            }
                        };
                    }
                    AbilityDetail::Heal(_) => {
                        panic!("Not implemented");
                    }
                    AbilityDetail::Revive(_) => {
                        panic!("Not implemented");
                    }
                };
            }

            // mark it as applied
            player_score.0 -= ability.xp_cost;
            found = true;
            break;
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
