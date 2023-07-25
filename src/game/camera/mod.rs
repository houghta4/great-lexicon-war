//! CameraPlugin
//! Pan to track player, zoom in/out to show battlefield

use bevy::prelude::*;

mod systems;

use systems::*;

use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            camera_follow_player.run_if(in_state(AppState::InGame)),
        ); // may want PostUpdate schedule
    }
}
