use crate::components::Enemy;
use crate::{components::HealthBar, constants::UI_SPRITE_MARGIN};
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
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    transparent_material: Handle<ColorMaterial>,
) -> Entity {
    let texture_handle: Handle<Texture> = Handle::from_u128(UI_CONTAINER_ID);
    // TODO: store on a resource and get only once?
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        (),
    ));

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
                    sidebar_parent
                        .spawn(get_node_components(
                            Size::new(Val::Percent(100.), Val::Percent(25.)),
                            transparent_material,
                            true,
                        ))
                        .with_children(|sidebar_parent_inner| {
                            sidebar_parent_inner.spawn(get_ui_box(
                                nine_patch_handle,
                                texture_handle,
                                Vec2::new(320., 180.),
                            ));
                        })
                        .spawn(get_node_components(
                            Size::new(Val::Percent(100.), Val::Percent(25.)),
                            transparent_material,
                            true,
                        ))
                        .with_children(|sidebar_parent_inner| {
                            sidebar_parent_inner.spawn(get_ui_box(
                                nine_patch_handle,
                                texture_handle,
                                Vec2::new(320., 180.),
                            ));
                        })
                        .spawn(get_node_components(
                            Size::new(Val::Percent(100.), Val::Percent(25.)),
                            transparent_material,
                            true,
                        ))
                        .with_children(|sidebar_parent_inner| {
                            sidebar_parent_inner.spawn(get_ui_box(
                                nine_patch_handle,
                                texture_handle,
                                Vec2::new(320., 180.),
                            ));
                        });
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
