use abilities::{ability_data::*, systems::ability_purchase_system};
use assets::MaterialsAndTextures;
use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use bevy_ninepatch::NinePatchPlugin;
use combat::enemy_target_selection_system;
use combat::{dead_enemy_removal_system, enemy_auto_attack_system, player_auto_attack_system};
use components::PlayerScore;
use components::{CurrentWave, GameSceneConfigured};
use constants::*;
use events::*;
use game_scenes::*;
use game_ui::health_bar_system;
use movement::MovementPlugin;
use player_ui::*;
use spectre_animations::prelude::AnimationPlugin;
use spectre_core::CharacterStatsPlugin;
use spectre_loaders::{LoadAssets, ResourceLoaderPlugin};
use spectre_time::GameTimePlugin;
use systems::*;
use waves::wave_spawning_system;

mod abilities;
mod assets;
mod combat;
mod components;
mod constants;
mod data;
mod enemy_factory;
mod events;
mod game_scenes;
mod game_ui;
mod movement;
mod player_factory;
mod player_ui;
mod systems;
mod waves;

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
        //resources
        // .add_resource(ClearColor(Color::rgb_u8(8, 20, 30))) // not sure why this colour is too bright?
        .add_resource(ClearColor(Color::rgb_u8(1, 2, 3)))
        .init_resource::<AbilityDatabase>() // loaded using asset loader
        .init_resource::<CurrentWave>()
        .init_resource::<GameSceneConfigured>()
        .init_resource::<PlayerScore>()
        // event registration
        .add_event::<WaveSpawnedEvent>()
        .init_resource::<WaveSpawnedEventListener>()
        .add_event::<EndOfDayEvent>()
        .init_resource::<EndOfDayEventListener>()
        .add_event::<RedrawAbilityUiEvent>()
        .init_resource::<RedrawAbilityUiEventListener>()
        .add_system(wave_spawned_event_system.system())
        .add_system(end_of_day_system.system())
        // setup and plugins
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
        // random systems not properly organised into plugins yet
        .add_system(wave_spawning_system.system())
        .add_stage_after("update", "dead_removal")
        .add_system(player_auto_attack_system.system())
        .add_system(enemy_target_selection_system.system())
        .add_system(enemy_auto_attack_system.system())
        .add_system(health_bar_system.system())
        .add_system(update_player_health_ui.system())
        .add_system(update_player_mana_ui.system())
        .add_system(update_obelisk_status_text.system())
        .add_system(player_lane_change_interaction.system())
        .add_system(ability_purchase_system.system())
        .add_system(close_ability_screen.system())
        .add_system_to_stage("dead_removal", game_over_trigger.system())
        .add_system_to_stage("dead_removal", dead_enemy_removal_system.system())
        .add_system(player_incapacitation_system.system())
        .add_system(player_revival_system.system())
        .init_resource::<MaterialsAndTextures>()
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
                ("assets/character1_portrait.png", CHARACTER_1_PORTRAIT),
                ("assets/character2_portrait.png", CHARACTER_2_PORTRAIT),
                ("assets/character3_portrait.png", CHARACTER_3_PORTRAIT),
                ("assets/enemy_wolf.png", ENEMY_WOLF_SPRITE),
                ("assets/enemy_bear.png", ENEMY_BEAR_SPRITE),
                ("assets/enemy_troll.png", ENEMY_TROLL_SPRITE),
                ("assets/health_bar_full.png", HEALTHBAR_SPRITE_ID),
                ("assets/time_of_day1.png", TIME_OF_DAY_SPRITE1_ID),
                ("assets/time_of_day2.png", TIME_OF_DAY_SPRITE2_ID),
                ("assets/time_of_day3.png", TIME_OF_DAY_SPRITE3_ID),
                ("assets/time_of_day4.png", TIME_OF_DAY_SPRITE4_ID),
                ("assets/canyon.png", CANYON_SPRITE_ID),
                ("assets/rock.png", ROCK_SPRITE_ID),
                ("assets/tree.png", TREE_SPRITE_ID),
                ("assets/obelisk.png", OBELISK_SPRITE_ID),
                ("assets/splatter.png", SPLATTER_ID),
            ]
            .into_iter()
            .map(|a| a.into())
            .collect(),
        },))
        .spawn((LoadAssets {
            assets: vec![
                ("assets/audio/attacking_obelisk.mp3"),
                ("assets/audio/clang.mp3"),
                ("assets/audio/everywhere.mp3"),
                ("assets/audio/here_they_come.mp3"),
                ("assets/audio/leaving.mp3"),
                ("assets/audio/more_of_them.mp3"),
                ("assets/audio/moving.mp3"),
                ("assets/audio/obelisk_fallen.mp3"),
                ("assets/audio/protect_obelisk.mp3"),
                ("assets/audio/swoosh1.mp3"),
                ("assets/audio/swoosh2.mp3"),
            ]
            .into_iter()
            .map(|a| a.into())
            .collect(),
        },));
}
