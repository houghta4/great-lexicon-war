use bevy::prelude::{AssetServer, Interaction, NextState, Query, Res, ResMut, UiImage};
use bevy::ecs::event::Events;
use bevy::app::AppExit;
use crate::AppState;
use crate::main_menu::components::{ButtonType, Faction, MenuButton};
use crate::main_menu::MainMenuState;
use crate::utils::ChangedWith;
use crate::game::level::resources::Level;
use crate::main_menu::resources::SaveInfo;

pub fn interact_with_play_button(
    asset_server: Res<AssetServer>,
    mut button_q: Query<(&Interaction, &mut UiImage, &MenuButton), ChangedWith<MenuButton>>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_game_state: ResMut<NextState<AppState>>,
    mut level: ResMut<Level>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    save_info: Res<SaveInfo>
) {
    let button_interact = asset_server.load("sprites/ui/button_interact.png");
    let button = asset_server.load("sprites/ui/button.png");
    for (interaction, mut image, menu_button) in button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                //TODO: causes flickering, fix?
                image.texture = button_interact.clone();
                match menu_button.0 {
                    ButtonType::Play => {
                        next_menu_state.set(MainMenuState::Faction);
                    }
                    ButtonType::Settings => {
                        next_menu_state.set(MainMenuState::Settings);
                    }
                    ButtonType::Quit => {
                        app_exit_events.send(AppExit);
                    }
                    ButtonType::Faction(faction) => {
                        match faction {
                            Faction::Soviet => {
                                next_menu_state.set(MainMenuState::SovietCampaign);
                            }
                            Faction::German => {
                                next_menu_state.set(MainMenuState::GermanCampaign);
                            }
                        }
                    }
                    ButtonType::Loadout(faction) => {
                        match faction {
                            Faction::Soviet => {
                                next_menu_state.set(MainMenuState::SovietLoadout);
                            }
                            Faction::German => {
                                next_menu_state.set(MainMenuState::GermanLoadout);
                            }
                        }
                    }
                    ButtonType::LevelSelect(faction, id) => {
                        if !save_info.get_faction_status(faction).levels[id].locked {
                            level.0 = id;
                            next_menu_state.set(MainMenuState::Exit);
                            next_game_state.set(AppState::InGame);
                        }
                    }
                    ButtonType::ReturnToMain => {
                        next_menu_state.set(MainMenuState::Main);
                    }
                }
            }
            Interaction::Hovered => {
                image.texture = button_interact.clone();
            }
            Interaction::None => {
                image.texture = button.clone();
            }
        }
    }
}