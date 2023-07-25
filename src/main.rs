mod components;
mod events;
mod game;
mod game_over;
mod main_menu;
mod systems;

use bevy::prelude::*;
use game::InGamePlugin;
use systems::*;

fn main() {
    App::new()
        // Default plugins
        .add_plugins(DefaultPlugins)
        //States
        .add_state::<AppState>()
        // Custom plugins
        .add_plugins(InGamePlugin)
        // Startup Systems
        .add_systems(Startup, spawn_camera)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    // mm should be default once it is created
    // #[default]
    MainMenu,
    #[default]
    InGame, // inside Game plug in have Running and Paused states
    GameOver,
}
