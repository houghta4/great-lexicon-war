mod components;
mod events;
mod game;
mod game_over;
mod main_menu;
mod systems;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use game::InGamePlugin;
use systems::*;

fn main() {
    App::new()
        // Default plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Great Lexicon War".into(),
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::AutoVsync,
                        // wasm settings
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                }),
        )
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
