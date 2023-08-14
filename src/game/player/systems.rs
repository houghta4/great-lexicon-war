use bevy::prelude::*;

// use crate::game::animations::components::AnimateSprite;

use crate::game::enemy::events::EnemyShotEvent;

use super::{
    components::*,
    events::{PlayerReloadEvent, PlayerShotEvent},
};

// Not sure if needed, but random player sprite is 64x64
pub const _PLAYER_SIZE: f32 = 64.0;
pub const _PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_DAMAGE: f32 = 5.0;

/// Spawn `Player` entity
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 1.0),
            texture: asset_server.load("sprites/player_01.png"),
            ..default()
        },
        // AnimateSprite, // player is not using sprite sheet, but will need one eventually!
        Player::default(),
    ));
    println!("Spawned player.");
}

/// Remove `Player` entity
pub fn despawn_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    println!("Despawning player.");
    if let Ok(player_entity) = player_q.get_single() {
        commands.entity(player_entity).despawn();
    }
}

/// Whenever a `PlayerShotEvent` happens, the player will lose health
///
/// This is triggered by an `Enemy`
pub fn player_take_damage(
    mut player_q: Query<&mut Player>,
    mut player_shot_event_reader: EventReader<PlayerShotEvent>,
) {
    for _ in player_shot_event_reader.iter() {
        if let Ok(mut player) = player_q.get_single_mut() {
            if player.health - PLAYER_DAMAGE >= 0.0 {
                player.health -= PLAYER_DAMAGE;
            } else {
                player.health = 0.0;
            }
        }
    }
}

/// Whenever a `EnemyShotEvent` happens, the player will lose ammo
///
/// This is triggered by the `Player` when a `WordTarget::Enemy` is typed
pub fn player_shot_enemy(
    mut player_q: Query<&mut Player>,
    mut enemy_shot_event_reader: EventReader<EnemyShotEvent>,
) {
    for _ in enemy_shot_event_reader.iter() {
        if let Ok(mut player) = player_q.get_single_mut() {
            if player.ammo.0 > 0 {
                player.ammo.0 -= 5; // TODO: subtract by 1 when finished testing
            } else {
                player.ammo.0 = 0;
            }
        }
    }
}

/// Whenever a `PlayerReloadEvent` happens, the player's ammo will be replenished
///
/// This is triggered by the `Player` when a `WordTarget::Reload` is typed
pub fn player_reload(
    mut player_q: Query<&mut Player>,
    mut reload_event_reader: EventReader<PlayerReloadEvent>,
) {
    for _ in reload_event_reader.iter() {
        if let Ok(mut player) = player_q.get_single_mut() {
            player.ammo.0 = player.ammo.1;
            println!("<< reloading >>");
        }
    }
}
