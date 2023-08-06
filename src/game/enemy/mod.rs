use bevy::prelude::*;

mod components;
pub mod events;
mod resources;
mod systems;

use systems::*;

use crate::game::enemy::events::EnemyShotEvent;
use crate::AppState;

use self::resources::{EnemySpawnCount, EnemySpawnTimer};
use super::InGameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EnemyShotEvent>()
            // Systems
            .add_systems(Update, init_enemy_level_info)
            .add_systems(
                Update,
                (despawn_enemies, spawn_initial_enemies)
                    .chain()
                    .run_if(resource_exists_and_changed::<EnemySpawnCount>()),
            )
            .add_systems(
                Update,
                (catch_shot_event,)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            .add_systems(
                Update,
                (tick_enemy_spawn_timer, spawn_enemies_gradually)
                    .run_if(resource_exists::<EnemySpawnTimer>())
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            // On Exit systems
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}
