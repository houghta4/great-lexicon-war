mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::OnEnter;
use crate::AppState;
use crate::game::ui::hud::systems::spawn_ui;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_ui);
    }
}