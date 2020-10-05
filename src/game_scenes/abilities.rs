use crate::{
    abilities::AbilityDefinition, assets::MaterialsAndTextures, combat::resolve_combat,
    constants::FLAME_WALL_ID, constants::HEAL_ID, constants::MELEE_RANGE,
    constants::TARGET_LOCATIONS,
};
use crate::{abilities::AbilityDetail, player_ui::text};
use bevy::prelude::*;
use spectre_animations::spawn_animated_spritesheet;
use spectre_core::Health;
use spectre_state::GameState;
use spectre_state::GameStatus;
use spectre_time::GameTime;

use crate::{
    abilities::ability_data::AbilityDatabase, abilities::AbilityPurchaseRequest, components::*,
    events::*, game_scenes::MyGameScenes,
};

pub struct AbilityGuiMarker;
pub struct AbilityGuiSidebarMarker;

pub fn setup_ability_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    assets: Res<MaterialsAndTextures>,
    player_score: Res<PlayerScore>,
    mut ability_data: ResMut<AbilityDatabase>,
    mut game_running_components: Query<(Entity, &GameRunningPlayerUi)>,
    mut sidebar_components: Query<(Entity, &MainGameSidebarUi)>,
    mut player_query: Query<(&Player, &PlayerAbilityActions)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Abilities)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    println!("Destroying game running GUI");
    for (ent, _) in &mut game_running_components.iter() {
        commands.despawn_recursive(ent);
    }

    println!("Spawning ability GUI");
    // should only happen once I hope :D
    for (parent, _) in &mut sidebar_components.iter() {
        spawn_ability_sidebar(
            parent,
            &mut commands,
            &player_score,
            &assets,
            &mut player_query,
            &mut ability_data,
        );
    }
}

fn spawn_ability_sidebar(
    parent: Entity,
    mut commands: &mut Commands,
    player_score: &Res<PlayerScore>,
    assets: &Res<MaterialsAndTextures>,
    player_query: &mut Query<(&Player, &PlayerAbilityActions)>,
    mut ability_data: &mut ResMut<AbilityDatabase>,
) {
    let mut player_uis: Vec<Entity> = Vec::default();
    for (player, actions) in &mut player_query.iter() {
        player_uis.push(spawn_player_ability_ui(
            &player,
            &actions,
            &mut commands,
            &assets,
            &mut ability_data,
        ));
    }

    let button = commands
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(32.0)),
                margin: Rect {
                    left: Val::Px(107.),
                    right: Val::Px(107.),
                    ..Default::default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: assets.button_material,
            ..Default::default()
        })
        .with(AbilityGuiMarker)
        .with(AbilityGuiSidebarMarker)
        .with(CloseAbilitiesButtonLink)
        .with_children(|button_parent| {
            button_parent.spawn(TextComponents {
                text: Text {
                    value: "Start New Loop".to_string(),
                    font: assets.main_font,
                    style: TextStyle {
                        font_size: 12.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        })
        .current_entity()
        .unwrap();

    let abort_button = commands
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(32.0)),
                margin: Rect {
                    left: Val::Px(107.),
                    right: Val::Px(107.),
                    ..Default::default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: assets.button_material,
            ..Default::default()
        })
        .with(AbilityGuiMarker)
        .with(AbilityGuiSidebarMarker)
        .with(AbortGameButtonLink)
        .with_children(|button_parent| {
            button_parent.spawn(TextComponents {
                text: Text {
                    value: "Give Up".to_string(),
                    font: assets.main_font,
                    style: TextStyle {
                        font_size: 12.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        })
        .current_entity()
        .unwrap();

    let available_xp = commands
        .spawn(text(
            assets.main_font,
            format!("{} XP TO SPEND", player_score.xp),
            12.,
        ))
        .with(AbilityGuiMarker)
        .with(AbilityGuiSidebarMarker)
        .current_entity()
        .unwrap();

    let heading = commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(310.), Val::Px(40.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: assets.ui_material,
            ..Default::default()
        })
        .with(AbilityGuiMarker)
        .with(AbilityGuiSidebarMarker)
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "THE STONE GIVES US STRENGTH".to_string(), // random spacer
                    font: assets.main_font,
                    style: TextStyle {
                        font_size: 14.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        })
        .current_entity()
        .unwrap();

    commands.push_children(
        parent,
        &[
            abort_button,
            button,
            player_uis[2],
            player_uis[1],
            player_uis[0],
            available_xp,
            heading,
        ],
    );
}

pub fn spawn_player_ability_ui(
    player: &Player,
    actions: &PlayerAbilityActions,
    commands: &mut Commands,
    assets: &Res<MaterialsAndTextures>,
    ability_database: &mut ResMut<AbilityDatabase>,
) -> Entity {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(310.), Val::Px(170.)),
                margin: Rect::all(Val::Px(5.)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: assets.ui_material,
            ..Default::default()
        })
        .with(AbilityGuiMarker)
        .with(AbilityGuiSidebarMarker)
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: format!("Player {} Upgrades", player.player_id),
                    font: assets.main_font,
                    style: TextStyle {
                        font_size: 14.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });

            let mut to_spawn: Vec<AbilityDefinition> = vec![];

            if actions.actions[0].action.is_none() {
                to_spawn.push(
                    ability_database
                        .get(get_ability_id(player.player_id, 1))
                        .clone(),
                );
            }
            if actions.actions[1].action.is_none() {
                to_spawn.push(
                    ability_database
                        .get(get_ability_id(player.player_id, 2))
                        .clone(),
                );
            }

            let next_level = player.get_next_level();
            if next_level.is_some() {
                to_spawn.push(ability_database.get(next_level.unwrap() - 2).clone());
            }

            parent
                .spawn(NodeComponents {
                    style: Style {
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    material: assets.ui_material,
                    ..Default::default()
                })
                .with_children(|button_wrapper| {
                    for ability in to_spawn.iter() {
                        button_wrapper
                            .spawn(ButtonComponents {
                                style: Style {
                                    size: Size::new(Val::Px(90.0), Val::Px(32.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..Default::default()
                                },
                                material: assets.button_material,
                                ..Default::default()
                            })
                            .with(AbilityPurchaseInteraction {
                                ability_id: ability.id,
                                player_id: player.player_id,
                            })
                            .with_children(|inner_parent| {
                                inner_parent
                                    .spawn(text(assets.main_font, ability.name.clone(), 8.))
                                    .spawn(text(
                                        assets.main_font,
                                        format!("{} XP", ability.xp_cost),
                                        8.,
                                    ));
                            });
                    }
                });
        })
        .current_entity()
        .unwrap()
}

pub fn teardown_ability_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut menu_scene_items: Query<(Entity, &AbilityGuiMarker)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Abilities)
        || !game_state.is_in_status(&GameStatus::Exiting)
    {
        return;
    }

    println!("Tearing down abilities screen");
    for (entity, _) in &mut menu_scene_items.iter() {
        commands.despawn_recursive(entity);
    }
}

pub fn ability_purchase_interaction(
    mut commands: Commands,
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &AbilityPurchaseInteraction)>,
) {
    for (_, interaction, request) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                println!("Requesting ability purchase");

                // spawn the request
                commands.spawn((AbilityPurchaseRequest {
                    ability_id: request.ability_id,
                    player_id: request.player_id,
                },));
            }
            _ => {}
        };
    }
}

pub fn redraw_ability_ui_on_event(
    mut commands: Commands,
    assets: Res<MaterialsAndTextures>,
    player_score: Res<PlayerScore>,
    mut state: ResMut<RedrawAbilityUiEventListener>,
    events: Res<Events<RedrawAbilityUiEvent>>,
    mut ability_data: ResMut<AbilityDatabase>,
    mut existing_sidebar_items: Query<(Entity, &AbilityGuiSidebarMarker)>,
    mut sidebar_components: Query<(Entity, &MainGameSidebarUi)>,
    mut player_query: Query<(&Player, &PlayerAbilityActions)>,
) {
    let mut found = false;

    for _ in state.redraw_gui_reader.iter(&events) {
        println!("Redrawing ability GUI");
        found = true;
        break;
    }

    if !found {
        return;
    }

    // despawn the old GUI
    for (ent, _) in &mut existing_sidebar_items.iter() {
        commands.despawn_recursive(ent);
    }

    // recreate the GUI
    for (parent, _) in &mut sidebar_components.iter() {
        spawn_ability_sidebar(
            parent,
            &mut commands,
            &player_score,
            &assets,
            &mut player_query,
            &mut ability_data,
        );
    }
}

fn get_ability_prefix(player_id: u8, slot_id: usize) -> String {
    match player_id {
        0 => match slot_id {
            0 => "[q]".to_string(),
            1 => "[w]".to_string(),
            _ => "?".to_string(),
        },
        1 => match slot_id {
            0 => "[e]".to_string(),
            1 => "[r]".to_string(),
            _ => "?".to_string(),
        },
        2 => match slot_id {
            0 => "[d]".to_string(),
            1 => "[f]".to_string(),
            _ => "?".to_string(),
        },
        _ => "-".to_string(),
    }
}

fn get_ability_id(player_id: u8, slot_id: usize) -> u16 {
    (player_id as u16 + 1) * 1000 - 1 + slot_id as u16
}

pub fn ability_ui_updates(
    game_time: Res<GameTime>,
    mut ability_database: ResMut<AbilityDatabase>,
    mut abilities: Query<(&mut Text, &PlayerAbilityLink)>,
    mut players: Query<(&Player, &PlayerAbilityActions)>,
) {
    for (mut text, link) in &mut abilities.iter() {
        for (player, actions) in &mut players.iter() {
            if player.player_id != link.player_id {
                continue;
            }

            let slot_action = actions.actions[link.action_number];
            let ability_data =
                ability_database.get(get_ability_id(link.player_id, link.action_number));

            if slot_action.action.is_none() {
                text.value = "".to_string();
                break;
            }

            text.value = format!(
                "{} {}",
                get_ability_prefix(link.player_id, link.action_number),
                if game_time.elapsed_time > slot_action.next_available {
                    ability_data.name.clone()
                } else {
                    format!(
                        "{:.1}s cooldown",
                        (slot_action.next_available - game_time.elapsed_time) as isize
                    )
                }
            );

            // no need to keep searching
            break;
        }
    }
}

pub fn spawn_abilities(
    mut commands: Commands,
    game_time: Res<GameTime>,
    input: ResMut<Input<KeyCode>>,
    mut database: ResMut<AbilityDatabase>,
    mut players: Query<Without<Incapacitated, (&Player, &PlayerAbilityActions)>>,
) {
    for (player, abilities) in &mut players.iter() {
        let id = player.player_id;

        let codes: (KeyCode, KeyCode) = match id {
            0 => (KeyCode::Q, KeyCode::W),
            1 => (KeyCode::E, KeyCode::R),
            2 => (KeyCode::D, KeyCode::F),
            _ => (KeyCode::Q, KeyCode::W),
        };

        let slot: Option<AbilityActionDetails> = if input.just_pressed(codes.0) {
            Some(abilities.actions[0])
        } else if input.just_pressed(codes.1) {
            Some(abilities.actions[1])
        } else {
            None
        };

        if slot.is_none() {
            continue;
        }

        let mut action = slot.unwrap();

        if action.action.is_none() {
            println!("Ability aborted - not equipped");
            continue;
        }
        if action.next_available > game_time.elapsed_time {
            println!("Ability aborted - not ready");
            continue;
        }

        let definition = database.get(action.action.unwrap()).clone();
        action.next_available = game_time.elapsed_time + definition.cooldown;

        println!("Spawned ability {}", definition.name);
        commands.spawn((SpawnedAbility {
            lane: player.current_lane,
            effects: definition.effects,
            applied: false,
        },));
    }
}

/// Reeeeeeeeeee(factor)
/// a lot of this stuff (i.e. resolving combat) really shouldn't be done here - should raise an event or something instead?
pub fn execute_abilities(
    mut commands: Commands,
    assets: Res<MaterialsAndTextures>,
    mut spawned_abilities: Query<(Entity, &mut SpawnedAbility)>,
    mut players: Query<Without<Incapacitated, (&Player, &mut Health)>>,
    mut incapacitated_players: Query<(&Player, &mut Incapacitated)>,
    mut enemies: Query<(&Enemy, &Defence, &mut Health, &Transform)>,
) {
    for (_entityTODO_USE_TO_DESPAWN_AND_REMOVE_ABILITY_APPLIED, mut ability) in
        &mut spawned_abilities.iter()
    {
        if ability.applied {
            continue;
        }

        ability.applied = true;

        for effect in &mut ability.effects.iter() {
            match effect {
                AbilityDetail::Buff(_) => todo!("Need a way to target a buff? No time!"),
                AbilityDetail::Attack(data) => {
                    for (enemy, defence, mut health, tx) in &mut enemies.iter() {
                        // wrong lane
                        if enemy.lane != ability.lane {
                            continue;
                        }

                        // out of range
                        // TODO - probably need to include player offset here as well?
                        if (tx.translation().y() - TARGET_LOCATIONS[ability.lane].1).abs()
                            > MELEE_RANGE
                        {
                            continue;
                        }

                        // resolve combat
                        // TODO - don't do this in here, as its editing the health from too many places
                        let result = resolve_combat(
                            &BaseAttack {
                                min_attack_damage: data.min_damage,
                                max_attack_damage: data.max_damage,
                                ..Default::default()
                            },
                            defence,
                        );
                        health.target_health -= result.damage as f32;

                        // just apply to the first available
                        break;
                    }
                }
                AbilityDetail::AttackArea(data, range) => {
                    for (enemy, defence, mut health, tx) in &mut enemies.iter() {
                        // wrong lane
                        if enemy.lane != ability.lane {
                            continue;
                        }

                        // out of range
                        // TODO - probably need to include player offset here as well?
                        if (tx.translation().y() - TARGET_LOCATIONS[ability.lane].1).abs()
                            > *range as f32
                        {
                            continue;
                        }

                        // resolve combat
                        // TODO - don't do this in here, as its editing the health from too many places
                        let result = resolve_combat(
                            &BaseAttack {
                                min_attack_damage: data.min_damage,
                                max_attack_damage: data.max_damage,
                                ..Default::default()
                            },
                            defence,
                        );
                        health.target_health -= result.damage as f32;
                    }
                }
                AbilityDetail::Heal(data) => {
                    for (player, mut health) in &mut players.iter() {
                        if player.current_lane != ability.lane {
                            continue;
                        }

                        health.target_health += data.burst_heal;
                    }
                }
                AbilityDetail::Revive(_) => {
                    for (player, mut incap) in &mut incapacitated_players.iter() {
                        if player.current_lane != ability.lane {
                            continue;
                        }

                        incap.is_revived = true;
                    }
                }
                AbilityDetail::SpawnAnimation(atlas_id, frame_start, frame_end) => {
                    let pos: Vec3 = Vec2::from(TARGET_LOCATIONS[ability.lane]).extend(0.);
                    let atlas = match *atlas_id {
                        FLAME_WALL_ID => assets.flame_wall_atlas,
                        HEAL_ID => assets.heal_atlas,
                        _ => assets.splatter_atlas,
                    };

                    spawn_animated_spritesheet(
                        &mut commands,
                        atlas,
                        0.1,
                        vec![(*frame_start, *frame_end)],
                        pos,
                        true,
                    );
                }
            };

            // TODO: for some reason this panics, no time to debug
            // commands.despawn(entity);
        }
    }
}
