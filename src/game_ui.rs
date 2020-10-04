use crate::components::Enemy;
use crate::player_ui::{spawn_obelisk_ui, spawn_player_ui};
use crate::{components::HealthBar, constants::*};
use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchComponents, NinePatchData, NinePatchSize};
use spectre_core::Health;

use crate::constants::UI_CONTAINER_ID;

pub fn get_ui_box(
    nine_patch_handle: Handle<NinePatchBuilder>,
    texture_handle: Handle<Texture>,
    size: Vec2,
) -> NinePatchComponents<()> {
    NinePatchComponents {
        style: Style {
            margin: Rect::all(Val::Percent(5.)),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        nine_patch_data: NinePatchData {
            nine_patch: nine_patch_handle,
            texture: texture_handle,
            ..Default::default()
        },
        nine_patch_size: NinePatchSize(size),
        ..Default::default()
    }
}

pub fn get_node_components(
    size: Size<Val>,
    material: Handle<ColorMaterial>,
    is_column: bool,
) -> NodeComponents {
    NodeComponents {
        style: Style {
            size,
            justify_content: JustifyContent::FlexEnd,
            flex_direction: if is_column {
                FlexDirection::Column
            } else {
                FlexDirection::Row
            },
            ..Default::default()
        },
        material,
        ..Default::default()
    }
}

pub fn spawn_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    transparent_material: Handle<ColorMaterial>,
) -> Entity {
    let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();

    let texture_handle: Handle<Texture> = Handle::from_u128(UI_CONTAINER_ID);
    // TODO: store on a resource and get only once?
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        (),
    ));

    // get the portrait handle for the UI headers
    let char1_port_handle: Handle<Texture> = Handle::from_u128(CHARACTER_1_PORTRAIT);
    let character1_portrait_material = materials.add(char1_port_handle.into());

    let char2_port_handle: Handle<Texture> = Handle::from_u128(CHARACTER_2_PORTRAIT);
    let character2_portrait_material = materials.add(char2_port_handle.into());

    let char3_port_handle: Handle<Texture> = Handle::from_u128(CHARACTER_3_PORTRAIT);
    let character3_portrait_material = materials.add(char3_port_handle.into());

    // get the time of day UI items
    let handle_timeofday: Handle<Texture> = Handle::from_u128(TIME_OF_DAY_SPRITE1_ID);
    let initial_time_of_day_material = materials.add(handle_timeofday.into());

    // spawn a 75% full height box on the left
    // then spawn a sidebar on the right
    // the right hand sidebar should have the three character portraits
    commands
        .spawn(get_node_components(
            Size::new(Val::Percent(100.), Val::Percent(100.)),
            transparent_material,
            false,
        ))
        .with_children(|outer_parent| {
            outer_parent
                .spawn(get_node_components(
                    Size::new(Val::Percent(75.), Val::Percent(100.)),
                    transparent_material,
                    false,
                ))
                .with_children(|main_parent| {
                    main_parent.spawn(get_ui_box(
                        nine_patch_handle,
                        texture_handle,
                        Vec2::new(960., 720.),
                    ));
                })
                .spawn(get_node_components(
                    Size::new(Val::Percent(25.), Val::Percent(100.)),
                    transparent_material,
                    true,
                ))
                .with_children(|sidebar_parent| {
                    spawn_obelisk_ui(
                        sidebar_parent,
                        transparent_material,
                        initial_time_of_day_material,
                        font_handle,
                    );
                    // player 2
                    spawn_player_ui(
                        sidebar_parent,
                        transparent_material,
                        character3_portrait_material,
                        font_handle,
                        2,
                    );
                    spawn_player_ui(
                        sidebar_parent,
                        transparent_material,
                        character2_portrait_material,
                        font_handle,
                        1,
                    );
                    spawn_player_ui(
                        sidebar_parent,
                        transparent_material,
                        character1_portrait_material,
                        font_handle,
                        0,
                    );
                });
        });

    commands.current_entity().unwrap()
}

pub fn health_bar_system(
    mut commands: Commands,
    mut health_bar_query: Query<(Entity, &HealthBar, &mut Transform)>,
    enemy_query: Query<(&Enemy, &Health, &Transform)>,
) {
    let health_bar_offset = Vec3::new(0., 24., 0.);

    for (entity, health_bar, mut health_transform) in &mut health_bar_query.iter() {
        let tx_res = enemy_query.get::<Transform>(health_bar.entity);
        match tx_res {
            Err(_) => {
                // usually here because the linked entity has been removed, e.g. dead.
                // this seems (game jam) easier than parenting the healthbar to the sprite
                commands.despawn_recursive(entity);
                continue;
            }
            Ok(tx) => {
                health_transform.set_translation(tx.translation() + health_bar_offset);

                // shrink the health bar
                let health_res = enemy_query.get::<Health>(health_bar.entity);
                match health_res {
                    Err(_) => continue,
                    Ok(health) => {
                        health_transform.set_non_uniform_scale(Vec3::new(
                            health.current_health / health.max_health.value,
                            1.,
                            1.,
                        ));
                    }
                }
            }
        };
    }
}
