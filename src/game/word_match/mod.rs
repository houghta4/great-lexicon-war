use crate::game::word_match::systems::{check_matches, clear_word};
use bevy::prelude::*;

use super::InGameRunning;

pub mod components;
mod systems;

pub struct WordMatchPlugin;

impl Plugin for WordMatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_matches, clear_word).in_set(InGameRunning));
    }
}
