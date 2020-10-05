use super::*;
use crate::{components::*, events::RedrawAbilityUiEvent};
use ability_data::AbilityDatabase;
use bevy::prelude::*;
use spectre_core::{Health, Mana, Movement};
use spectre_time::GameTime;

pub fn ability_purchase_system(
    mut commands: Commands,
    game_time: Res<GameTime>,
    mut abilities: ResMut<AbilityDatabase>,
    mut player_score: ResMut<PlayerScore>,
    mut ability_redraw_event: ResMut<Events<RedrawAbilityUiEvent>>,
    mut purchase_requests: Query<(Entity, &AbilityPurchaseRequest)>,
    mut players: Query<(
        &mut Player,
        &mut PlayerAbilityActions,
        &mut Defence,
        &mut Health,
        &mut Mana,
        &mut Movement,
    )>,
) {
    for (ent, request) in &mut purchase_requests.iter() {
        let ability = abilities.get(request.ability_id);

        if ability.xp_cost > player_score.xp {
            println!(
                "Unable to purchase ability {} for player {}, too expensive",
                request.ability_id, request.player_id
            );

            commands.despawn(ent);
            continue;
        }

        let mut found: bool = false;
        for (mut player, mut actions, mut defence, mut health, mut mana, mut movement) in
            &mut players.iter()
        {
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

            // register a slot if not passive
            if !ability.passive {
                actions.actions[ability.slot_number - 1].action = Some(ability.id);
                actions.actions[ability.slot_number - 1].next_available = 0.;
            }

            // apply the effects
            for effect in ability.effects.iter() {
                match effect {
                    AbilityDetail::Buff(detail) => {
                        match detail.buff_type {
                            BuffType::Armour => {
                                println!("Buffing base_armour");
                                defence.base_armour.buffs.push(detail.buff.clone());
                                defence.base_armour.update(game_time.elapsed_time);
                            }
                            BuffType::Health => {
                                println!("Buffing max_health");
                                health.max_health.buffs.push(detail.buff.clone());
                                health.max_health.update(game_time.elapsed_time);
                                health.current_health = health.max_health.value;
                                health.target_health = health.current_health;
                            }
                            BuffType::Mana => {
                                println!("Buffing max_mana");
                                mana.max_mana.buffs.push(detail.buff.clone());
                                mana.max_mana.update(game_time.elapsed_time);
                                mana.current_mana = mana.max_mana.value;
                            }
                            BuffType::Regeneration => {
                                println!("Buffing regeneration");
                                health.regeneration.buffs.push(detail.buff.clone());
                                health.regeneration.update(game_time.elapsed_time);
                            }
                            BuffType::MovementSpeed => {
                                println!("Buffing movement_speed");
                                movement.movement_speed.buffs.push(detail.buff.clone());
                                movement.movement_speed.update(game_time.elapsed_time);
                            }
                        };
                    }
                    _ => {}
                };
            }

            // mark it as applied
            player_score.xp -= ability.xp_cost;
            found = true;
            println!(
                "Added ability for player {}, they now have {:?}",
                player.player_id, player.abilities
            );

            ability_redraw_event.send(RedrawAbilityUiEvent);
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
