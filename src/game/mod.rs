use bevy::prelude::*;

mod camera;
mod enemy;
mod player;
mod systems;

use camera::CameraPlugin;
use player::PlayerPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, CameraPlugin));
    }
}
