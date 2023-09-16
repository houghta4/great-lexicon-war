use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::level::events::SpawnMovePointsEvent;
use crate::game::level::resources::Level;

use self::events::{LevelCompletedEvent, LevelInitEvent};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(Level::default())
            // Events
            .add_event::<LevelCompletedEvent>()
            .add_event::<LevelInitEvent>()
            .add_event::<SpawnMovePointsEvent>()
            // Startup systems
            .add_systems(Startup, setup_levels) // Happens on app start, not when entering InGame state
            // On enter systems
            .add_systems(OnEnter(AppState::InGame), init_level)
            //Systems
            .add_systems(
                Update,
                (level_complete_event, render_level_data, catch_spawn_move_points_event).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                catch_level_completed_event.run_if(on_event::<LevelCompletedEvent>()),
            )
            // On exit systems
            .add_systems(OnExit(AppState::InGame), clear_map);
    }
}
