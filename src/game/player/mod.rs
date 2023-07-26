use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On enter InGame state
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            // On exit InGame state
            .add_systems(OnExit(AppState::InGame), despawn_player);
    }
}
