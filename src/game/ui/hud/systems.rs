use bevy::asset::AssetServer;
use bevy::prelude::{AlignItems, BuildChildren, Color, Commands, Font, Handle, JustifyItems, NodeBundle, RepeatedGridTrack, Res, Style, TextBundle, UiRect};
use bevy::text::TextStyle;
use bevy::ui::{Display, FlexDirection, GridPlacement, Val};
use bevy::utils::default;
use crate::game::input::components::InputText;
use crate::game::ui::styles::{BACKGROUND_COLOR, SECTION_BACKGROUND_COLOR};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }).with_children(|builder| {
        builder.spawn(NodeBundle {
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
        }).with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK
                    }
                ),
                InputText
            ));
        });

        builder.spawn(NodeBundle {
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
        }).with_children(|builder| {
            builder.spawn(NodeBundle {
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
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Faction".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.,
                        color: Color::WHITE
                    }
                ));
            });
            builder.spawn(NodeBundle {
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
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Mission Title".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.,
                        color: Color::WHITE
                    }
                ));
            });
            builder.spawn(NodeBundle {
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
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Health".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.,
                        color: Color::WHITE
                    }
                ));
            });
            builder.spawn(NodeBundle {
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
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Ammo".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.,
                        color: Color::WHITE
                    }
                ));
            });
            builder.spawn(NodeBundle {
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
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Special".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.,
                        color: Color::WHITE
                    }
                ));
            });
        });
    });
}