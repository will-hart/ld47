use bevy::prelude::*;
use spectre_time::GameSpeedRequest;

use crate::{components::Enemy, components::ObeliskStatusImageUiLink, constants::*};

pub struct WaveSpawnedEvent {
    pub wave_idx: usize,
}

#[derive(Default)]
pub struct WaveSpawnedEventListener {
    pub wave_spawned_reader: EventReader<WaveSpawnedEvent>,
}

/// Triggered on the end of day, passes true if all waves are complete (VICTORY!)
pub struct EndOfDayEvent(pub bool);

#[derive(Default)]
pub struct EndOfDayEventListener {
    pub end_of_day_reader: EventReader<EndOfDayEvent>,
}

pub fn end_of_day_system(
    mut commands: Commands,
    mut state: ResMut<EndOfDayEventListener>,
    events: Res<Events<EndOfDayEvent>>,
    mut enemies: Query<With<Enemy, Entity>>,
) {
    let mut found = false;
    for _ in state.end_of_day_reader.iter(&events) {
        println!("End of day - pausing game");
        found = true;
        break;
    }

    // no events, just return
    if !found {
        return;
    }

    // pause the game
    commands.spawn((GameSpeedRequest {
        new_game_speed: 0.0,
    },));

    // destroying remaining enemies
    for entity in &mut enemies.iter() {
        commands.despawn_recursive(entity);
    }

    // show the ability UI
    println!("Showing end of day UI");
}

pub fn wave_spawned_event_system(
    mut state: ResMut<WaveSpawnedEventListener>,
    events: Res<Events<WaveSpawnedEvent>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut obelisk_images: Query<With<ObeliskStatusImageUiLink, &mut Handle<ColorMaterial>>>,
) {
    let mut idx: usize = 0;
    let mut found = false;
    for my_event in state.wave_spawned_reader.iter(&events) {
        found = true;
        idx = my_event.wave_idx % 4;
    }

    if !found {
        return;
    }

    for mut material in &mut obelisk_images.iter() {
        *material = match idx {
            0 => {
                let handle: Handle<Texture> = Handle::from_u128(TIME_OF_DAY_SPRITE1_ID);
                materials.add(handle.into())
            }
            1 => {
                let handle: Handle<Texture> = Handle::from_u128(TIME_OF_DAY_SPRITE2_ID);
                materials.add(handle.into())
            }
            2 => {
                let handle: Handle<Texture> = Handle::from_u128(TIME_OF_DAY_SPRITE3_ID);
                materials.add(handle.into())
            }
            3 => {
                let handle: Handle<Texture> = Handle::from_u128(TIME_OF_DAY_SPRITE4_ID);
                materials.add(handle.into())
            }
            _ => {
                let handle: Handle<Texture> = Handle::from_u128(TIME_OF_DAY_SPRITE1_ID);
                materials.add(handle.into())
            }
        };
    }
}
