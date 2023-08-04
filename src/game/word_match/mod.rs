use crate::game::word_match::systems::check_matches;
use crate::AppState;
use bevy::prelude::*;

use super::InGameState;

pub mod components;
mod systems;

pub struct WordMatchPlugin;

impl Plugin for WordMatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_matches
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(InGameState::Running)),
        );
    }
}
