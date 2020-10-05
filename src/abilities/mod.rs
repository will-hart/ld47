use serde::{Deserialize, Serialize};
use spectre_core::Buff;

pub mod ability_data;
pub mod systems;

/// A rudimentary ability system.
///
/// The ability system needs to cover these requirements
///
/// - An "ability tree" which
///   - has distinct paths that are mutually exclusive
///   - has prerequisites for further abilities
///  - triggering the abilities through the UI
/// - Applying the effects of those abliities as part of the combat system
///
/// TODO:
///   - side effects like stun, damage over time

#[derive(Clone, Deserialize, Serialize)]
pub enum BuffType {
    Armour,
    Health,
    Mana,
    Regeneration,
    MovementSpeed,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum DamageType {
    Pure,
    Posion,
    Shock,
    Ice,
    Fire,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum AbilityDetail {
    Attack(AbilityAttackDetail),
    // TODO: AttackMultiple(AbilityAttackDetail, u32),
    AttackArea(AbilityAttackDetail, u32),
    Buff(AbilityBuffDetail),
    Heal(AbilityHealDetail),
    Revive(AbilityReviveDetail),
    SpawnAnimation(u128, usize, usize),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AbilityAttackDetail {
    pub damage_type: DamageType,
    pub min_damage: i32,
    pub max_damage: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AbilityBuffDetail {
    pub buff_type: BuffType,
    pub buff: Buff,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AbilityHealDetail {
    /// use Buff for faster regen over time
    pub burst_heal: f32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AbilityReviveDetail {
    pub revive_time: f32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AbilityDefinition {
    pub id: u16,
    pub prerequisites: Vec<u16>,
    pub xp_cost: usize,
    pub mana_cost: f32,
    pub passive: bool,
    pub slot_number: usize,
    pub cooldown: f32,
    pub name: String,
    pub description: String,
    pub effects: Vec<AbilityDetail>,
}

pub struct AbilityPurchaseRequest {
    pub player_id: u8,
    pub ability_id: u16,
}
