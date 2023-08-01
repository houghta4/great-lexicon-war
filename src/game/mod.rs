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

use crate::AppState;

use self::systems::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resource inserted on app Startup
            .add_systems(Startup, insert_word_bank)
            // Plugins
            .add_plugins((
                PlayerPlugin,
                CameraPlugin,
                EnemyPlugin,
                TextInputPlugin,
                AnimationPlugin,
                LevelPlugin,
                WordMatchPlugin
            ))            
            // Used to display words in console, remove later
            .add_systems(Update, test_words.run_if(in_state(AppState::InGame)));
    }
}

struct SpriteSheetInfo<'a> {
    path: &'a str, // path from assets folder
    x: f32,        // width of one sprite in sprite sheet
    y: f32,        // height of sprite sheet
    cols: usize,   // how many sprites per row
    rows: usize,   // how many rows of sprites
}
