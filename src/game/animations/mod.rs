use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use super::InGameRunning;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite.in_set(InGameRunning));
    }
}
