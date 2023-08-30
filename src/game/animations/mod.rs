use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::animations::events::CharacterMoveEvent;

use super::InGameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprite, move_character, catch_character_move_event)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(InGameState::Running)),
        ).add_event::<CharacterMoveEvent>();
    }
}
