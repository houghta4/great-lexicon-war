use bevy::prelude::*;
use crate::AppState;
use crate::game::word_match::components::WordEvent;
use crate::game::word_match::systems::{catch_events, check_matches};

mod components;
mod systems;

pub struct WordMatchPlugin;

impl Plugin for WordMatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WordEvent>()
            .add_systems(Update, (check_matches, catch_events).run_if(in_state(AppState::InGame)));
    }
}