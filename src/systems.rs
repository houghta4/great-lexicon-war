//! Systems that do not belong to any plugins

use bevy::prelude::*;

use crate::components::*;

pub fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera");
    commands.spawn((Camera2dBundle::default(), GameCamera {}));
}
