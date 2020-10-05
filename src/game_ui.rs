use crate::components::{Enemy, MainGameSidebarUi, MaterialsAndTextures};
use crate::player_ui::{spawn_obelisk_ui, spawn_player_ui};
use crate::{components::HealthBar, constants::*};
use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchComponents, NinePatchData, NinePatchSize};
use spectre_core::Health;

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
    assets: &Res<MaterialsAndTextures>,
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
        material: assets.ui_material,
        ..Default::default()
    }
}

pub fn spawn_ui(
    commands: &mut Commands,
    assets: &Res<MaterialsAndTextures>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    transparent_material: Handle<ColorMaterial>,
) -> Entity {
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
    let mut sidebar_entity = Entity::new(0);
    let root_entity = commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect::all(Val::Px(0.)),
                ..Default::default()
            },
            material: transparent_material,
            ..Default::default()
        })
        .with_children(|outer_parent| {
            outer_parent
                // main content
                .spawn(get_node_components(
                    Size::new(Val::Percent(75.), Val::Percent(100.)),
                    &assets,
                    false,
                ))
                .with_children(|main_parent| {
                    main_parent.spawn(get_ui_box(
                        nine_patch_handle,
                        assets.nine_patch_texture,
                        Vec2::new(960., 720.),
                    ));
                });
            // sidebar
            sidebar_entity = outer_parent
                .spawn(get_node_components(
                    Size::new(Val::Percent(25.), Val::Percent(100.)),
                    &assets,
                    true,
                ))
                .with(MainGameSidebarUi)
                .current_entity()
                .unwrap();
        })
        .current_entity()
        .unwrap();

    spawn_player_sidebar(sidebar_entity, commands, &assets);

    root_entity
}

pub fn spawn_player_sidebar(
    parent: Entity,
    commands: &mut Commands,
    assets: &Res<MaterialsAndTextures>,
) {
    let obelisk_ui = spawn_obelisk_ui(commands, assets, assets.main_font);
    let player_2 = spawn_player_ui(commands, assets, assets.char3_portrait_material, 2);
    let player_1 = spawn_player_ui(commands, assets, assets.char2_portrait_material, 1);
    let player_0 = spawn_player_ui(commands, assets, assets.char1_portrait_material, 0);
    commands.push_children(parent, &[obelisk_ui, player_2, player_1, player_0]);
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
