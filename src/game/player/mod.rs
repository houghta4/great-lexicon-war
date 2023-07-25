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
            // Systems
            .add_systems(Update, player_movement.run_if(in_state(AppState::InGame)))
            // On exit InGame state
            .add_systems(OnExit(AppState::InGame), despawn_player);
    }
}
