//! Things like play/pause systems should go here

use bevy::{prelude::*, window::PrimaryWindow};

// Really janky, but wanted to get a placeholder
// I have no idea what the correct way to do this is
// We're going to either need to create level backgrounds with tilesets or a high res image that we can stretch like this for the entire level
// Another option is make the backgrounds able to be pieced together, so if we see a boundary we can load in the right image
pub fn spawn_game_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_q: Query<&Window, With<PrimaryWindow>>,
) {
    println!("Spawning background image");
    let win = win_q.get_single().unwrap();
    let bg_img = asset_server.load("sprites/forest_01.png");
    let bg_img_x: f32 = 480.0;
    let bg_img_y: f32 = 272.0;

    commands.spawn(SpriteBundle {
        texture: bg_img,
        transform: Transform {
            translation: Vec3::new(win.width() / 2.0, win.height() / 2.0, 0.0),
            scale: Vec3::new(
                win.width() * 1.5 / bg_img_x,
                win.height() * 1.5 / bg_img_y,
                0.0,
            ),
            ..default()
        },
        ..default()
    });
}
