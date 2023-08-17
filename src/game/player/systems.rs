use bevy::prelude::*;
use crate::game::animations::components::{CharacterAnimations, MovableCharacter};
use crate::game::resources::CharacterHandles;

use super::components::*;

// Not sure if needed, but random player sprite is 64x64
pub const _PLAYER_SIZE: f32 = 64.0;
pub const _PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(mut commands: Commands, character_handles: Res<CharacterHandles>) {

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: character_handles.soviet_idle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 1.0),
            ..default()
        },
        CharacterAnimations::SovietIdle.get_animation(),
        Player,
        MovableCharacter {
            id: 0,
            move_target: None,
            move_timer: Timer::from_seconds(0.0375, TimerMode::Repeating)
        }
    ));
    println!("Spawned player.");
}

pub fn despawn_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    println!("Despawning player.");
    if let Ok(player_entity) = player_q.get_single() {
        commands.entity(player_entity).despawn();
    }
}
