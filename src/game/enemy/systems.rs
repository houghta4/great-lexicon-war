use bevy::{prelude::*, window::PrimaryWindow};
use rand::seq::IteratorRandom;
use rand::{random, thread_rng, Rng};

use crate::game::animations::components::AnimateSprite;
use crate::game::enemy::events::EnemyShotEvent;
use crate::game::enemy::resources::EnemySpawns;
use crate::game::enemy::{components::*, EnemyAnimations};
use crate::game::input::components::InputText;
use crate::game::level::components::LevelInfo;
use crate::game::level::events::LevelCompletedEvent;
use crate::game::resources::{RandomWord, WordBank};
use crate::game::utils::spawn_word;
use crate::game::word_match::components::{Word, WordTarget};
use crate::game::{SpriteSheetInfo, WordComplexity};

use super::events::EnemyShotPlayerEvent;
use super::resources::{EnemyHandles, EnemySpawnTimer};

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

const SOVIET_IDLE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soviet_soldier/ppsh_idle.png",
    x: 128.0,
    y: 128.0,
    cols: 10,
    rows: 1,
};
const SOVIET_WALK: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soviet_soldier/ppsh_walking.png",
    x: 128.0,
    y: 128.0,
    cols: 8,
    rows: 1,
};
const SOVIET_FIRE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soviet_soldier/ppsh_firing.png",
    x: 128.0,
    y: 128.0,
    cols: 10,
    rows: 1,
};
const GERMAN_WALK: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/german_soldier/mp40_walking.png",
    x: 128.0,
    y: 128.0,
    cols: 8,
    rows: 1,
};
const GERMAN_FIRE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/german_soldier/mp40_firing.png",
    x: 128.0,
    y: 128.0,
    cols: 10,
    rows: 1,
};

fn get_texture_atlas_handle(
    cur_sprite: SpriteSheetInfo,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(cur_sprite.path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(cur_sprite.x, cur_sprite.y),
        cur_sprite.cols,
        cur_sprite.rows,
        None,
        None,
    );
    texture_atlases.add(texture_atlas)
}

/// `Res<EnemyHandles>` contains all enemy sprite sheet handles
///
/// When rendering an ememy, clone whichever handle is needed
pub fn init_texture_atlas_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handles = EnemyHandles {
        soviet_idle: get_texture_atlas_handle(SOVIET_IDLE, &asset_server, &mut texture_atlases),
        soviet_walk: get_texture_atlas_handle(SOVIET_WALK, &asset_server, &mut texture_atlases),
        soviet_fire: get_texture_atlas_handle(SOVIET_FIRE, &asset_server, &mut texture_atlases),
        german_walk: get_texture_atlas_handle(GERMAN_WALK, &asset_server, &mut texture_atlases),
        german_fire: get_texture_atlas_handle(GERMAN_FIRE, &asset_server, &mut texture_atlases),
    };

    commands.insert_resource(handles);
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
            transform: Transform::from_translation(Vec3::new(0., 70., 3.)),
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
    asset_server: Res<AssetServer>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    enemy_spawn: Res<EnemySpawns>,
    enemy_handles: Res<EnemyHandles>,
) {
    println!("Spawning initial enemies");
    let font = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    let mut rng = thread_rng();
    for pos in enemy_spawn.enemies.as_slice() {
        let animation_indices = EnemyAnimations::SovietIdle.get_indices();
        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: enemy_handles.soviet_idle.clone(),
                    sprite: TextureAtlasSprite::new(
                        rng.gen_range(animation_indices.0..animation_indices.1),
                    ),
                    transform: Transform::from_xyz(pos.x, pos.y, 1.),
                    ..default()
                },
                animation_indices,
                EnemyAnimations::SovietIdle.get_timer(),
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
    enemy_handles: Res<EnemyHandles>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
) {
    if enemy_spawn_timer.timer.finished() {
        println!("<< Spawning enemy from timer >>");
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
        let animation_indices = EnemyAnimations::SovietIdle.get_indices();
        let mut rng = thread_rng();
        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: enemy_handles.soviet_idle.clone(),
                    sprite: TextureAtlasSprite::new(
                        rng.gen_range(animation_indices.0..animation_indices.1),
                    ),
                    transform: Transform::from_xyz(random_x, random_y, 1.),
                    ..default()
                },
                animation_indices,
                EnemyAnimations::SovietIdle.get_timer(),
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

/// Enemy changes sprite sheet to shooting and triggers shot event
pub fn enemy_shoot_player(
    mut commands: Commands,
    mut enemy_q: Query<(Entity, &Transform), (With<Enemy>, Without<Firing>)>,
    enemy_handles: Res<EnemyHandles>,
    mut enemy_shot_player_event_writer: EventWriter<EnemyShotPlayerEvent>,
    keyboard_input: Res<Input<KeyCode>>, // Remove later
) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        println!("<< shoot >>");
        let mut rng = rand::thread_rng();
        if let Some((enemy_entity, transform)) = enemy_q.iter_mut().choose(&mut rng) {
            // inserting this replaces the old one
            commands.entity(enemy_entity).insert((
                SpriteSheetBundle {
                    texture_atlas: enemy_handles.soviet_fire.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: *transform,
                    ..default()
                },
                EnemyAnimations::SovietFire.get_indices(),
                EnemyAnimations::SovietFire.get_timer(),
                Firing::default(),
            ));
            enemy_shot_player_event_writer.send(EnemyShotPlayerEvent(enemy_entity.index()));
        }
    }
}

/// Ticks enemy Firing timer until finished
///
/// When finished revert to base animation
pub fn tick_and_replace_enemy_fire_timer(
    mut commands: Commands,
    mut firing_q: Query<(Entity, &mut Firing, &Transform), (With<Firing>, With<Enemy>)>,
    enemy_handles: Res<EnemyHandles>,
    time: Res<Time>,
) {
    for (enemy_entity, mut firing, transform) in firing_q.iter_mut() {
        if firing.timer.just_finished() {
            commands.entity(enemy_entity).remove::<Firing>();
            // inserting this replaces the old one
            commands.entity(enemy_entity).insert((
                SpriteSheetBundle {
                    texture_atlas: enemy_handles.soviet_idle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: *transform,
                    ..default()
                },
                EnemyAnimations::SovietIdle.get_indices(),
                EnemyAnimations::SovietIdle.get_timer(),
            ));
        } else {
            firing.timer.tick(time.delta());
        }
    }
}
