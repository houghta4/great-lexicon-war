use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::InGameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_sprite
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(InGameState::Running)),
        );
    }
}
