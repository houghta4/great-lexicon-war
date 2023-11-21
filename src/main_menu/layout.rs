use bevy::prelude::*;
use bevy::utils::default;
use crate::main_menu::components::*;
use crate::main_menu::resources::SaveInfo;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands.spawn((
        NodeBundle {
           style: Style {
               display: Display::Grid,
               width: Val::Percent(100.0),
               height: Val::Percent(100.0),
               grid_template_columns: RepeatedGridTrack::flex(3, 1.),
               grid_template_rows: RepeatedGridTrack::flex(5, 1.),
               flex_direction: FlexDirection::Column,
               ..default()
           },
        ..default()
        },
        MainMenu
    )).with_children(|builder| {
            builder.spawn(NodeBundle {
                style: Style {
                    grid_column: GridPlacement::span(3),
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Great Lexicon War".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::BLACK
                    }
                ));
            });
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        grid_column: GridPlacement::start(2),
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                    ..default()
                },
                MenuButton(ButtonType::Play)
            )).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Play".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK
                    }
                ));
            });
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        grid_row: GridPlacement::start(3),
                        grid_column: GridPlacement::start(2),
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                    ..default()
                },
                MenuButton(ButtonType::Settings)
            )).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Settings".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK
                    }
                ));
            });
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        grid_row: GridPlacement::start(4),
                        grid_column: GridPlacement::start(2),
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                    ..default()
                },
                MenuButton(ButtonType::Quit)
            )).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Quit".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK
                    }
                ));
            });
        });
}

pub fn despawn_main_menu(mut commands: Commands, menu_q: Query<Entity, With<MainMenu>>) {
    if let Ok(menu) = menu_q.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

pub fn spawn_faction_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(3, 1.),
                grid_template_rows: RepeatedGridTrack::flex(5, 1.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        FactionMenu
    )).with_children(|builder| {
        builder.spawn(NodeBundle {
            style: Style {
                grid_column: GridPlacement::span(3),
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Factions".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::BLACK
                }
            ));
        });
        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_row: GridPlacement::start(3),
                    margin: UiRect::left(Val::Px(50.)),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::Faction(Faction::Soviet))
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Soviet".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });
        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_column: GridPlacement::start(3),
                    grid_row: GridPlacement::start(3),
                    margin: UiRect::right(Val::Px(50.)),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::Faction(Faction::German))
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "German".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });
        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_column: GridPlacement::start(2),
                    grid_row: GridPlacement::start(5),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::ReturnToMain)
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Return To Menu".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });
    });
}

pub fn despawn_faction_menu(mut commands: Commands, menu_q: Query<Entity, With<FactionMenu>>) {
    if let Ok(menu) = menu_q.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

pub fn spawn_german_campaign_menu(commands: Commands, asset_server: Res<AssetServer>, save_info: Res<SaveInfo>) {
    spawn_campaign_menu(commands, asset_server, save_info, Faction::German);
}

pub fn spawn_soviet_campaign_menu(commands: Commands, asset_server: Res<AssetServer>, save_info: Res<SaveInfo>) {
    spawn_campaign_menu(commands, asset_server, save_info, Faction::Soviet);
}

fn spawn_campaign_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    save_info: Res<SaveInfo>,
    faction: Faction) {
    let faction_status = save_info.get_faction_status(faction);
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(5, 1.),
                grid_template_rows: RepeatedGridTrack::flex(5, 1.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        CampaignMenu
    )).with_children(|builder| {
        builder.spawn(NodeBundle {
            style: Style {
                grid_column: GridPlacement::span(5),
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                format!("Campaign - {}", faction).to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::BLACK
                }
            ));
        });

        for i in 0..faction_status.levels.len() {
            let level_status = &faction_status.levels[i];
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::horizontal(Val::Percent(2.5)),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                    ..default()
                },
                MenuButton(ButtonType::LevelSelect(faction, i))
            )).with_children(|builder| {
                builder.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(1, 1.),
                        grid_template_rows: RepeatedGridTrack::flex(2, 1.),
                        flex_direction: FlexDirection::Column,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::vertical(Val::Percent(15.)),
                        ..default()
                    },
                    ..default()
                }).with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        level_status.id.to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::BLACK
                        }
                    ));
                    if !level_status.locked {
                        builder.spawn(TextBundle::from_sections(vec![
                            TextSection::new(format!("{}/100", level_status.points).to_string(),
                                             TextStyle {
                                                 font: font.clone(),
                                                 font_size: 20.0,
                                                 color: Color::BLACK
                                             }
                            )
                        ]));
                    } else {
                        builder.spawn(TextBundle::from_section(
                           "Locked".to_string(),
                           TextStyle {
                               font: font.clone(),
                               font_size: 20.0,
                               color: Color::BLACK
                           }
                        ));
                    }
                });
            });
        }

        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_column: GridPlacement::start_end(2, 3),
                    grid_row: GridPlacement::start(5),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::Loadout(faction))
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Loadout".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });

        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_column: GridPlacement::start_end(4, 5),
                    grid_row: GridPlacement::start(5),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::ReturnToMain)
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Return To Menu".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });
    });
}

pub fn despawn_campaign_menu(mut commands: Commands, menu_q: Query<Entity, With<CampaignMenu>>) {
    if let Ok(menu) = menu_q.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

pub fn spawn_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(3, 1.),
                grid_template_rows: RepeatedGridTrack::flex(5, 1.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        SettingsMenu
    )).with_children(|builder| {
        builder.spawn(NodeBundle {
            style: Style {
                grid_column: GridPlacement::span(3),
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Settings".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::BLACK
                }
            ));
        });

        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_column: GridPlacement::start(2),
                    grid_row: GridPlacement::start(4),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::ReturnToMain)
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Return To Menu".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });
    });
}

pub fn despawn_settings_menu(mut commands: Commands, menu_q: Query<Entity, With<SettingsMenu>>) {
    if let Ok(menu) = menu_q.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

pub fn spawn_german_loadout_menu(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_loadout_menu(commands, asset_server, Faction::German);
}

pub fn spawn_soviet_loadout_menu(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_loadout_menu(commands, asset_server, Faction::Soviet);
}

fn spawn_loadout_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    faction: Faction) {
    let font: Handle<Font> = asset_server.load("fonts/propaganda/propaganda.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(3, 1.),
                grid_template_rows: RepeatedGridTrack::flex(5, 1.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        LoadoutMenu
    )).with_children(|builder| {
        builder.spawn(NodeBundle {
            style: Style {
                grid_column: GridPlacement::span(3),
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                format!("Loadout - {}", faction).to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::BLACK
                }
            ));
        });

        builder.spawn((
            ButtonBundle {
                style: Style {
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    grid_column: GridPlacement::start(2),
                    grid_row: GridPlacement::start(4),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/ui/button.png")),
                ..default()
            },
            MenuButton(ButtonType::ReturnToMain)
        )).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Return To Menu".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK
                }
            ));
        });
    });
}

pub fn despawn_loadout_menu(mut commands: Commands, menu_q: Query<Entity, With<LoadoutMenu>>) {
    if let Ok(menu) = menu_q.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}
