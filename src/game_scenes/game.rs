use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_ninepatch::NinePatchBuilder;
use spectre_animations::spawn_animated_spritesheet;
use spectre_state::*;
use spectre_time::GameSpeedRequest;

use crate::{components::*, game_ui::spawn_ui, player_factory::get_player};
use crate::{constants::*, enemy_factory::get_wolf};

use super::MyGameScenes;

pub struct GameSceneEntity;

pub fn setup_game_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    materials: ResMut<Assets<ColorMaterial>>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    // spawn the UI
    let entity = spawn_ui(&mut commands, nine_patches, materials);
    commands.insert_one(entity, GameSceneEntity);

    // spawn a sample game entity with easing
    let handle: Handle<Texture> = Handle::from_u128(ENEMY_WOLF_SPRITE);
    let texture = textures.get(&handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(handle, texture.size, 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let spawned = spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle,
        0.3,
        vec![(0, 3)],
        Vec3::new(
            SPAWN_LOCATIONS[0].0,
            SPAWN_LOCATIONS[0].1,
            GAME_ELEMENT_LAYER,
        ),
    );
    commands.insert(spawned, get_wolf());

    // spawn a sample game entity with easing
    let handle_player: Handle<Texture> = Handle::from_u128(CHARACTER_1_SPRITE);
    let texture_player = textures.get(&handle_player).unwrap();
    let texture_atlas_player = TextureAtlas::from_grid(handle_player, texture_player.size, 2, 1);
    let texture_atlas_handle_player = texture_atlases.add(texture_atlas_player);
    let spawned_player = spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle_player,
        0.3,
        vec![(0, 1)],
        Vec3::new(
            TARGET_LOCATIONS[0].0,
            TARGET_LOCATIONS[0].1 - PLAYER_OFFSET,
            GAME_ELEMENT_LAYER,
        ),
    );

    commands.insert(spawned_player, get_player(1));
}

pub fn apply_easing_to_enemy(
    mut commands: Commands,
    mut enemies_needing_init: Query<Without<HasEaseApplied, (Entity, &Transform, &Enemy)>>,
) {
    for (ent, transform, enemy) in &mut enemies_needing_init.iter() {
        commands.insert(
            ent,
            (
                transform.ease_to(
                    Transform::default().with_translate(enemy.target.extend(GAME_ELEMENT_LAYER)),
                    EaseFunction::QuadraticInOut,
                    EasingType::Once {
                        duration: std::time::Duration::from_millis(ENEMY_TWEEN_DURATION),
                    },
                ),
                HasEaseApplied,
            ),
        );
    }
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
