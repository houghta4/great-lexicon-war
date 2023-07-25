use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;

// Not sure if needed, but random player sprite is 64x64
pub const _PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // normally can't just unwrap, but this is guaranteed to exist from Bevy
    let win = win_q.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(win.width() / 2.0, win.height() / 2.0, 1.0),
            texture: asset_server.load("sprites/player_01.png"),
            ..default()
        },
        Player {},
    ));
    println!("Spawned player.");
}

pub fn despawn_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    println!("Despawning player.");
    if let Ok(player_entity) = player_q.get_single() {
        commands.entity(player_entity).despawn();
    }
}

// TODO: Will not be using wasd or arrow keys. This was for camera tracking testing
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // shouldn't unwrap because our player entity isn't guaranteed to exist like the window entity is in spawn_player
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            println!("movin");
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            println!("movin");
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            println!("movin");
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            println!("movin");
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        //time.delta_seconds() can be thought of as time (as f32) since the last frame
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
