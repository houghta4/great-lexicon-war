use bevy::prelude::*;

use self::pause_menu::PauseMenuPlugin;

mod hud;
mod pause_menu;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(PauseMenuPlugin);
    }
}
