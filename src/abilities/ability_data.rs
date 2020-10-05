/// Loads in an ability database. This could better be done from a file
/// but its a bit tricky to work out the RON format for these so I'll start here.
///
/// The following conventions are used for ability IDs:
///
/// - 0-999 are general abilities
/// - 1000 - 1999 are healer abilities
/// - 2000 - 2999 are rogue abilities
/// - 3000 - 3999 are warrior abilities
use crate::constants::*;

use super::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct AbilityDatabase {
    pub abilities: HashMap<u16, AbilityDefinition>,
}

impl AbilityDatabase {
    /// will cause a panic for missing Ids, so don't call this CRAZILY.
    pub fn get(&mut self, id: u16) -> &mut AbilityDefinition {
        self.abilities.get_mut(&id).unwrap()
    }

    pub fn new() -> Self {
        let mut abilities: HashMap<u16, AbilityDefinition> = HashMap::new();

        // level 2 upgrade
        abilities.insert(
            0,
            AbilityDefinition {
                id: 0,
                xp_cost: 50,
                prerequisites: vec![],
                name: "Level 2".to_string(),
                description: "+50 health, +50 mana".to_string(),
                passive: true,
                slot_number: 0,
                cooldown: 0.,
                mana_cost: 0.,
                effects: vec![
                    AbilityDetail::Buff(AbilityBuffDetail {
                        buff_type: BuffType::Health,
                        buff: Buff {
                            expiry: 0.,
                            percentage: 0.,
                            amount: 50.,
                        },
                    }),
                    AbilityDetail::Buff(AbilityBuffDetail {
                        buff_type: BuffType::Mana,
                        buff: Buff {
                            expiry: 0.,
                            percentage: 0.,
                            amount: 50.,
                        },
                    }),
                ],
            },
        );

        // level 3 upgrade
        abilities.insert(
            1,
            AbilityDefinition {
                id: 1,
                xp_cost: 150,
                prerequisites: vec![0],
                name: "Level 3".to_string(),
                description: "+50 health, +25 speed".to_string(),
                passive: true,
                slot_number: 0,
                cooldown: 0.,
                mana_cost: 0.,
                effects: vec![
                    AbilityDetail::Buff(AbilityBuffDetail {
                        buff_type: BuffType::Health,
                        buff: Buff {
                            expiry: 0.,
                            percentage: 0.,
                            amount: 50.,
                        },
                    }),
                    AbilityDetail::Buff(AbilityBuffDetail {
                        buff_type: BuffType::MovementSpeed,
                        buff: Buff {
                            expiry: 0.,
                            percentage: 0.,
                            amount: 25.,
                        },
                    }),
                ],
            },
        );

        // level 4 upgrade
        abilities.insert(
            2,
            AbilityDefinition {
                id: 2,
                xp_cost: 300,
                prerequisites: vec![1],
                name: "Level 4".to_string(),
                description: "+100 health, +2 regen".to_string(),
                passive: true,
                slot_number: 0,
                cooldown: 0.,
                mana_cost: 0.,
                effects: vec![
                    AbilityDetail::Buff(AbilityBuffDetail {
                        buff_type: BuffType::Health,
                        buff: Buff {
                            expiry: 0.,
                            percentage: 0.,
                            amount: 50.,
                        },
                    }),
                    AbilityDetail::Buff(AbilityBuffDetail {
                        buff_type: BuffType::Regeneration,
                        buff: Buff {
                            expiry: 0.,
                            percentage: 0.,
                            amount: 2.,
                        },
                    }),
                ],
            },
        );

        // flame_wall
        abilities.insert(
            1000,
            AbilityDefinition {
                id: 1000,
                xp_cost: 300,
                prerequisites: vec![],
                name: "Flame Wall".to_string(),
                description: "Fire, walled".to_string(),
                passive: false,
                slot_number: 1,
                cooldown: 15.,
                mana_cost: 30.,
                effects: vec![
                    AbilityDetail::AttackArea(
                        AbilityAttackDetail {
                            damage_type: DamageType::Fire,
                            min_damage: 10,
                            max_damage: 30,
                        },
                        96,
                    ),
                    AbilityDetail::SpawnAnimation(FLAME_WALL_ID, 0, 9),
                ],
            },
        );

        // flame bash
        abilities.insert(
            1001,
            AbilityDefinition {
                id: 1001,
                xp_cost: 200,
                prerequisites: vec![],
                name: "Flame Bash".to_string(),
                description: "Fire, bashed".to_string(),
                passive: false,
                slot_number: 2,
                cooldown: 12.,
                mana_cost: 30.,
                effects: vec![AbilityDetail::Attack(AbilityAttackDetail {
                    damage_type: DamageType::Fire,
                    min_damage: 30,
                    max_damage: 60,
                })],
            },
        );

        // heal
        abilities.insert(
            2000,
            AbilityDefinition {
                id: 2000,
                xp_cost: 200,
                prerequisites: vec![],
                name: "Heal".to_string(),
                description: "Heal nearby heroes".to_string(),
                passive: false,
                slot_number: 1,
                cooldown: 15.,
                mana_cost: 30.,
                effects: vec![
                    AbilityDetail::Heal(AbilityHealDetail { burst_heal: 50. }),
                    AbilityDetail::SpawnAnimation(HEAL_ID, 0, 7),
                ],
            },
        );

        // revive
        abilities.insert(
            2001,
            AbilityDefinition {
                id: 2001,
                xp_cost: 200,
                prerequisites: vec![],
                name: "Revive".to_string(),
                description: "Revive nearby heroes".to_string(),
                passive: false,
                slot_number: 2,
                cooldown: 20.,
                mana_cost: 60.,
                effects: vec![
                    AbilityDetail::Revive(AbilityReviveDetail { revive_time: 0. }),
                    AbilityDetail::SpawnAnimation(HEAL_ID, 0, 7),
                ],
            },
        );

        // smash
        abilities.insert(
            3000,
            AbilityDefinition {
                id: 3000,
                xp_cost: 200,
                prerequisites: vec![],
                name: "Smash".to_string(),
                description: "Hit a single enemy, very hard".to_string(),
                passive: false,
                slot_number: 1,
                cooldown: 15.,
                mana_cost: 20.,
                effects: vec![AbilityDetail::Attack(AbilityAttackDetail {
                    damage_type: DamageType::Pure,
                    min_damage: 15,
                    max_damage: 75,
                })],
            },
        );

        // detonate
        abilities.insert(
            3001,
            AbilityDefinition {
                id: 3001,
                xp_cost: 200,
                prerequisites: vec![],
                name: "Smash".to_string(),
                description: "Hit a single enemy, very hard".to_string(),
                passive: false,
                slot_number: 2,
                cooldown: 20.,
                mana_cost: 20.,
                effects: vec![AbilityDetail::AttackArea(
                    AbilityAttackDetail {
                        damage_type: DamageType::Pure,
                        min_damage: 20,
                        max_damage: 50,
                    },
                    50,
                )],
            },
        );

        AbilityDatabase { abilities }
    }
}

/// Loads the abilities into the ability database
impl FromResources for AbilityDatabase {
    fn from_resources(_: &Resources) -> Self {
        AbilityDatabase::new()
    }
}
