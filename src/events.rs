use bevy::prelude::*;

use crate::{components::ObeliskStatusImageUiLink, constants::*};

pub struct WaveSpawnedEvent {
    pub wave_idx: usize,
}

#[derive(Default)]
pub struct WaveSpawnedEventListener {
    pub wave_spawned_reader: EventReader<WaveSpawnedEvent>,
}

pub struct EndOfDayEvent;

#[derive(Default)]
pub struct EndOfDayEventListener {
    pub end_of_day_reader: EventReader<EndOfDayEvent>,
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
