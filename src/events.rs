use crate::MaterialsAndTextures;
use bevy::prelude::*;
use spectre_state::GameState;
use spectre_time::GameSpeedRequest;

use crate::{
    components::CurrentWave, components::Enemy, components::ObeliskStatusImageUiLink,
    game_scenes::MyGameScenes, waves::WAVE_DATA,
};

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

/// Triggered on the end of day, passes true if all waves are complete (VICTORY!)
pub struct RedrawAbilityUiEvent;

#[derive(Default)]
pub struct RedrawAbilityUiEventListener {
    pub redraw_gui_reader: EventReader<RedrawAbilityUiEvent>,
}

pub fn end_of_day_system(
    mut commands: Commands,
    mut state: ResMut<EndOfDayEventListener>,
    mut game_state: ResMut<GameState<MyGameScenes>>,
    waves: Res<CurrentWave>,
    events: Res<Events<EndOfDayEvent>>,
    audio: Res<AudioOutput>,
    assets: Res<MaterialsAndTextures>,
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

    audio.play(assets.leaving_audio);

    // pause the game
    commands.spawn((GameSpeedRequest {
        new_game_speed: 0.0,
    },));

    // show the ability UI
    if waves.wave_idx >= WAVE_DATA.len() {
        println!("Showing victory UI");
        game_state.set_transition(MyGameScenes::GameOver);
    } else {
        // destroying remaining enemies
        // TODO - set a target back near the spawn, then remove
        for entity in &mut enemies.iter() {
            commands.despawn_recursive(entity);
        }

        println!("Showing end of day UI");
        game_state.set_transition(MyGameScenes::Abilities);
    }
}

pub fn wave_spawned_event_system(
    mut state: ResMut<WaveSpawnedEventListener>,
    audio: Res<AudioOutput>,
    assets: Res<MaterialsAndTextures>,
    events: Res<Events<WaveSpawnedEvent>>,
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

    if idx == 0 {
        audio.play(assets.here_they_come_audio);
    }

    for mut material in &mut obelisk_images.iter() {
        *material = match idx {
            0 => assets.time_of_day1_material,
            1 => assets.time_of_day2_material,
            2 => assets.time_of_day3_material,
            3 => assets.time_of_day4_material,
            _ => assets.time_of_day1_material,
        };
    }
}
