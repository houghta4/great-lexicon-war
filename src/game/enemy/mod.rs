use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

use crate::AppState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Start up systems
            .add_systems(
                OnEnter(AppState::InGame),
                (spawn_single_enemy, spawn_enemies),
            )
            // Systems
            .add_systems(
                Update,
                (
                    animate_sprite,
                    // print_enemy_words // constant flow of all enemy words
                )
                    .run_if(in_state(AppState::InGame)),
            )
            // On Exit systems
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}
