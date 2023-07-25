use bevy::prelude::*;

mod camera;
mod enemy;
mod player;
mod systems;
mod input;

use camera::CameraPlugin;
use player::PlayerPlugin;
use input::TextInputPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, CameraPlugin, TextInputPlugin));
    }
}
