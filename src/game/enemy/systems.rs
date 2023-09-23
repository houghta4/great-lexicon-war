use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::animations::components::AnimateSprite;
use crate::game::enemy::events::EnemyShotEvent;
use crate::game::enemy::resources::EnemySpawns;
use crate::game::enemy::components::*;
use crate::game::input::components::InputText;
use crate::game::level::components::LevelInfo;
use crate::game::level::events::LevelInitEvent;
use crate::game::player::components::Player;
use crate::game::player::events::PlayerShotEvent;
use crate::game::resources::{CharacterHandles, RandomWord, WordBank};
use crate::game::utils::spawn_word;
use crate::game::word_match::components::{Word, WordTarget};
use crate::game::WordComplexity;
use crate::game::animations::components::CharacterAnimations;

use super::resources::EnemySpawnTimer;

// https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_sheet.rs

type EnemyBundle = (
    SpriteSheetBundle,
    AnimateSprite,
    Enemy,
);

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

/// All components a newly created Enemy will need
// TODO: factions?
fn get_enemy_bundle(x: f32, y: f32, character_handles: &Res<CharacterHandles>) -> EnemyBundle {

    (
        SpriteSheetBundle {
            texture_atlas: character_handles.german_idle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                flip_x: true,
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.),
            ..default()
        },
        CharacterAnimations::GermanIdle.get_animation(),
        Enemy::default()
    )
}

/// Spawn all enemies related to the level info
pub fn spawn_initial_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    enemy_spawn: Res<EnemySpawns>,
    character_handles: Res<CharacterHandles>
) {
    println!("Spawning initial enemies");
    let font = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    for pos in enemy_spawn.enemies.as_slice() {
        commands
            .spawn(get_enemy_bundle(pos.x, pos.y, &character_handles))
            .with_children(|builder| {
                spawn_health_bar(builder);
                spawn_word(
                    builder,
                    word_bank.get_word(WordComplexity::Medium, &word_q).as_str(),
                    &font,
                    WordTarget::Enemy(builder.parent_entity().index())
                );
            });
    }
}

/// Set resources once we progress to a new level
pub fn init_enemy_level_info(
    mut commands: Commands,
    level_info_q: Query<&LevelInfo>,
    mut level_init_event_reader: EventReader<LevelInitEvent>,
) {
    for level in level_init_event_reader.iter() {
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

/// Spawn enemies over time depending on the current level's `spawn_rate`
#[allow(clippy::too_many_arguments)]
pub fn spawn_enemies_gradually(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    character_handles: Res<CharacterHandles>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
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
        commands
            .spawn(get_enemy_bundle(random_x, random_y, &character_handles))
            .with_children(|builder| {
                spawn_health_bar(builder);
                spawn_word(
                    builder,
                    word_bank.get_word(WordComplexity::Medium, &word_q).as_str(),
                    &font,
                    WordTarget::Enemy(builder.parent_entity().index())
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
                                WordTarget::Enemy(builder.parent_entity().index())
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
#[allow(clippy::type_complexity)]
pub fn enemy_shoot_player(
    mut commands: Commands,
    mut enemy_q: Query<(Entity, &Transform), (With<Enemy>, Without<Firing>)>,
    character_handles: Res<CharacterHandles>,
    mut enemy_shot_player_event_writer: EventWriter<PlayerShotEvent>,
    player_q: Query<&Transform, With<Player>>,
    win_q: Query<&Window, With<PrimaryWindow>>,
) {
    let win = win_q.get_single().unwrap();
    if let Ok(player_transform) = player_q.get_single() {
        for (enemy_entity, enemy_transform) in enemy_q.iter_mut() {
            // If enemy is on screen, theres a chance to shoot player
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation)
                + 0.1;
            if distance < f32::max(win.height(), win.width()) {
                let shot_chance = distance * random::<f32>();
                if random::<f32>() > shot_chance {
                    println!("<< shot >>");
                    commands.entity(enemy_entity).insert((
                        SpriteSheetBundle {
                            texture_atlas: character_handles.german_fire.clone(),
                            sprite: TextureAtlasSprite {
                                flip_x: true,
                                ..default()
                            },
                            transform: *enemy_transform,
                            ..default()
                        },
                        CharacterAnimations::GermanFire.get_animation(),
                        Firing::default(),
                    ));
                    enemy_shot_player_event_writer.send(PlayerShotEvent);
                }
            }
        }
    }
}

/// Ticks enemy Firing timer until finished
///
/// When finished revert to base animation
#[allow(clippy::type_complexity)]
pub fn tick_and_replace_enemy_fire_timer(
    mut commands: Commands,
    mut firing_q: Query<(Entity, &mut Firing, &Transform), (With<Firing>, With<Enemy>)>,
    character_handles: Res<CharacterHandles>,
    time: Res<Time>,
) {
    //TODO: generify?
    for (enemy_entity, mut firing, transform) in firing_q.iter_mut() {
        firing.timer.tick(time.delta());
        if firing.timer.just_finished() {
            commands.entity(enemy_entity).remove::<Firing>();
            // inserting this replaces the old one
            commands.entity(enemy_entity).insert((
                SpriteSheetBundle {
                    texture_atlas: character_handles.german_idle.clone(),
                    sprite: TextureAtlasSprite {
                        flip_x: true,
                        ..default()
                    },
                    transform: *transform,
                    ..default()
                },
                CharacterAnimations::GermanIdle.get_animation(),
            ));
        }
    }
}
