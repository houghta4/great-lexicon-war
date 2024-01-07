use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::level::events::SpawnMovePointsEvent;
use crate::game::level::resources::LevelInfo;

use self::events::{EnemyKilledEvent, LevelCompletedEvent, LevelInitEvent, ProgressEvent, TypoEvent};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(LevelInfo::default())
            // Events
            .add_event::<LevelCompletedEvent>()
            .add_event::<LevelInitEvent>()
            .add_event::<SpawnMovePointsEvent>()
            .add_event::<EnemyKilledEvent>()
            .add_event::<ProgressEvent>()
            .add_event::<TypoEvent>()
            // Startup systems
            .add_systems(Startup, setup_levels) // Happens on app start, not when entering InGame state
            // On enter systems
            .add_systems(OnEnter(AppState::InGame), render_level_data)
            //Systems
            .add_systems(
                Update,
                (level_complete_event, catch_spawn_move_points_event).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                catch_level_completed_event.run_if(on_event::<LevelCompletedEvent>()),
            )
            .add_systems(
                Update,
                catch_enemy_killed_event.run_if(on_event::<EnemyKilledEvent>()),
            )
            .add_systems(
                Update,
                catch_progress_event.run_if(on_event::<ProgressEvent>()),
            )
            .add_systems(
                Update,
                catch_typo_event.run_if(on_event::<TypoEvent>()),
            )
            // On exit systems
            .add_systems(OnExit(AppState::InGame), clear_map);
    }
}
