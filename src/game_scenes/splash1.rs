use bevy::prelude::*;
use spectre_state::*;

use super::{ButtonMaterials, MyGameScenes};

pub struct Splash1SceneEntity;

pub fn run_splash1_scene(
    mut game_state: ResMut<GameState<MyGameScenes>>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Splash1) {
        return;
    }

    for (_button, interaction) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                game_state.set_transition(MyGameScenes::Splash2);
            }
            _ => {}
        }
    }
}

pub fn setup_splash1_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    asset_server: Res<AssetServer>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Splash1)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonComponents {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Px(15.)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal,
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn(TextComponents {
                        text: Text {
                            value: "Next".to_string(),
                            font: font_handle,
                            style: TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        },
                        ..Default::default()
                    });
                })
                .spawn(render_line("The obelisk speaks to them in their dreams, whispering of great treasures and limitless power.".to_string(), font_handle))
                .spawn(render_line("Bones are scattered at the feet of the obelisk, and mysterious power oozes from the stone's surface.".to_string(), font_handle))
                .spawn(render_line("In the depths of the forbidden jungle, three adventurers stumble upon an obselisk.".to_string(), font_handle));
        })
        .with(Splash1SceneEntity);
}

pub fn render_line(line: String, font_handle: Handle<Font>) -> TextComponents {
    TextComponents {
        style: Style {
            align_self: AlignSelf::Center,
            ..Default::default()
        },
        text: Text {
            value: line,
            font: font_handle,
            style: TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
            },
        },
        ..Default::default()
    }
}

pub fn teardown_splash1_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut splash_scene_items: Query<(Entity, &Splash1SceneEntity)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Splash1)
        || !game_state.is_in_status(&GameStatus::Exiting)
    {
        return;
    }

    println!("Tearing down splash1 screen");
    for (entity, _) in &mut splash_scene_items.iter() {
        commands.despawn_recursive(entity);
    }
}
