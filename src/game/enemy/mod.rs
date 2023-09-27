use bevy::prelude::*;

mod components;
pub mod events;
mod resources;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::enemy::events::EnemyShotEvent;

use self::resources::EnemySpawns;
use super::level::events::LevelInitEvent;
use super::InGameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EnemyShotEvent>()
            // Systems
            .add_systems(
                Update,
                init_enemy_level_info.run_if(on_event::<LevelInitEvent>()),
            )
            .add_systems(
                Update,
                (despawn_enemies, spawn_initial_enemies)
                    .chain()
                    .run_if(resource_exists_and_changed::<EnemySpawns>()),
            )
            .add_systems(
                Update,
                (
                    catch_shot_event,
                    enemy_shoot_player,
                    tick_and_replace_enemy_fire_timer,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            // On Exit systems
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}
