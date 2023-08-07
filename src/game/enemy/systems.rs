use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::components::{AnimationIndices, AnimationTimer};
use crate::game::animations::components::AnimateSprite;
use crate::game::enemy::components::*;
use crate::game::enemy::events::EnemyShotEvent;
use crate::game::enemy::resources::EnemySpawns;
use crate::game::input::components::InputText;
use crate::game::level::components::LevelInfo;
use crate::game::level::events::LevelCompletedEvent;
use crate::game::resources::{RandomWord, WordBank};
use crate::game::utils::spawn_word;
use crate::game::word_match::components::{Word, WordTarget};
use crate::game::{SpriteSheetInfo, WordComplexity};

use super::resources::{EnemySpawnTimer, PlayerHandles};

// https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_sheet.rs

#[allow(dead_code)]
const SOLDIER_01_IDLE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soldier_01/Idle.png",
    x: 128.0,
    y: 128.0,
    cols: 7,
    rows: 1,
};

#[allow(dead_code)]
const SOLDIER_01_RUN: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soldier_01/Run.png",
    x: 128.0,
    y: 128.0,
    cols: 7,
    rows: 1,
};

pub fn init_texture_atlas_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
    let texture_atlas_idle_handle = texture_atlases.add(texture_atlas);

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
    let texture_atlas_run_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(PlayerHandles {
        idle: texture_atlas_idle_handle,
        run: texture_atlas_run_handle,
    });
}

/**
    Spawn health bar for enemy
    NOTE: should move somewhere common if other things need health bars
**/
fn spawn_health_bar(builder: &mut ChildBuilder) {
    builder.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(80.0, 2.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 25., 3.)),
            ..default()
        },
        HealthBar,
    ));
}

pub fn despawn_enemies(mut commands: Commands, enemy_q: Query<Entity, With<Enemy>>) {
    println!("Removing all enemies");
    enemy_q.iter().for_each(|enemy_entity| {
        commands.entity(enemy_entity).despawn_recursive();
    });
}

pub fn spawn_initial_enemies(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    enemy_spawn: Res<EnemySpawns>,
    player_handles: Res<PlayerHandles>,
) {
    println!("Spawning initial enemies");
    let _win = win_q.get_single().unwrap();
    let font = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    for pos in enemy_spawn.enemies.as_slice() {
        // let cur_sprite = SOLDIER_01_IDLE;

        // let texture_handle = asset_server.load(cur_sprite.path);
        // let texture_atlas = TextureAtlas::from_grid(
        //     texture_handle,
        //     Vec2::new(cur_sprite.x, cur_sprite.y),
        //     cur_sprite.cols,
        //     cur_sprite.rows,
        //     None,
        //     None,
        // );
        // let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 6 };

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: player_handles.idle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_xyz(pos.x, pos.y, 1.),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                AnimateSprite,
                Enemy::default(),
            ))
            .with_children(|builder| {
                spawn_health_bar(builder);
                spawn_word(
                    builder,
                    word_bank.get_word(WordComplexity::Medium, &word_q).as_str(),
                    &font,
                );
            });
    }
}

/// Set resources once we progress to a new level
pub fn init_enemy_level_info(
    mut commands: Commands,
    level_info_q: Query<&LevelInfo>,
    mut level_complete_event_reader: EventReader<LevelCompletedEvent>,
) {
    for level in level_complete_event_reader.iter() {
        if let Some(level_info) = level_info_q.iter().nth(level.0) {
            println!("Inserting resources");
            commands.insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(level_info.spawn_rate, TimerMode::Repeating),
            });
            commands.insert_resource(EnemySpawns {
                enemies: level_info.enemies.clone(),
            });
        }
    }
}

/// Tick spawn timer while in game and unpaused
pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

/// Spawn enemies over time depending on the current level's `spawn_rate`
pub fn spawn_enemies_gradually(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
) {
    if enemy_spawn_timer.timer.finished() {
        println!("Spawning enemy from timer");
        let win = win_q.get_single().unwrap();
        let font = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
        let random_x = if random::<f32>() < 0.5 {
            random::<f32>() * win.width() / 2.
        } else {
            -random::<f32>() * win.width() / 2.
        };
        let random_y = if random::<f32>() < 0.5 {
            random::<f32>() * win.height() / 2.
        } else {
            -random::<f32>() * win.height() / 2.
        };

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

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_xyz(random_x, random_y, 1.),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                AnimateSprite,
                Enemy::default(),
            ))
            .with_children(|builder| {
                spawn_health_bar(builder);
                spawn_word(
                    builder,
                    word_bank.get_word(WordComplexity::Medium, &word_q).as_str(),
                    &font,
                );
            });
    }
}

/**
    Catch and handle EnemyShotEvent
    - Determine enemy event was for
    - Apply damage to enemy
    - If enemy is dead, despawn
    - Else, apply new word, and update health bar
**/
#[allow(clippy::too_many_arguments)] //TODO: reduce complexity?
pub fn catch_shot_event(
    mut commands: Commands,
    enemy_word_q: Query<(&Parent, Entity, &Word), With<Word>>,
    mut enemy_q: Query<(&mut Enemy, &Children)>,
    mut health_bar_q: Query<(&mut Sprite, &mut Transform), With<HealthBar>>,
    mut shot_event_reader: EventReader<EnemyShotEvent>,
    asset_server: Res<AssetServer>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
) {
    let font: Handle<Font> = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    for shot in shot_event_reader.iter() {
        for (parent, entity, word) in &mut enemy_word_q.iter() {
            if word.0 == WordTarget::Enemy(shot.0) {
                if let Ok(mut enemy) = enemy_q.get_mut(parent.get()) {
                    enemy.0.health -= 10;
                    if enemy.0.health == 0 {
                        commands.entity(parent.get()).despawn_recursive();
                    } else {
                        commands.entity(entity).despawn_recursive();
                        commands.entity(parent.get()).with_children(|builder| {
                            //TODO: update background box to change by word length? how to handle health bar then
                            spawn_word(
                                builder,
                                word_bank.get_word(WordComplexity::Medium, &word_q).as_str(),
                                &font.clone(),
                            );
                        });
                        //TODO: better way to get the child sprite/transform than this?
                        for &child in enemy.1 {
                            if let Ok(mut health_bar) = health_bar_q.get_mut(child) {
                                // 80 is the starting health bar size, multiplied against the health percentage
                                health_bar.0.custom_size =
                                    Some(Vec2::new(80. * (enemy.0.health as f32 / 100.), 2.));
                                // how much we need to offset x by to keep it left-justified, percentage of health lost multiplied by health bar size (80), divided by 2 as x is the center of the bar
                                health_bar.1.translation.x =
                                    -((100. - enemy.0.health as f32) / 100.) * 80. / 2.;
                            }
                        }
                    }
                } else {
                    println!("ERROR: no matching enemy found!");
                }
            }
        }
    }
}

pub fn enemy_movement(
    mut enemy_q: Query<(&mut Transform, &mut Handle<TextureAtlas>), With<Enemy>>,
    player_handles: Res<PlayerHandles>,
    time: Res<Time>,
) {
    if let Some((mut transform, mut atlas)) = enemy_q.iter_mut().next() {
        *atlas = player_handles.run.clone();
        transform.translation += Vec3::new(1., 1., 0.0).normalize() * 2. + time.delta_seconds();
    }
}
