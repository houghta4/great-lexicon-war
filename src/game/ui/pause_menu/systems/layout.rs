use bevy::prelude::*;

use crate::game::ui::pause_menu::{components::*, styles::*};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(mut commands: Commands, pause_menu_q: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = pause_menu_q.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            // this is transparent, takes up whole screen
            NodeBundle {
                style: get_pause_menu_style(),
                z_index: ZIndex::Local(1),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent
                // probably want to use ImageBundle when/if we get a nice pause menu image created
                .spawn(NodeBundle {
                    style: get_pause_menu_container_style(),
                    background_color: BACKGROUND_COLOR,
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Pause Menu",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    // Resume button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ResumeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    pause_menu_entity
}
