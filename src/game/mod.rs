use bevy::prelude::*;

mod animations;
mod camera;
mod enemy;
mod player;
mod resources;
mod systems;

use animations::AnimationPlugin;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((PlayerPlugin, CameraPlugin, EnemyPlugin, AnimationPlugin));
    }
}

struct SpriteSheetInfo<'a> {
    path: &'a str, // path from assets folder
    x: f32,        // width of one sprite in sprite sheet
    y: f32,        // height of sprite sheet
    cols: usize,   // how many sprites per row
    rows: usize,   // how many rows of sprites
}
