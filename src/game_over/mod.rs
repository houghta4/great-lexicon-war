mod layout;
mod interactions;
mod components;

use bevy::prelude::{App, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};
use crate::AppState;
use crate::game_over::interactions::interact_with_game_over_button;
use crate::game_over::layout::{despawn_ui, spawn_ui};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), spawn_ui)
            .add_systems(Update, interact_with_game_over_button.run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), despawn_ui);
    }
}