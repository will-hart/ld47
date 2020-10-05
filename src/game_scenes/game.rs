use crate::{
    components::{
        GameSceneConfigured, GameSceneEntity, HealthBar, MainGameSidebarUi, MaterialsAndTextures,
    },
    game_ui::spawn_player_sidebar,
};
use bevy::prelude::*;
use bevy_ninepatch::NinePatchBuilder;
use spectre_animations::spawn_animated_spritesheet;
use spectre_random::RNG;
use spectre_state::*;
use spectre_time::{GameSpeedRequest, GameTime};

use crate::{components::CurrentWave, constants::*};
use crate::{game_ui::spawn_ui, player_factory::get_player};

use super::MyGameScenes;

fn spawn_player(
    mut commands: &mut Commands,
    assets: &Res<MaterialsAndTextures>,
    texture_atlas_handle: Handle<TextureAtlas>,
    player_id: u8,
    lane: usize,
) {
    let player_entity = spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle,
        0.75,
        vec![(0, 1), (2, 3)], // idle, dead
        Vec2::from(TARGET_LOCATIONS[lane]).extend(GAME_ELEMENT_LAYER)
            - Vec3::new(0., PLAYER_OFFSET_Y, 0.),
    )
    .with_bundle(get_player(player_id, lane))
    .with(GameSceneEntity)
    .current_entity()
    .unwrap();

    commands
        .spawn(SpriteComponents {
            material: assets.healthbar_material,
            // spawn off screen
            transform: Transform::from_translation(Vec3::new(-1000., -1000., GAME_ELEMENT_LAYER)),
            ..Default::default()
        })
        .with(HealthBar {
            entity: player_entity,
        })
        .with(GameSceneEntity);
}

pub fn setup_game_scene(
    mut commands: Commands,
    mut waves: ResMut<CurrentWave>,
    mut is_configured: ResMut<GameSceneConfigured>,
    assets: Res<MaterialsAndTextures>,
    game_time: Res<GameTime>,
    game_state: Res<GameState<MyGameScenes>>,
    nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
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
            spawn_player_sidebar(parent, &mut commands, &assets);
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

    // spawn the UI
    // NOTE: this moves materials. it shouldn't
    let entity = spawn_ui(&mut commands, &assets, nine_patches, assets.ui_material);
    commands.insert_one(entity, GameSceneEntity); // mark for cleanup

    // spawn a sample game entity with easing
    spawn_player(&mut commands, &assets, assets.char1_atlas, 0, 0);
    spawn_player(&mut commands, &assets, assets.char2_atlas, 1, 1);
    spawn_player(&mut commands, &assets, assets.char3_atlas, 2, 2);

    // spawn "world"
    commands
        .spawn(SpriteComponents {
            material: assets.canyon_material,
            transform: Transform::from_translation(Vec3::new(
                -270.,
                300.,
                GAME_ELEMENT_LAYER - 0.5,
            )),
            ..Default::default()
        })
        .with(GameSceneEntity)
        .spawn(SpriteComponents {
            material: assets.boulder_material,
            transform: Transform::from_translation(Vec3::new(50., 150., GAME_ELEMENT_LAYER + 0.5)),
            ..Default::default()
        })
        .with(GameSceneEntity);

    let trees: [[f32; 4]; 15] = [
        [
            -240.,
            320.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -210.,
            270.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            80.,
            170.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            100.,
            160.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -50.,
            -220.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            200.,
            140.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -340.,
            -110.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -75.,
            -75.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            250.,
            -270.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -250.,
            -220.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -280.,
            -120.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            110.,
            270.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            280.,
            70.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -310.,
            50.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
        [
            -450.,
            -20.,
            RNG::f32_between(0., 3.14),
            RNG::f32_between(0.85, 1.05),
        ],
    ];
    trees.iter().for_each(|[x, y, rot, scale]| {
        commands
            .spawn(SpriteComponents {
                material: assets.tree_material,
                transform: Transform::from_translation(Vec3::new(*x, *y, GAME_ELEMENT_LAYER + 0.5))
                    .with_rotation(Quat::from_axis_angle(Vec3::unit_z(), *rot))
                    .with_scale(*scale),
                ..Default::default()
            })
            .with(GameSceneEntity);
    });

    commands
        .spawn(SpriteComponents {
            material: assets.obelisk_material,
            transform: Transform::from_translation(Vec3::new(-180., -300., GAME_ELEMENT_LAYER)),
            ..Default::default()
        })
        .with(GameSceneEntity);
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
