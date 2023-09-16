use bevy::prelude::*;

use crate::game::{
    input::components::InputText,
    word_match::components::{Word, WordTarget},
};
use crate::game::ui::hud::components::Hud;
use crate::styles::{BACKGROUND_COLOR, SECTION_BACKGROUND_COLOR};

use super::{
    components::{
        HealText, PlayerAmmoText, PlayerHealthBar, PlayerHealthPack, PlayerHealthText, ReloadText,
    },
    systems::get_health_bundle,
};

pub fn despawn_ui(mut commands: Commands, hud_q: Query<Entity, With<Hud>>) {
    if let Ok(hud) = hud_q.get_single() {
        commands.entity(hud).despawn_recursive();
    }
}

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Hud
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(5.),
                        width: Val::Percent(40.),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(10.)),
                        top: Val::Percent(5.),
                        left: Val::Percent(30.),
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR,
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle::from_section(
                            "".to_string(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        ),
                        InputText,
                    ));
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(20.),
                        width: Val::Percent(100.),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(10.)),
                        grid_template_columns: RepeatedGridTrack::flex(5, 1.),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.),
                        row_gap: Val::Px(12.),
                        column_gap: Val::Px(12.),
                        top: Val::Percent(75.),
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR,
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_row: GridPlacement::span(3),
                                padding: UiRect::all(Val::Px(3.)),
                                justify_items: JustifyItems::Center,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.)),
                                ..default()
                            },
                            background_color: SECTION_BACKGROUND_COLOR,
                            ..default()
                        })
                        .with_children(|builder| {
                            // Health icons
                            builder
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            display: Display::Flex,
                                            grid_row: GridPlacement::span(3),
                                            padding: UiRect::all(Val::Px(3.)),
                                            justify_items: JustifyItems::Center,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            margin: UiRect::all(Val::Px(10.)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    PlayerHealthPack,
                                ))
                                .with_children(|builder| {
                                    for _ in 0..3 {
                                        builder.spawn(get_health_bundle(&asset_server));
                                    }
                                });
                            // Health Word
                            builder.spawn((
                                TextBundle {
                                    text: Text::from_sections([
                                        TextSection::from_style(TextStyle {
                                            font_size: 24.,
                                            font: font.clone(),
                                            color: Color::ORANGE,
                                        }),
                                        TextSection::new(
                                            " ",
                                            TextStyle {
                                                font_size: 24.,
                                                font: font.clone(),
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        ),
                                    ]),
                                    transform: Transform::from_translation(Vec3::new(0., 60., 2.)),
                                    ..default()
                                },
                                Word(WordTarget::Heal, " ".to_string()),
                                HealText,
                            ));
                        });

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::span(3),
                                margin: UiRect::horizontal(Val::Px(120.)),
                                padding: UiRect::all(Val::Px(3.)),
                                justify_items: JustifyItems::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: SECTION_BACKGROUND_COLOR,
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "Mission Title".to_string(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 24.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::span(2),
                                grid_row: GridPlacement::span(2),
                                padding: UiRect::all(Val::Px(3.)),
                                justify_items: JustifyItems::Center,
                                ..default()
                            },
                            background_color: SECTION_BACKGROUND_COLOR,
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "Health".to_string(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 24.,
                                    color: Color::WHITE,
                                },
                            ));

                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        padding: UiRect::all(Val::Px(3.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|builder| {
                                    builder.spawn((
                                        TextBundle {
                                            text: Text::from_sections(vec![
                                                TextSection {
                                                    value: "--".to_string(),
                                                    style: TextStyle {
                                                        font: font.clone(),
                                                        font_size: 24.,
                                                        color: Color::WHITE,
                                                    },
                                                },
                                                TextSection {
                                                    value: " / 100".to_string(),
                                                    style: TextStyle {
                                                        font: font.clone(),
                                                        font_size: 24.,
                                                        color: Color::WHITE,
                                                    },
                                                },
                                            ]),
                                            z_index: ZIndex::Global(3),
                                            ..default()
                                        },
                                        PlayerHealthText,
                                    ));
                                });
                            builder.spawn((
                                NodeBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(3.)),
                                        width: Val::Percent(100.0),
                                        justify_self: JustifySelf::Start,
                                        ..default()
                                    },
                                    ..default()
                                },
                                PlayerHealthBar,
                            ));
                        });
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::span(1),
                                grid_row: GridPlacement::span(2),
                                padding: UiRect::all(Val::Px(3.)),
                                justify_items: JustifyItems::Center,
                                ..default()
                            },
                            background_color: SECTION_BACKGROUND_COLOR,
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "Ammo".to_string(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 24.,
                                    color: Color::WHITE,
                                },
                            ));
                            builder.spawn((
                                TextBundle::from_sections(vec![
                                    TextSection {
                                        value: "--".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    },
                                    TextSection {
                                        value: " / ".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    },
                                    TextSection {
                                        value: "--".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    },
                                ]),
                                PlayerAmmoText,
                            ));
                            // Reload Word
                            builder.spawn((
                                TextBundle {
                                    text: Text::from_sections([
                                        TextSection::from_style(TextStyle {
                                            font_size: 24.,
                                            font: font.clone(),
                                            color: Color::ORANGE,
                                        }),
                                        TextSection::new(
                                            " ",
                                            TextStyle {
                                                font_size: 24.,
                                                font: font.clone(),
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        ),
                                    ]),
                                    transform: Transform::from_translation(Vec3::new(0., 60., 2.)),
                                    ..default()
                                },
                                Word(WordTarget::Reload, " ".to_string()),
                                ReloadText,
                            ));
                        });
                    // Special
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::span(1),
                                grid_row: GridPlacement::span(2),
                                padding: UiRect::all(Val::Px(3.)),
                                justify_items: JustifyItems::Center,
                                ..default()
                            },
                            background_color: SECTION_BACKGROUND_COLOR,
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "Special".to_string(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 24.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}
