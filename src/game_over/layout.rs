use bevy::prelude::{AlignItems, AssetServer, BuildChildren, ButtonBundle, Color, Commands, DespawnRecursiveExt, Display, Entity, FlexDirection, Font, Handle, JustifyContent, NodeBundle, PositionType, Query, Res, Style, Text, TextBundle, TextStyle, Val, With, ZIndex};
use bevy::utils::default;
use crate::game_over::components::{GameOverButton, GameOverMenu};
use crate::styles::{BACKGROUND_COLOR, get_button_style, NORMAL_BUTTON};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");
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
        GameOverMenu
    ))
        .with_children(|builder| {
            builder.spawn(NodeBundle {
               style: Style {
                   display: Display::Flex,
                   flex_direction: FlexDirection::Column,
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   width: Val::Percent(25.0),
                   height: Val::Percent(10.),
                   row_gap: Val::Px(8.0),
                   column_gap: Val::Px(8.0),
                   ..default()
               },
                background_color: BACKGROUND_COLOR,
                ..default()
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Defeat, comrade.".to_string(),
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
                    GameOverButton
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

pub fn despawn_ui(mut commands: Commands, game_over_menu_q: Query<Entity, With<GameOverMenu>>) {
    if let Ok(game_over_menu) = game_over_menu_q.get_single() {
        commands.entity(game_over_menu).despawn_recursive();
    }
}