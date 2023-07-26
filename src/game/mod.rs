use bevy::prelude::*;

mod animations;
mod camera;
mod enemy;
mod input;
mod level;
mod player;
mod resources;
mod systems;
mod word_match;

use animations::AnimationPlugin;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use input::TextInputPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use word_match::WordMatchPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((
                PlayerPlugin,
                CameraPlugin,
                EnemyPlugin,
                TextInputPlugin,
                AnimationPlugin,
                LevelPlugin,
                WordMatchPlugin
            ));
    }
}

struct SpriteSheetInfo<'a> {
    path: &'a str, // path from assets folder
    x: f32,        // width of one sprite in sprite sheet
    y: f32,        // height of sprite sheet
    cols: usize,   // how many sprites per row
    rows: usize,   // how many rows of sprites
}
