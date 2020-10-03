use crate::constants::UI_SPRITE_MARGIN;
use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchComponents, NinePatchData, NinePatchSize};

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
    mut commands: Commands,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: Handle<Texture> = Handle::from_u128(UI_CONTAINER_ID);
    // TODO: store on a resource and get only once?
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        UI_SPRITE_MARGIN,
        (),
    ));

    let material = materials.add(Color::NONE.into());

    // TODO remove
    let material_red = materials.add(Color::rgb_u8(255, 0, 0).into());
    let material_green = materials.add(Color::rgb_u8(0, 255, 0).into());

    // spawn a 75% full height box on the left
    // then spawn a sidebar on the right
    // the right hand sidebar should have the three character portraits
    commands
        .spawn(get_node_components(
            Size::new(Val::Percent(100.), Val::Percent(100.)),
            material,
            false,
        ))
        .with_children(|outer_parent| {
            outer_parent
                .spawn(get_node_components(
                    Size::new(Val::Percent(75.), Val::Percent(100.)),
                    material,
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
                    material,
                    true,
                ))
                .with_children(|sidebar_parent| {
                    sidebar_parent
                        .spawn(get_node_components(
                            Size::new(Val::Percent(100.), Val::Percent(25.)),
                            material_red,
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
                            material_green,
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
                            material_red,
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
}
