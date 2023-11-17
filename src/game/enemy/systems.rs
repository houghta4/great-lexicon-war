use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::animations::components::{AnimateSprite, CharacterAnimations, Firing, MovableCharacter};
use crate::game::enemy::components::*;
use crate::game::enemy::events::EnemyShotEvent;
use crate::game::enemy::resources::EnemySpawns;
use crate::game::input::components::InputText;
use crate::game::level::components::LevelInfo;
use crate::game::level::events::LevelInitEvent;
use crate::game::player::components::Player;
use crate::game::player::events::PlayerShotEvent;
use crate::game::resources::{CharacterHandles, RandomWord, WordBank};
use crate::game::utils::{determine_hit, spawn_word};
use crate::game::word_match::components::{Word, WordTarget};
use crate::game::WordComplexity;

use super::resources::EnemySpawnTimer;

// https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_sheet.rs

type EnemyBundle = (SpriteSheetBundle, AnimateSprite, Enemy);
const ENEMY_SPRITE_SIZE: f32 = 128.0 + 70.0; // size of enemy sprite + health bar

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
    character_handles: Res<CharacterHandles>,
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
                    WordTarget::Enemy(builder.parent_entity().index()),
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

// TODO: add later on after we finish poc 
#[allow(dead_code)]
/// Chance to spawn another enemy after the death of a fellow comrade
/// 
/// run_if(on_event::<EnemyDeathEvent>())
pub fn spawn_enemy_on_death(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    character_handles: Res<CharacterHandles>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    player_q: Query<&Transform, With<Player>>,
) {
    let chance_to_spawn = 0.15;
    // spawn new enemy
    println!("spawn_enemy_on_death");
    if random::<f32>() < chance_to_spawn {        
        let win = win_q.get_single().unwrap();
        let font: Handle<Font> = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");

        let player_y =  match player_q.get_single() {
            Ok(p) => p.translation.y,
            _ => 0.0
        };

        let random_x = (1.0 + random::<f32>()) * (win.width() / 2.0);
        let random_y = if random::<f32>() < 0.5 {
            player_y - ENEMY_SPRITE_SIZE + random::<f32>() * (win.height() / 2.0)
        } else {
            player_y + ENEMY_SPRITE_SIZE - random::<f32>() * (win.height() / 2.0)
        };
        let w = word_bank.get_word(WordComplexity::Medium, &word_q);
        commands
            .spawn(get_enemy_bundle(random_x, random_y, &character_handles))
            .with_children(|builder| {
                spawn_health_bar(builder);
                
                spawn_word(
                    builder,
                    &w,
                    &font,
                    WordTarget::Enemy(builder.parent_entity().index()),
                );
            });
        println!("spawning enemy: {} at (x: {}, y: {})!", w, random_x, random_y);
    }
}

/**
    Catch and handle EnemyShotEvent
    - Determine enemy event was for
    - Apply damage to enemy
    - If enemy is dead, despawn
    - Else, apply new word, and update health bar
**/
#[allow(clippy::too_many_arguments, clippy::type_complexity)] //TODO: reduce complexity?
pub fn catch_shot_event(
    mut commands: Commands,
    enemy_word_q: Query<(&Parent, Entity, &Word), With<Word>>,
    mut enemy_q: Query<(&mut Enemy, &Children, &Transform)>,
    mut health_bar_q: Query<(&mut Sprite, &mut Transform), (With<HealthBar>, Without<Enemy>, Without<Player>)>,
    mut shot_event_reader: EventReader<EnemyShotEvent>,
    asset_server: Res<AssetServer>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    player_q: Query<(&Transform, &MovableCharacter), With<Player>>
) {
    let font: Handle<Font> = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    for shot in shot_event_reader.iter() {
        for (parent, entity, word) in &mut enemy_word_q.iter() {
            if word.0 == WordTarget::Enemy(shot.0) {
                if let (Ok(mut enemy), Ok(player)) = (enemy_q.get_mut(parent.get()), player_q.get_single()) {
                    let distance = player.0.translation.distance(enemy.2.translation);
                    for _ in 0..5 {
                        if determine_hit(distance, player.1.move_target.is_none(), 0.3) {
                            if enemy.0.health >= 10 {
                                enemy.0.health -= 10;
                            } else {
                                enemy.0.health = 0;
                            }
                            if enemy.0.health == 0 {
                                commands.entity(parent.get()).despawn_recursive();
                            } else {
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
                        }
                    }
                    if enemy.0.health > 0 {
                        commands.entity(entity).despawn_recursive();
                        commands.entity(parent.get()).with_children(|builder| {
                            //TODO: update background box to change by word length? how to handle health bar then
                            spawn_word(
                                builder,
                                word_bank.get_word(WordComplexity::Medium, &word_q).as_str(),
                                &font.clone(),
                                WordTarget::Enemy(builder.parent_entity().index()),
                            );
                        });
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
    mut enemy_q: Query<(Entity, &Transform, &ComputedVisibility), (With<Enemy>, Without<Firing>)>,
    character_handles: Res<CharacterHandles>,
    mut enemy_shot_player_event_writer: EventWriter<PlayerShotEvent>,
    player_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        for (enemy_entity, enemy_transform, vis) in enemy_q.iter_mut() {
            // !
            //#region Enemy fire rate
            // If enemy is on screen, theres a chance to shoot player
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation)
                + 0.1;
            if vis.is_visible_in_view() && shot_chance(distance) {
                println!("<< shot from {} units away >>", distance);
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
                    Firing::create(character_handles.german_idle.clone(), CharacterAnimations::GermanIdle, true),
                ));
                enemy_shot_player_event_writer.send(PlayerShotEvent(distance));
            }
            //#endregion
        }
    }
}

/// Scale shot chance with distance from enemy to player
/// * `dist` is the Euclidean distance between the enemy and the player
/// ### Return true if enemy should fire else false
fn shot_chance(dist: f32) -> bool {
    let chance = 1.0 / dist; // bigger distance -> lower chance
    random::<f32>() <= chance
}

/*/// Ticks enemy Firing timer until finished
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
}*/

/// Test with --nocapture to see prints
#[cfg(test)]
mod shot_chance_tests {
    use super::shot_chance;

    const RANGE: u32 = 1000000;

    #[test]
    fn test_close() {
        let dist = 100.0; // 0.0390625
        let mut counter = 0;
        for _ in 0..RANGE {
            if shot_chance(dist) {
                counter += 1
            }
        }
        println!("Fired {} times from {} units away", counter, dist);
    }

    #[test]
    fn test_med() {
        let dist = 500.0;
        let mut counter = 0;
        for _ in 0..RANGE {
            if shot_chance(dist) {
                counter += 1
            }
        }
        println!("Fired {} times from {} units away", counter, dist);
    }

    #[test]
    fn test_far() {
        let dist = 1000.0;
        let mut counter = 0;
        for _ in 0..RANGE {
            if shot_chance(dist) {
                counter += 1
            }
        }
        println!("Fired {} times from {} units away", counter, dist);
    }
}
