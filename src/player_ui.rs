// A lot of this code is pretty ugly, but UI is a bit underdone in Bevy and its
// possible there are better ways to do this sort of thing. I'm going for dev speed here,
// not nice code /shrug

use crate::{components::*, constants::*};
use bevy::prelude::*;
use spectre_core::{Health, Mana};

fn spacer(font_handle: Handle<Font>) -> TextComponents {
    TextComponents {
        text: Text {
            value: "".to_string(), // random spacer
            font: font_handle,
            style: TextStyle {
                font_size: 10.0,
                color: Color::rgb(0.8, 0.8, 0.8),
            },
        },
        ..Default::default()
    }
}

/// A function that creates the empty player ui and marks components for
/// the init_player_ui system to set up
pub fn spawn_player_ui(
    parent: &mut ChildBuilder,
    material: Handle<ColorMaterial>,
    portrait_material: Handle<ColorMaterial>,
    font_handle: Handle<Font>,
    player_id: u8,
) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(340.), Val::Px(180.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                margin: Rect::all(Val::Px(5.)),
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|ui_parent| {
            ui_parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Px(320.), Val::Px(40.)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material,
                    ..Default::default()
                })
                .with_children(|player_header_parent| {
                    player_header_parent
                        .spawn(ImageComponents {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                ..Default::default()
                            },
                            material: portrait_material,
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .spawn(TextComponents {
                            text: Text {
                                // TODO: Player name
                                value: format!("Player {}", player_id),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 12.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                },
                            },
                            ..Default::default()
                        });
                })
                .spawn(spacer(font_handle))
                .spawn(TextComponents {
                    text: Text {
                        value: "  Health ?/?".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 10.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                })
                .with(PlayerHealthLink {
                    player_id,
                    entity: None,
                })
                .spawn(TextComponents {
                    text: Text {
                        value: "  Mana ?/?".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 10.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                })
                .with(PlayerManaLink {
                    player_id,
                    entity: None,
                })
                .spawn(spacer(font_handle))
                .spawn(TextComponents {
                    text: Text {
                        value: "Abilities".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 12.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                })
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Px(320.), Val::Px(80.)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material,
                    ..Default::default()
                })
                .with_children(|ability_button_parent| {
                    ability_button_parent
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material,
                            ..Default::default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextComponents {
                                text: Text {
                                    value: "1".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 12.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material,
                            ..Default::default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextComponents {
                                text: Text {
                                    value: "2".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 12.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material,
                            ..Default::default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextComponents {
                                text: Text {
                                    value: "3".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 12.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material,
                            ..Default::default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextComponents {
                                text: Text {
                                    value: "4".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 12.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        });
                })
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Px(320.), Val::Px(80.)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material,
                    ..Default::default()
                })
                .with_children(|lane_change_parent| {
                    lane_change_parent
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material,
                            ..Default::default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextComponents {
                                text: Text {
                                    value: "<".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 20.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .with(PlayerLaneChangeLink {
                            player_id,
                            delta: -1,
                        })
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material,
                            ..Default::default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextComponents {
                                text: Text {
                                    value: ">".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 20.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .with(PlayerLaneChangeLink {
                            player_id,
                            delta: 1,
                        });
                });
        });
}

pub fn spawn_obelisk_ui(
    parent: &mut ChildBuilder,
    material: Handle<ColorMaterial>,
    initial_time_of_day_material: Handle<ColorMaterial>,
    font_handle: Handle<Font>,
) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(310.), Val::Px(100.)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                margin: Rect::all(Val::Px(5.)),
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|obelisk_parent| {
            obelisk_parent
                .spawn(ImageComponents {
                    style: Style {
                        size: Size::new(Val::Px(32.), Val::Px(32.)),
                        ..Default::default()
                    },
                    material: initial_time_of_day_material,
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(ObeliskStatusImageUiLink)
                .spawn(TextComponents {
                    text: Text {
                        value: "Obelisk heath ? / ?".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 12.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                })
                .with(ObeliskStatusTextUiLink);
        });
}

/// A system that updates player UI. UGLY
pub fn update_player_health_ui(
    mut text_query: Query<(&mut PlayerHealthLink, &mut Text)>,
    mut players: Query<(Entity, &mut Player, &Health)>,
) {
    for (mut ply, mut txt) in &mut text_query.iter() {
        if ply.entity.is_none() {
            for (ent, test_player, _) in &mut players.iter() {
                if test_player.player_id == ply.player_id {
                    ply.entity = Some(ent);
                    break;
                }
            }
        }

        if ply.entity.is_none() {
            println!("Unable to find entity for player health text");
            continue;
        }

        // update the text
        let health = players.get::<Health>(ply.entity.unwrap()).unwrap();
        txt.value = format!(
            "Health: {:.0} / {:.0}",
            health.target_health, health.max_health.value
        );
    }
}

/// A system that updates player UI. UGLY
pub fn update_player_mana_ui(
    mut text_query: Query<(&mut PlayerManaLink, &mut Text)>,
    mut players: Query<(Entity, &mut Player, &Mana)>,
) {
    for (mut ply, mut txt) in &mut text_query.iter() {
        if ply.entity.is_none() {
            for (ent, test_player, _) in &mut players.iter() {
                if test_player.player_id == ply.player_id {
                    ply.entity = Some(ent);
                    break;
                }
            }
        }

        if ply.entity.is_none() {
            println!("Unable to find entity for player mana text");
            continue;
        }

        // update the text
        let mana = players.get::<Mana>(ply.entity.unwrap()).unwrap();
        txt.value = format!(
            "Mana: {:.0} / {:.0}",
            mana.current_mana, mana.max_mana.value
        );
    }
}

pub fn player_lane_change_interaction(
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &PlayerLaneChangeLink)>,
    mut player_query: Query<&mut Player>,
) {
    for (_, interaction, link) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                println!(
                    "Player {} requested lane change delta {}",
                    link.player_id, link.delta
                );

                for mut player in &mut player_query.iter() {
                    if player.player_id == link.player_id {
                        let new_lane = player.target_lane as i8 + link.delta;
                        if new_lane < MIN_LANE as i8 || new_lane > MAX_LANE as i8 {
                            println!(" --> Invalid request, ignoring lane change");
                        } else {
                            player.target_lane = new_lane as usize;
                        }
                        break;
                    }
                }
            }
            _ => {}
        };
    }
}

pub fn update_obelisk_status_text(
    player_score: Res<PlayerScore>,
    _obelisk: &ObeliskStatusTextUiLink,
    mut text: Mut<Text>,
) {
    text.value = format!("Obelisk health {} / 1000", player_score.obelisk_health);
}
