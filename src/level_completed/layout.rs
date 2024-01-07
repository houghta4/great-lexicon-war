use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Color, Commands, DespawnRecursiveExt, Display, Entity, FlexDirection, Font, Handle, JustifyContent, NodeBundle, PositionType, Query, Res, Style, Text, TextBundle, TextStyle, Val, With, ZIndex};
use bevy::utils::default;
use crate::game::level::resources::LevelInfo;
use crate::level_completed::components::{LevelCompletedButton, LevelCompletedMenu};
use crate::styles::{BACKGROUND_COLOR, get_button_style, NORMAL_BUTTON};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>, level_info: Res<LevelInfo>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");
    let title = if level_info.completed {
        "Victory, comrade".to_string()
    } else {
        "Defeat, comrade".to_string()
    };
    let score = level_info.get_score();
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute, // Needed to display separately from HUD.
                display: Display::Flex,                // Hidden by Default
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                ..default()
            },
            z_index: ZIndex::Local(1),
            ..default()
        },
        LevelCompletedMenu
    ))
        .with_children(|builder| {
            builder.spawn(NodeBundle {
               style: Style {
                   display: Display::Flex,
                   flex_direction: FlexDirection::Column,
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   width: Val::Percent(25.0),
                   height: Val::Percent(30.),
                   row_gap: Val::Px(8.0),
                   column_gap: Val::Px(8.0),
                   ..default()
               },
                background_color: BACKGROUND_COLOR,
                ..default()
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    title,
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::BLACK
                    }
                ));
                builder.spawn(TextBundle::from_section(
                    format!("Kills: {}", level_info.kills).to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::BLACK
                    }
                ));
                builder.spawn(TextBundle::from_section(
                    format!("Progress: {}", level_info.progress).to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::BLACK
                    }
                ));
                builder.spawn(TextBundle::from_section(
                    format!("Typos: {}", level_info.typos).to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::BLACK
                    }
                ));
                builder.spawn(TextBundle::from_section(
                    format!("Score: {}", score).to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::BLACK
                    }
                ));
                builder.spawn((
                    ButtonBundle {
                        style: get_button_style(150., 40.),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    LevelCompletedButton
                )).with_children(|builder| {
                    builder.spawn(TextBundle {
                       text: Text::from_section(
                           "Continue",
                           TextStyle {
                               font: font.clone(),
                               font_size: 20.,
                               color: Color::WHITE
                           }
                       ),
                        ..default()
                    });
                });
            });
        });
}

pub fn despawn_ui(mut commands: Commands, level_completed_menu_q: Query<Entity, With<LevelCompletedMenu>>) {
    if let Ok(level_completed_menu) = level_completed_menu_q.get_single() {
        commands.entity(level_completed_menu).despawn_recursive();
    }
}