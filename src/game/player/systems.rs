use bevy::prelude::*;

// use crate::game::animations::components::AnimateSprite;

use super::components::*;

// Not sure if needed, but random player sprite is 64x64
pub const _PLAYER_SIZE: f32 = 64.0;
pub const _PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 1.0),
            texture: asset_server.load("sprites/player_01.png"),
            ..default()
        },
        // AnimateSprite, // player is not using sprite sheet, but will need one eventually!
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
