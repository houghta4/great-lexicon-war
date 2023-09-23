mod layout;
mod interactions;
mod components;

use bevy::prelude::{App, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};
use crate::AppState;
use crate::level_completed::interactions::interact_with_level_complete_button;
use crate::level_completed::layout::{despawn_ui, spawn_ui};

pub struct LevelCompletedPlugin;

impl Plugin for LevelCompletedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::LevelCompleted), spawn_ui)
            .add_systems(Update, interact_with_level_complete_button.run_if(in_state(AppState::LevelCompleted)))
            .add_systems(OnExit(AppState::LevelCompleted), despawn_ui);
    }
}