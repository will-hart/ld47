use crate::player_ui::text;
use crate::{abilities::AbilityDefinition, assets::MaterialsAndTextures};
use bevy::prelude::*;
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
                size: Size::new(Val::Px(96.0), Val::Px(32.0)),
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
                    value: "Done".to_string(),
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
                    value: "Upgrades".to_string(), // random spacer
                    font: assets.main_font,
                    style: TextStyle {
                        font_size: 20.0,
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
