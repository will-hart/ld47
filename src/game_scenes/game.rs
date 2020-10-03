use bevy::prelude::*;
use bevy_ninepatch::NinePatchBuilder;
use spectre_state::*;
use spectre_time::GameSpeedRequest;

use crate::game_ui::spawn_ui;

use super::MyGameScenes;

pub struct GameSceneEntity;

pub fn setup_game_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    // start running the game when entering
    commands.spawn((GameSpeedRequest {
        new_game_speed: 1.0,
    },));

    spawn_ui(commands, nine_patches, materials);
}

// demonstrates spawning a player using the spawn_animated_spritesheet helper
pub fn run_game_scene(
    // commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    // textures: ResMut<Assets<Texture>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game) {
        return;
    }
}

pub fn teardown_game_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut loading_scene_items: Query<(Entity, &GameSceneEntity)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game)
        || !game_state.is_in_status(&GameStatus::Exiting)
    {
        return;
    }

    println!("Tearing down loading screen");
    for (entity, _) in &mut loading_scene_items.iter() {
        commands.despawn(entity);
    }
}
