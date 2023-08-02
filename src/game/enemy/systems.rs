use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{AnimationIndices, AnimationTimer};
use crate::game::animations::components::AnimateSprite;
use crate::game::enemy::components::*;
use crate::game::word_match::components::Word;
use crate::game::SpriteSheetInfo;

// https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_sheet.rs

const SOLDIER_01_IDLE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soldier_01/Idle.png",
    x: 128.0,
    y: 128.0,
    cols: 7,
    rows: 1,
};
const SOLDIER_01_RUN: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soldier_01/Run.png",
    x: 128.0,
    y: 128.0,
    cols: 7,
    rows: 1,
};

pub fn spawn_single_enemy(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let win = win_q.get_single().unwrap();

    let cur_sprite = SOLDIER_01_IDLE;

    let texture_handle = asset_server.load(cur_sprite.path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(cur_sprite.x, cur_sprite.y),
        cur_sprite.cols,
        cur_sprite.rows,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 6 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(win.width() / 2.0 - 50.0, win.height() / 2.0, 1.0),

            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimateSprite,
        Enemy {
            word: EnemyWord {
                word: "zoinks".into(),
                ..default()
            },
            ..default()
        },
    ));
    println!("Spawned single enemy.");
}

pub fn despawn_enemies(mut commands: Commands, enemy_q: Query<Entity, With<Enemy>>) {
    println!("Removing all enemies");
    enemy_q.iter().for_each(|enemy_entity| {
        commands.entity(enemy_entity).despawn();
    });
}

// This will change a lot once
pub fn spawn_enemies(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let win = win_q.get_single().unwrap();

    let words = vec!["cat", "rat", "delicious", "delete", "banana"];

    for (i, w) in words.iter().enumerate() {
        let cur_sprite = SOLDIER_01_RUN;

        let texture_handle = asset_server.load(cur_sprite.path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(cur_sprite.x, cur_sprite.y),
            cur_sprite.cols,
            cur_sprite.rows,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 6 };
        let x_offset = i * 130;
        let rotate = if i % 2 == 0 {
            Quat::from_rotation_y(std::f32::consts::PI)
        } else {
            Quat::default()
        };
        let transform = Transform::from_xyz(
            win.width() / 4.0 + (x_offset as f32),
            win.height() / 2.0 - 130.0,
            1.0,
        )
        .with_rotation(rotate);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform,
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            AnimateSprite,
            Enemy {
                word: EnemyWord {
                    word: w.to_string(),
                    ..default()
                },
                ..default()
            },
            Word(w.len(), w.to_string()), // for testing word uniqueness
        ));
        println!("Spawned enemy: {w}.");
    }
}

#[allow(dead_code)]
pub fn print_enemy_words(enemy_q: Query<&Enemy, With<Enemy>>) {
    if enemy_q.is_empty() {
        println!("empty enemy_word query");
    } else {
        println!("<<--->>");
    }
    enemy_q
        .iter()
        .for_each(|enemy| println!("enemy_word: {}", enemy.word));
}
