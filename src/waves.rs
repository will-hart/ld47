use crate::{
    components::*, constants::*, enemy_factory::get_enemy_bundle, enemy_factory::EnemyType,
    events::*, CurrentWave,
};
use bevy::prelude::*;
use spectre_animations::spawn_animated_spritesheet;
use spectre_random::RNG;
use spectre_time::GameTime;

/// Defines the enemy waves in the game. Waves are stored by monster type, in tuples of tuples
/// e.g. wolves might be item 1 in the top level tuple. The number of wolves to spawn is given
/// by the inner tuple values, e.g. ((lane 1 count, lane 2 count, lane 3 count), ...)

#[derive(Copy, Clone)]
pub struct Wave([isize; 3]);

impl Default for Wave {
    fn default() -> Self {
        Wave([0, 0, 0])
    }
}

pub struct WaveData {
    pub wolves: Wave,
    pub bears: Wave,
    pub post_wave_delay: f32,
}

const WAVE_DATA: [WaveData; 4] = [
    WaveData {
        wolves: Wave([1, 1, 1]),
        bears: Wave([0, 0, 0]),
        post_wave_delay: 10.,
    },
    WaveData {
        wolves: Wave([1, 1, 1]),
        bears: Wave([0, 1, 0]),
        post_wave_delay: 10.,
    },
    WaveData {
        wolves: Wave([1, 1, 1]),
        bears: Wave([0, 0, 0]),
        post_wave_delay: 8.,
    },
    WaveData {
        wolves: Wave([1, 1, 1]),
        bears: Wave([1, 1, 1]),
        post_wave_delay: 10.,
    },
];

fn spawn_enemy(
    mut commands: &mut Commands,
    enemy_type: EnemyType,
    lane: usize,
    texture_atlas_handle: Handle<TextureAtlas>,
    health_bar_full: Handle<ColorMaterial>,
) {
    let bundle = get_enemy_bundle(enemy_type, lane);

    let spawned = spawn_animated_spritesheet(
        &mut commands,
        texture_atlas_handle,
        0.3,
        vec![(0, 3)],
        Vec2::from(SPAWN_LOCATIONS[lane]).extend(GAME_ELEMENT_LAYER)
            + Vec3::new(RNG::f32_between(-10., 10.), RNG::f32_between(-30., 0.), 0.),
    )
    // TODO enum to specify enemy type
    .with_bundle(bundle)
    .with(GameSceneEntity)
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

pub fn wave_spawning_system(
    mut commands: Commands,
    mut waves: ResMut<CurrentWave>,
    game_time: Res<GameTime>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut wave_spawned: ResMut<Events<WaveSpawnedEvent>>,
    mut end_of_day: ResMut<Events<EndOfDayEvent>>,
) {
    // not ready to spawn
    if waves.next_wave_time > game_time.elapsed_time {
        return;
    }

    if waves.wave_idx > 0 && waves.wave_idx % 4 == 0 {
        end_of_day.send(EndOfDayEvent);
    }

    // nothing else to spawn
    if waves.wave_idx >= WAVE_DATA.len() {
        return;
    }

    // game paused
    if game_time.game_speed < 0.01 {
        return;
    }

    let wave_to_spawn = &WAVE_DATA[waves.wave_idx];
    wave_spawned.send(WaveSpawnedEvent {
        wave_idx: waves.wave_idx,
    });

    println!(
        "Spawning wave {} at {} (expected at {})",
        waves.wave_idx, game_time.elapsed_time, waves.next_wave_time
    );

    waves.next_wave_time = game_time.elapsed_time + wave_to_spawn.post_wave_delay;
    waves.wave_idx += 1;
    println!(
        "Wave {} should spawn at {}",
        waves.wave_idx, waves.next_wave_time
    );

    // just hack it out for now
    let wolves = wave_to_spawn.wolves.clone();

    let handle: Handle<Texture> = Handle::from_u128(ENEMY_WOLF_SPRITE);
    let texture = textures.get(&handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(handle, texture.size, 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar_handle = Handle::from_u128(HEALTHBAR_SPRITE_ID);
    let health_bar_material = materials.add(health_bar_handle.into());

    wolves
        .0
        .iter()
        .enumerate()
        .for_each(|(lane, &num_to_spawn)| {
            for _ in 0..num_to_spawn {
                spawn_enemy(
                    &mut commands,
                    EnemyType::Wolf,
                    lane,
                    texture_atlas_handle,
                    health_bar_material,
                );
            }
        });

    let bears = wave_to_spawn.bears.clone();

    let handle_bear: Handle<Texture> = Handle::from_u128(ENEMY_BEAR_SPRITE);
    let texture_bear = textures.get(&handle_bear).unwrap();
    let texture_atlas_bear = TextureAtlas::from_grid(handle_bear, texture_bear.size, 4, 1);
    let texture_atlas_handle_bear = texture_atlases.add(texture_atlas_bear);

    bears
        .0
        .iter()
        .enumerate()
        .for_each(|(lane, &num_to_spawn)| {
            for _ in 0..num_to_spawn {
                spawn_enemy(
                    &mut commands,
                    EnemyType::Bear,
                    lane,
                    texture_atlas_handle_bear,
                    health_bar_material,
                );
            }
        });
}
