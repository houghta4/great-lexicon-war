use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite.run_if(in_state(AppState::InGame)));
    }
}
