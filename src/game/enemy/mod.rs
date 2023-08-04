use bevy::prelude::*;

mod components;
mod systems;
pub mod events;

use systems::*;

use crate::AppState;
use crate::game::enemy::events::EnemyShotEvent;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnemyShotEvent>()
            // Start up systems
            .add_systems(
                OnEnter(AppState::InGame),
                (spawn_single_enemy, spawn_enemies),
            )
            .add_systems(Update, catch_shot_event.run_if(in_state(AppState::InGame)))
            // On Exit systems
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}
