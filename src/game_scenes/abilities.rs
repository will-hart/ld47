use bevy::prelude::*;
use spectre_state::GameState;
use spectre_state::GameStatus;

use crate::{components::*, game_scenes::MyGameScenes};

pub struct AbilityGuiMarker;

pub fn setup_ability_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_running_components: Query<(Entity, &GameRunningPlayerUi)>,
    mut sidebar_components: Query<(Entity, &MainGameSidebarUi)>,
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
        let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();
        let ui_material = materials.add(Color::NONE.into());
        let button_material = materials.add(Color::rgba_u8(70, 70, 70, 30).into());

        let player_1 = commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(310.), Val::Px(170.)),
                    margin: Rect::all(Val::Px(5.)),
                    ..Default::default()
                },
                material: ui_material,
                ..Default::default()
            })
            .with(AbilityGuiMarker)
            .current_entity()
            .unwrap();

        let player_2 = commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(310.), Val::Px(170.)),
                    margin: Rect::all(Val::Px(5.)),
                    ..Default::default()
                },
                material: ui_material,
                ..Default::default()
            })
            .with(AbilityGuiMarker)
            .current_entity()
            .unwrap();

        let player_3 = commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(310.), Val::Px(170.)),
                    margin: Rect::all(Val::Px(5.)),
                    ..Default::default()
                },
                material: ui_material,
                ..Default::default()
            })
            .with(AbilityGuiMarker)
            .current_entity()
            .unwrap();

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
                material: button_material,
                ..Default::default()
            })
            .with(AbilityGuiMarker)
            .with(CloseAbilitiesButtonLink)
            .with_children(|button_parent| {
                button_parent.spawn(TextComponents {
                    text: Text {
                        value: "Done".to_string(),
                        font: font_handle,
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

        commands.push_children(parent, &[button, player_3, player_2, player_1]);
    }
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
