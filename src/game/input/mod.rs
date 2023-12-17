use bevy::prelude::*;

use crate::game::input::resource::BackspaceTimer;
use crate::game::input::systems::*;
use crate::AppState;

use self::events::InputEvent;
use super::InGameState;

pub mod components;
pub mod events;
mod resource;
mod systems;

pub struct TextInputPlugin;

/**
    Plugin for text input, core way the user interacts with the game
**/
impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<InputEvent>()
            .add_systems(
                Update,
                (
                    listen_received_character_events,
                    listen_keyboard_input_events,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            .insert_resource(BackspaceTimer(Timer::from_seconds(
                0.075,
                TimerMode::Repeating,
            )));
    }
}
