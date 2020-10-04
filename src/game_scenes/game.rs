use crate::{
    components::{GameSceneConfigured, GameSceneEntity, MainGameSidebarUi},
    game_ui::spawn_player_sidebar,
};
use bevy::prelude::*;
use bevy_ninepatch::NinePatchBuilder;
use spectre_animations::spawn_animated_spritesheet;
use spectre_state::*;
use spectre_time::{GameSpeedRequest, GameTime};

use crate::{components::CurrentWave, constants::*};
use crate::{game_ui::spawn_ui, player_factory::get_player};

use super::MyGameScenes;

fn spawn_player(
    mut commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    player_id: u8,
    lane: usize,
) {
    spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle,
        0.75,
        vec![(0, 1), (2, 3)], // idle, dead
        Vec2::from(TARGET_LOCATIONS[lane]).extend(GAME_ELEMENT_LAYER)
            - Vec3::new(0., PLAYER_OFFSET_Y, 0.),
    )
    .with_bundle(get_player(player_id, lane))
    .with(GameSceneEntity);
}

pub fn setup_game_scene(
    mut commands: Commands,
    mut waves: ResMut<CurrentWave>,
    mut is_configured: ResMut<GameSceneConfigured>,
    asset_server: ResMut<AssetServer>,
    game_time: Res<GameTime>,
    game_state: Res<GameState<MyGameScenes>>,
    nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sidebar_components: Query<(Entity, &MainGameSidebarUi)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    if is_configured.0 == true {
        println!("Skipping full game scene configuration");

        println!("Respawning player GUI");

        for (parent, _) in &mut sidebar_components.iter() {
            let transparent_material = materials.add(Color::NONE.into());
            spawn_player_sidebar(
                parent,
                &mut commands,
                materials,
                transparent_material,
                asset_server.load("assets/fonts/teletactile.ttf").unwrap(),
            );

            break; // really only should be one, need a neater way to do this
        }

        return;
    }

    is_configured.0 = true;

    // start running the game when entering
    commands.spawn((GameSpeedRequest {
        new_game_speed: DEFAULT_GAME_SPEED,
    },));

    // start spawning waves some time in the future
    waves.next_wave_time = game_time.elapsed_time + 2.;

    let transparent_material = materials.add(Color::NONE.into());

    // spawn the UI
    let entity = spawn_ui(
        &mut commands,
        asset_server,
        materials,
        nine_patches,
        transparent_material,
    );
    commands.insert_one(entity, GameSceneEntity); // mark for cleanup

    // spawn a sample game entity with easing
    let handle_player: Handle<Texture> = Handle::from_u128(CHARACTER_1_SPRITE);
    let texture_player = textures.get(&handle_player).unwrap();
    let texture_atlas_player = TextureAtlas::from_grid(handle_player, texture_player.size, 4, 1);
    let texture_atlas_handle_player = texture_atlases.add(texture_atlas_player);

    spawn_player(&mut commands, texture_atlas_handle_player, 0, 0);

    let handle_player2: Handle<Texture> = Handle::from_u128(CHARACTER_2_SPRITE);
    let texture_player2 = textures.get(&handle_player2).unwrap();
    let texture_atlas_player2 = TextureAtlas::from_grid(handle_player2, texture_player2.size, 4, 1);
    let texture_atlas_handle_player2 = texture_atlases.add(texture_atlas_player2);
    spawn_player(&mut commands, texture_atlas_handle_player2, 1, 1);

    let handle_player3: Handle<Texture> = Handle::from_u128(CHARACTER_3_SPRITE);
    let texture_player3 = textures.get(&handle_player3).unwrap();
    let texture_atlas_player3 = TextureAtlas::from_grid(handle_player3, texture_player3.size, 4, 1);
    let texture_atlas_handle_player3 = texture_atlases.add(texture_atlas_player3);
    spawn_player(&mut commands, texture_atlas_handle_player3, 2, 2);
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

    // don't tear down if we are going to abilities screen
    // can be *fairly* certain next is defined
    let skip_despawning = match game_state.next.unwrap() {
        MyGameScenes::Abilities => true,
        _ => false,
    };

    println!("NEXT::: {:?} - {}", game_state.next, skip_despawning);
    if skip_despawning {
        println!("Skipping game scene teardown");
        return;
    }

    println!("Tearing down game screen");
    for (entity, _) in &mut loading_scene_items.iter() {
        commands.despawn_recursive(entity);
    }
}
