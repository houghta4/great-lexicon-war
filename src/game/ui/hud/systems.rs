use crate::game::input::components::InputText;
use crate::game::player::components::Player;
use crate::game::resources::{RandomWord, WordBank};
use crate::game::ui::styles::{BACKGROUND_COLOR, SECTION_BACKGROUND_COLOR};
use crate::game::word_match::components::{Word, WordTarget};
use crate::game::WordComplexity;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::ui::{Display, FlexDirection, GridPlacement, Val};
use bevy::utils::default;

use super::components::{PlayerAmmoText, PlayerHealthText, ReloadText};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
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
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.)),
                                ..default()
                            },
                            background_color: SECTION_BACKGROUND_COLOR,
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "Faction".to_string(),
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
                                        value: " / 100".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    },
                                ]),
                                PlayerHealthText,
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
                    builder.spawn((TextBundle::default(), ReloadText));
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

/// Update player's health in HUD
///
/// Change colors depending on percentage
pub fn update_health(
    mut text_q: Query<&mut Text, With<PlayerHealthText>>,
    player_q: Query<&Player, Changed<Player>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok(mut health_bundle) = text_q.get_single_mut() {
            let mut health = &mut health_bundle.sections[0];
            health.value = format!("{}", player.health);
            match health {
                ref mut h if player.health > 75.0 => h.style.color = Color::GREEN,
                ref mut h if player.health > 50.0 => h.style.color = Color::YELLOW,
                ref mut h if player.health > 25.0 => h.style.color = Color::ORANGE_RED,
                _ => health.style.color = Color::RED,
            }
        }
    }
}

/// Update player's ammo count in HUD
///
/// Change colors depending on percentage
pub fn update_ammo(
    mut text_q: Query<&mut Text, With<PlayerAmmoText>>,
    player_q: Query<&Player, Changed<Player>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok(mut ammo_bundle) = text_q.get_single_mut() {
            ammo_bundle.sections[2].value = format!("{}", player.ammo.1);
            let ammo = &mut ammo_bundle.sections[0];
            ammo.value = format!("{}", player.ammo.0);
            match player.ammo.0 as f32 / player.ammo.1 as f32 {
                a if a > 0.75 => ammo.style.color = Color::WHITE,
                a if a > 0.5 => ammo.style.color = Color::YELLOW,
                a if a > 0.25 => ammo.style.color = Color::ORANGE_RED,
                _ => ammo.style.color = Color::RED,
            }
        }
    }
}

/// Reset `WordTarget::Reload` after a bullet is fired by the player
pub fn update_reload_word(
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<ReloadText>)>,
    mut reload_q: Query<(&mut Text, &mut Word), With<ReloadText>>,
) {
    if let Ok((mut text, mut word)) = reload_q.get_single_mut() {
        let w = word_bank.get_word(WordComplexity::Medium, &word_q);
        text.sections[1].value = w.to_owned();
        word.1 = w.to_owned();
    }
}

/// Remove `WordTarget::Reload` after reloading
pub fn remove_reload_word(mut reload_q: Query<(&mut Text, &mut Word), With<ReloadText>>) {
    if let Ok((mut text, mut word)) = reload_q.get_single_mut() {
        text.sections[1].value = " ".to_owned();
        word.1 = " ".to_owned();
    }
}
