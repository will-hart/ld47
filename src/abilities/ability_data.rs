/// Loads in an ability database. This could better be done from a file
/// but its a bit tricky to work out the RON format for these so I'll start here.
///
/// The following conventions are used for ability IDs:
///
/// - 0-999 are general abilities
/// - 1000 - 1999 are healer abilities
/// - 2000 - 2999 are rogue abilities
/// - 3000 - 3999 are warrior abilities
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
                description: "Upgrades +50 max health, +50 max mana".to_string(),
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
                description: "Upgrades +50 max health, +25 movement speed".to_string(),
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
                description: "Upgrades +100 max health, +2 health regen/sec".to_string(),
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

        AbilityDatabase { abilities }
    }
}

/// Loads the abilities into the ability database
impl FromResources for AbilityDatabase {
    fn from_resources(_: &Resources) -> Self {
        AbilityDatabase::new()
    }
}
