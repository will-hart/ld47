use crate::constants::*;
use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
// use bevy_easings::EasingsPlugin;
use bevy_ninepatch::NinePatchPlugin;
use combat::player_auto_attack_system;
use movement::MovementPlugin;
use spectre_animations::prelude::AnimationPlugin;
use spectre_core::CharacterStatsPlugin;
use spectre_loaders::{LoadAssets, ResourceLoaderPlugin};
use spectre_time::GameTimePlugin;

mod combat;
mod components;
mod constants;
mod data;
mod enemy_factory;
mod game_scenes;
mod game_ui;
mod movement;
mod player_factory;

use game_scenes::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Obelisk".to_string(),
            width: RESOLUTION_X,
            height: RESOLUTION_Y,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        // .add_resource(ClearColor(Color::rgb_u8(8, 20, 30))) // not sure why this colour is too bright?
        .add_resource(ClearColor(Color::rgb_u8(1, 2, 3)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_plugin(GameTimePlugin)
        .add_plugin(ResourceLoaderPlugin)
        // .add_plugin(DataFileLoaderPlugin)
        .add_plugin(CharacterStatsPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_plugin(MovementPlugin)
        .add_system(player_auto_attack_system.system())
        // .add_plugin(EasingsPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn the camera
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn((LoadAssets {
            assets: vec![
                ("assets/ui.png", UI_CONTAINER_ID),
                ("assets/character1.png", CHARACTER_1_SPRITE),
                ("assets/character2.png", CHARACTER_2_SPRITE),
                ("assets/character3.png", CHARACTER_3_SPRITE),
                ("assets/enemy_wolf.png", ENEMY_WOLF_SPRITE),
            ]
            .into_iter()
            .map(|a| a.into())
            .collect(),
        },));

    // start the game clock running
    // .spawn((GameSpeedRequest::new(1.0),));
}
