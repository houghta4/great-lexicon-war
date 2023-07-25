use bevy::prelude::*;

use crate::components::*;
use crate::game::player::components::*;

pub fn camera_follow_player(
    mut camera_q: Query<&mut Transform, With<GameCamera>>,
    player_q: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        if let Ok(mut camera_transform) = camera_q.get_single_mut() {
            // println!("Updating camera");
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        } else {
            println!("Camera query was not Ok.")
        }
    } else {
        println!("Player query was not Ok.");
    }
}
