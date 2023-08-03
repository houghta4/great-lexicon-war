use bevy::prelude::*;

use crate::game::InGameState;
use systems::layout::*;

use self::systems::interactions::interact_with_resume_button;

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(OnEnter(InGameState::Paused), spawn_pause_menu)
            // Systems
            .add_systems(
                Update,
                interact_with_resume_button.run_if(in_state(InGameState::Paused)),
            )
            // OnExit Systems
            .add_systems(OnExit(InGameState::Paused), despawn_pause_menu);
    }
}
