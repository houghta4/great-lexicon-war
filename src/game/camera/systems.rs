use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::game::player::components::*;

pub fn camera_follow_player(
    mut camera_q: Query<&mut Transform, With<GameCamera>>,
    player_q: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    win_q: Query<&Window, With<PrimaryWindow>>,
) {
    let win = win_q.get_single().unwrap();
    if let Ok(player_transform) = player_q.get_single() {
        if let Ok(mut camera_transform) = camera_q.get_single_mut() {
            // Keep player within the screen's edges

            // Update x values
            if player_transform.translation.x > camera_transform.translation.x + win.width() / 4. {
                camera_transform.translation.x += 1.;
            }
            if player_transform.translation.x < camera_transform.translation.x - win.width() / 4. {
                camera_transform.translation.x -= 1.;
            }

            // Update y values
            if player_transform.translation.y > camera_transform.translation.y + win.height() / 4. {
                camera_transform.translation.y += 1.;
            }
            if player_transform.translation.y < camera_transform.translation.y - win.height() / 4. {
                camera_transform.translation.y -= 1.;
            }
        } else {
            println!("Camera query was not Ok.")
        }
    } else {
        println!("Player query was not Ok.");
    }
}
