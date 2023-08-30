use bevy::prelude::*;

pub mod components;
pub mod events;
mod resources;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::level::events::SpawnBarriersEvent;

use self::{events::LevelCompletedEvent, resources::Level};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(Level::default())
            // Events
            .add_event::<LevelCompletedEvent>()
            .add_event::<SpawnBarriersEvent>()
            // Startup systems
            .add_systems(Startup, setup_levels) // Happens on app start, not when entering InGame state
            // On enter systems
            .add_systems(OnEnter(AppState::InGame), init_level)
            //Systems
            .add_systems(
                Update,
                (level_complete_event, render_level_data, catch_spawn_barriers_event).run_if(in_state(AppState::InGame)),
            )
            // On exit systems
            .add_systems(OnExit(AppState::InGame), clear_map);
    }
}
