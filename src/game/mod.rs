use bevy::prelude::*;

mod animations;
mod camera;
mod enemy;
mod input;
mod level;
mod player;
mod resources;
mod systems;
mod ui;
mod utils;
mod word_match;

use animations::AnimationPlugin;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use input::TextInputPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use word_match::WordMatchPlugin;

use crate::AppState;

use systems::*;

use self::ui::GameUIPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // System sets
            .configure_set(
                Update,
                InGameRunning
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            // Resource inserted on app Startup
            .add_systems(Startup, (insert_word_bank, init_texture_atlas_handles))
            // States
            .add_state::<InGameState>()
            // On enter state
            .add_systems(OnEnter(AppState::InGame), pause_game) // start paused for now
            // Plugins
            .add_plugins((
                PlayerPlugin,
                CameraPlugin,
                EnemyPlugin,
                TextInputPlugin,
                AnimationPlugin,
                LevelPlugin,
                WordMatchPlugin,
                GameUIPlugin,
            ))
            .add_systems(
                Update,
                (test_words, toggle_game_state, monitor_state).run_if(in_state(AppState::InGame)),
            )
            // This may not be needed
            .add_systems(OnExit(AppState::InGame), resume_game);
    }
}
/// All systems that run while in states AppState::InGame and InGameState::Running should belong to this set
///
/// # Example
/// `app.add_systems(<schedule>, <systems>.in_set(InGameRunning))`
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InGameRunning;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum InGameState {
    #[default]
    Running,
    Paused,
}

struct SpriteSheetInfo<'a> {
    path: &'a str, // path from assets folder
    x: f32,        // width of one sprite in sprite sheet
    y: f32,        // height of sprite sheet
    cols: usize,   // how many sprites per row
    rows: usize,   // how many rows of sprites
}

pub enum WordComplexity {
    Easy,
    Medium,
    Hard,
    Extreme,
}

