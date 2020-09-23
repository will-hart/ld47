use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use spectre_animations::prelude::{spawn_animated_spritesheet, AnimationPlugin};
use spectre_combat::prelude::AllegiancePlugin;
use spectre_core::prelude::{BuffableStatistic, CharacterStats, Health, Mana, Movement, Stats};
use spectre_loaders::{LoadingStatus, ResourceLoaderPlugin, TexturesToLoad};
use spectre_state::*;
use spectre_time::{GameSpeedRequest, GameTimePlugin};

const ANIMATED_SPRITESHEED_ID: u128 = 324890576394765893475;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum MyGameScenes {
    Loading,
    Menu,
    Game,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Spectre".to_string(),
            width: 1024,
            height: 768,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.005, 0.005, 0.005)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(spawn_player_debug.system())
        .add_plugin(GameTimePlugin)
        .add_plugin(ResourceLoaderPlugin)
        .add_plugin(AllegiancePlugin)
        .add_plugin(AnimationPlugin)
        .add_resource(GameState::<MyGameScenes> {
            current: GameStateStatus::Idle,
            next: None,
        })
        .add_system(game_state_transitions.system())
        .run();
}

fn setup(mut commands: Commands) {
    // spawn the camera
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // spawn a "character" with stats
        .spawn(CharacterStats {
            stats: Stats {
                strength: BuffableStatistic::new(10.),
                agility: BuffableStatistic::new(10.),
                intelligence: BuffableStatistic::new(10.),
                is_changed: true,
            },
            health: Health::new(100.),
            mana: Mana::new(200.),
            movement: Movement {
                movement_speed: BuffableStatistic::new(50.),
            },
        })
        // this loaders approach requires at least one tick of the game loop before
        // assets handles are available, therefore can't directly spawn player sprite here
        .spawn((TexturesToLoad {
            textures: vec![("assets/walk_sprite_sheet.png", ANIMATED_SPRITESHEED_ID).into()],
        },))
        // start the game clock running
        .spawn((GameSpeedRequest::new(1.0),));
}

fn game_state_transitions(
    loading: Res<LoadingStatus>,
    mut game_state: ResMut<GameState<MyGameScenes>>,
) {
    game_state.update();
    match game_state.current {
        GameStateStatus::Idle => game_state.set_transition(MyGameScenes::Loading),
        GameStateStatus::Running(state) => match state {
            MyGameScenes::Loading => {
                if loading.initial_load_done {
                    game_state.set_transition(MyGameScenes::Menu);
                }
            }
            MyGameScenes::Menu => {
                game_state.set_transition(MyGameScenes::Game);
            }
            _ => return,
        },
        _ => return,
    };
}

// demonstrates spawning a player using the spawn_animated_spritesheet helper
fn spawn_player_debug(
    commands: Commands,
    input: Res<Input<KeyCode>>,
    game_state: Res<GameState<MyGameScenes>>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !match game_state.current {
        GameStateStatus::Running(scene) => match scene {
            MyGameScenes::Game => true,
            _ => false,
        },
        _ => false,
    } {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let handle: Handle<Texture> = Handle::from_u128(ANIMATED_SPRITESHEED_ID);
    let texture = textures.get(&handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(handle, texture.size, 9, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_animated_spritesheet(
        commands,
        texture_atlas_handle,
        0.05,
        vec![(0, 8), (9, 17), (18, 26), (27, 35)],
        Vec3::new(0., 0., 0.),
    )
}
