//! Systems that do not belong to any plugins

use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;

pub fn spawn_camera(mut commands: Commands, win_q: Query<&Window, With<PrimaryWindow>>) {
    let win = win_q.get_single().unwrap();
    println!("Spawning camera");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(win.width() / 2.0, win.height() / 2.0, 999.0), // camera should be highest z index
            ..default()
        },
        GameCamera {},
    ));
}
