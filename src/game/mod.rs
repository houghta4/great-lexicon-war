use bevy::prelude::*;

mod camera;
mod enemy;
mod player;
mod resources;
mod systems;

use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use resources::*;
use systems::*;

use crate::AppState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((PlayerPlugin, CameraPlugin, EnemyPlugin))
            // Resources
            .init_resource::<Level>()
            // Startup systems
            .add_systems(OnEnter(AppState::InGame), spawn_game_background);
    }
}

struct SpriteSheetInfo<'a> {
    path: &'a str, // path from assets folder
    x: f32,        // width of one sprite in sprite sheet
    y: f32,        // height of sprite sheet
    cols: usize,   // how many sprites per row
    rows: usize,   // how many rows of sprites
}
