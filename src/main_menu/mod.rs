mod systems;
mod layout;
mod components;
mod interactions;

use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, States, Update};
use crate::AppState;
use crate::main_menu::interactions::interact_with_play_button;
use crate::main_menu::layout::{despawn_main_menu, spawn_main_menu, spawn_faction_menu, despawn_faction_menu, despawn_campaign_menu, spawn_settings_menu, despawn_settings_menu, spawn_soviet_campaign_menu, spawn_german_campaign_menu, spawn_german_loadout_menu, spawn_soviet_loadout_menu, despawn_loadout_menu};
use crate::main_menu::systems::enter_main_menu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MainMenuState>()
            .add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(MainMenuState::Main), spawn_main_menu)
            .add_systems(Update, interact_with_play_button.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(MainMenuState::Main), despawn_main_menu)
            .add_systems(OnEnter(MainMenuState::Faction), spawn_faction_menu)
            .add_systems(OnExit(MainMenuState::Faction), despawn_faction_menu)
            .add_systems(OnEnter(MainMenuState::SovietCampaign), spawn_soviet_campaign_menu)
            .add_systems(OnEnter(MainMenuState::GermanCampaign), spawn_german_campaign_menu)
            .add_systems(OnExit(MainMenuState::SovietCampaign), despawn_campaign_menu)
            .add_systems(OnExit(MainMenuState::GermanCampaign), despawn_campaign_menu)
            .add_systems(OnEnter(MainMenuState::SovietLoadout), spawn_soviet_loadout_menu)
            .add_systems(OnEnter(MainMenuState::GermanLoadout), spawn_german_loadout_menu)
            .add_systems(OnExit(MainMenuState::SovietLoadout), despawn_loadout_menu)
            .add_systems(OnExit(MainMenuState::GermanLoadout), despawn_loadout_menu)
            .add_systems(OnEnter(MainMenuState::Settings), spawn_settings_menu)
            .add_systems(OnExit(MainMenuState::Settings), despawn_settings_menu);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Main,
    Faction,
    SovietCampaign,
    GermanCampaign,
    SovietLoadout,
    GermanLoadout,
    Settings,
    Exit
}
