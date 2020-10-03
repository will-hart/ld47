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
        damage: calc_damage(base_attack, defence.base_armour),
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
pub fn player_auto_attack_system(
    game_time: Res<GameTime>,
    mut player_query: Query<(&Player, &mut BaseAttack)>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut Health, &Defence)>,
) {
    let player_y_pos = TARGET_LOCATIONS[0].1 + PLAYER_OFFSET;

    for (player, mut attack) in &mut player_query.iter() {
        // attack cooldown
        if attack.next_attack > game_time.elapsed_time {
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

                println!(
                    "COMBAT! {:?}, new health: {} --> {}",
                    result, health.current_health, health.target_health
                );

                // update cooldown
                attack.next_attack = game_time.elapsed_time + attack.attack_speed.value;
            }
        }
    }
}

/// Removes dead enemies in post_update
pub fn dead_enemy_removal_system(
    mut commands: Commands,
    mut enemy_query: Query<With<Enemy, (Entity, &Health)>>,
) {
    for (entity, health) in &mut enemy_query.iter() {
        if health.current_health <= 0. {
            println!("DESPAWNING");
            commands.despawn_recursive(entity);
        }
    }
}
