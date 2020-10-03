use bevy::prelude::*;
use bevy_ninepatch::NinePatchBuilder;
use spectre_animations::spawn_animated_spritesheet;
use spectre_state::*;
use spectre_time::GameSpeedRequest;

use crate::{components::HealthBar, constants::*, enemy_factory::get_wolf};
use crate::{game_ui::spawn_ui, player_factory::get_player};

use super::MyGameScenes;

pub struct GameSceneEntity;

fn spawn_enemy(
    mut commands: &mut Commands,
    lane: usize,
    texture_atlas_handle: Handle<TextureAtlas>,
    health_bar_full: Handle<ColorMaterial>,
) {
    let spawned = spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle,
        0.3,
        vec![(0, 3)],
        Vec2::from(SPAWN_LOCATIONS[lane]).extend(GAME_ELEMENT_LAYER),
    )
    // TODO enum to specify enemy type
    .with_bundle(get_wolf(lane))
    .current_entity()
    .unwrap();

    commands
        .spawn(SpriteComponents {
            material: health_bar_full,
            // spawn off screen
            transform: Transform::from_translation(Vec3::new(-1000., -1000., GAME_ELEMENT_LAYER)),
            ..Default::default()
        })
        .with(HealthBar { entity: spawned });
}

fn spawn_player(
    mut commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    lane: usize,
) {
    let spawned = spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle,
        0.75,
        vec![(0, 1)],
        Vec2::from(TARGET_LOCATIONS[lane]).extend(GAME_ELEMENT_LAYER)
            - Vec3::new(0., PLAYER_OFFSET, 0.),
    )
    .current_entity()
    .unwrap();

    // TODO enum to specify enemy type
    commands.insert(spawned, get_player(lane));
}

pub fn setup_game_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    let transparent_material = materials.add(Color::NONE.into());

    // spawn the UI
    let entity = spawn_ui(&mut commands, nine_patches, transparent_material);
    commands.insert_one(entity, GameSceneEntity);

    // spawn three enemies
    let handle: Handle<Texture> = Handle::from_u128(ENEMY_WOLF_SPRITE);
    let texture = textures.get(&handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(handle, texture.size, 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar_handle = Handle::from_u128(HEALTHBAR_SPRITE_ID);
    let health_bar_material = materials.add(health_bar_handle.into());

    spawn_enemy(&mut commands, 0, texture_atlas_handle, health_bar_material);
    spawn_enemy(&mut commands, 1, texture_atlas_handle, health_bar_material);
    spawn_enemy(&mut commands, 2, texture_atlas_handle, health_bar_material);

    // spawn a sample game entity with easing
    let handle_player: Handle<Texture> = Handle::from_u128(CHARACTER_1_SPRITE);
    let texture_player = textures.get(&handle_player).unwrap();
    let texture_atlas_player = TextureAtlas::from_grid(handle_player, texture_player.size, 2, 1);
    let texture_atlas_handle_player = texture_atlases.add(texture_atlas_player);

    spawn_player(&mut commands, texture_atlas_handle_player, 0);
    spawn_player(&mut commands, texture_atlas_handle_player, 1);
    spawn_player(&mut commands, texture_atlas_handle_player, 2);
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
