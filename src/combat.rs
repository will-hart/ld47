use crate::assets::MaterialsAndTextures;
use bevy::prelude::*;
use spectre_core::Health;
use spectre_random::RNG;
use spectre_time::GameTime;

use crate::components::*;
use crate::constants::*;

/// A combat system

#[derive(Debug)]
pub struct CombatResult {
    pub damage: i32,
    pub is_crit: bool,

    // status effects
    pub burning: bool,
    pub poisoned: bool,
    pub frozen: bool,
    pub shocked: bool,
}

fn calc_damage(attack: i32, defence: i32) -> i32 {
    return attack * attack / (attack + defence);
}

/// Resolves a combat between the given attacker and defender, returning a combat result
pub fn resolve_combat(attack: &BaseAttack, defence: &Defence) -> CombatResult {
    let is_crit = RNG::test(attack.crit_chance);

    let mut base_attack = RNG::i32_between(attack.min_attack_damage, attack.max_attack_damage);
    if is_crit {
        base_attack *= 2;
    }

    let mut result = CombatResult {
        is_crit,
        damage: calc_damage(base_attack, defence.base_armour.value as i32),
        frozen: false,
        poisoned: false,
        burning: false,
        shocked: false,
    };

    if attack.fire_damage > 0 {
        let fire_damage = calc_damage(attack.fire_damage, defence.fire_armour);
        result.damage += fire_damage;
        result.burning = fire_damage > 0;
    }

    if attack.electricity_damage > 0 {
        let electrical_damage = calc_damage(attack.electricity_damage, defence.electricity_armour);
        result.damage += electrical_damage;
        result.shocked = electrical_damage > 0;
    }

    if attack.poison_damage > 0 {
        let poison_damage = calc_damage(attack.poison_damage, defence.poison_armour);
        result.damage += poison_damage;
        result.poisoned = poison_damage > 0;
    }

    if attack.frost_damage > 0 {
        let frost_damage = calc_damage(attack.frost_damage, defence.frost_armour);
        result.damage += frost_damage;
        result.frozen = frost_damage > 0;
    }

    result
}

/// Loop through all players, find enemies within their range and attack one of them
/// ignores incapacitated enemies
pub fn player_auto_attack_system(
    game_time: Res<GameTime>,
    audio: Res<AudioOutput>,
    assets: Res<MaterialsAndTextures>,
    mut player_query: Query<Without<Incapacitated, (&Player, &mut BaseAttack)>>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut Health, &Defence)>,
) {
    let player_y_pos = TARGET_LOCATIONS[0].1 + PLAYER_OFFSET_Y;

    for (player, mut attack) in &mut player_query.iter() {
        // attack cooldown
        if attack.next_attack > game_time.elapsed_time {
            continue;
        }

        if player.is_moving {
            continue;
        }

        // just pick the first available target in the lane :shrug:
        let mut target: Option<(Mut<Health>, &Defence)> = None;

        let enemy_query_instance = &mut enemy_query.iter();
        for (enemy, transform, health, defence) in enemy_query_instance {
            if health.current_health <= 0. {
                continue;
            }

            if player.current_lane != enemy.lane {
                continue;
            }

            if (player_y_pos - transform.translation().y()).abs() > attack.attack_range {
                continue;
            }

            target = Some((health, defence));
            break;
        }

        match target {
            None => {
                continue;
            }
            Some((mut health, defence)) => {
                let result = resolve_combat(&attack, defence);
                health.target_health -= result.damage as f32;

                audio.play(assets.clang_audio);

                // println!(
                //     "COMBAT! {:?}, new health: {} --> {}",
                //     result, health.current_health, health.target_health
                // );

                // update cooldown
                attack.next_attack = game_time.elapsed_time + attack.attack_speed.value;
            }
        }
    }
}

pub fn enemy_target_selection_system(
    mut enemy_query: Query<(&Enemy, &mut AttackTarget)>,
    mut player_query: Query<(Entity, &Player)>,
) {
    for (enemy, mut target) in &mut enemy_query.iter() {
        match target.entity {
            // just continue here, the auto attack system will reset to None if the player moves lane
            Some(_) => continue,
            None => {
                // not quite sure how to filter queries yet, so do this slightly weird way
                let players_in_lane: Vec<Entity> = player_query
                    .iter()
                    .iter()
                    .filter(|result| {
                        return result.1.current_lane == enemy.lane;
                    })
                    .map(|(e, _)| {
                        return e.clone();
                    })
                    .collect::<Vec<_>>();

                // if nobody is in the lane, target the obelisk
                target.is_obelisk = players_in_lane.len() == 0;
                if target.is_obelisk {
                    continue;
                }

                println!("Selected new target for enemy");
                target.entity =
                    Some(players_in_lane[RNG::usize_between(0, players_in_lane.len())].clone());

                // TODO could throttle checks here, e.g. if no players in the lane don't check every frame
            }
        };
    }
}

pub fn enemy_auto_attack_system(
    game_time: Res<GameTime>,
    audio: Res<AudioOutput>,
    assets: Res<MaterialsAndTextures>,
    mut player_score: ResMut<PlayerScore>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut AttackTarget, &mut BaseAttack)>,
    player_query: Query<(&Player, &Transform, &mut Health, &Defence)>,
) {
    for (enemy, enemy_tx, mut target, mut attack) in &mut enemy_query.iter() {
        // attack cooldown
        if attack.next_attack > game_time.elapsed_time {
            continue;
        }

        // if we don't have a target, and aren't attacking the obelisk
        if target.entity.is_none() {
            if !target.is_obelisk || player_score.obelisk_health == 0 {
                continue;
            }

            // if we are attacking the obelisk, and are in range, attack with defence of 0
            let dummy_defence = Defence::default();
            let result = resolve_combat(&attack, &dummy_defence);
            let damage = result.damage as usize / OBELISK_DAMAGE_MODIFIER;

            // play audio annoucement
            if player_score.last_obelisk_damage - game_time.elapsed_time > 10. {
                if RNG::test(0.5) {
                    audio.play(assets.protect_obelisk_audio);
                } else {
                    audio.play(assets.attacking_obelisk_audio);
                }
            }

            // prevent underflow
            if damage > player_score.obelisk_health {
                // Game over handled in a separate system
                player_score.obelisk_health = 0;
            } else {
                player_score.obelisk_health -= damage;
            }

            continue;
        }

        // check if the target is still in the same lane
        let target_player_result = player_query.get::<Player>(target.entity.unwrap());
        match target_player_result {
            Err(_) => {
                target.entity = None;
                continue;
            }
            Ok(target_player) => {
                if target_player.current_lane != enemy.lane {
                    target.entity = None;
                    continue;
                }
            }
        }

        // check the target is in range
        let target_entity = target.entity.unwrap();
        let transform = player_query.get_mut::<Transform>(target_entity).unwrap();
        if (enemy_tx.translation().y() - transform.translation().y()).abs() > attack.attack_range {
            continue;
        }

        // now carry out the combat against the target player
        let mut health = player_query.get_mut::<Health>(target_entity).unwrap();
        let defence = player_query.get::<Defence>(target_entity).unwrap();
        let result = resolve_combat(&attack, &defence);
        health.target_health -= result.damage as f32;

        // println!(
        //     "COMBAT! {:?}, new health: {} --> {}",
        //     result, health.current_health, health.target_health
        // );

        // update cooldown
        attack.next_attack = game_time.elapsed_time + attack.attack_speed.value;
    }
}

/// Removes dead enemies in post_update
pub fn dead_enemy_removal_system(
    mut commands: Commands,
    mut player_score: ResMut<PlayerScore>,
    mut enemy_query: Query<(Entity, &Enemy, &Health)>,
) {
    for (entity, enemy, health) in &mut enemy_query.iter() {
        if health.current_health <= 0. {
            println!("Player killed enemy, gained {} XP", enemy.xp_reward);

            player_score.xp += enemy.xp_reward;
            commands.despawn_recursive(entity);
        }
    }
}
