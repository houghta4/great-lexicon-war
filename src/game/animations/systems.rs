use bevy::prelude::*;
use crate::game::animations::components::{AnimateSprite, CharacterAnimations, Firing, MovableCharacter};
use crate::game::animations::events::CharacterMoveEvent;
use crate::game::level::components::MovePoint;
use crate::game::level::events::SpawnMovePointsEvent;
use crate::game::resources::CharacterHandles;

// Sometimes the edges are white. Possible issues: z-index fighting, need background
pub fn animate_sprite(
    time: Res<Time>,
    mut entity_q: Query<
        (
            &mut AnimateSprite,
            &mut TextureAtlasSprite,
        ),
        With<AnimateSprite>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(mut animation, mut sprite)| {
            animation.1.0.tick(time.delta());
            if animation.1.0.just_finished() {
                sprite.index = if sprite.index == animation.0.1 {
                    animation.0.0
                } else {
                    sprite.index + 1
                };
            }
        });
}

/**
    Catches character move events and sets character destinations, switches to walking animation
**/
#[allow(clippy::type_complexity)]
pub fn catch_character_move_event(
    mut commands: Commands,
    mut character_q: Query<(&mut MovableCharacter, &Transform, Entity), With<MovableCharacter>>,
    mut move_event_reader: EventReader<CharacterMoveEvent>,
    character_handles: Res<CharacterHandles>,
    cover_point_q: Query<(&MovePoint, &Transform), (With<MovePoint>, Without<MovableCharacter>)>) {

    for move_event in move_event_reader.iter() {
        for mut character in character_q.iter_mut() {
            if character.0.id == move_event.character_id {
                for (cover_point, transform) in cover_point_q.iter() {
                    if cover_point.id == move_event.target_id {
                        let mut target_translation = transform.translation.to_owned();
                        if target_translation.x > character.1.translation.x {
                            target_translation.x -= 40.;
                        } else {
                            target_translation.x += 40.;
                        }
                        target_translation.y += 50.;
                        character.0.move_target = Some((target_translation, cover_point.group_id));
                    }
                }

                commands.entity(character.2).insert((
                    SpriteSheetBundle {
                        texture_atlas: character_handles.soviet_walk.clone(),
                        transform: *character.1,
                        ..default()
                    },
                    CharacterAnimations::SovietWalk.get_animation()
                ));
            }
        }
    }
}

/**
Moves character when they have a destination, switches animation, and moves player
 **/
#[allow(clippy::type_complexity)]
pub fn move_character(
    mut commands: Commands,
    mut character_q: Query<(&mut MovableCharacter, &mut Transform, Entity), With<MovableCharacter>>,
    mut barrier_event_writer: EventWriter<SpawnMovePointsEvent>,
    character_handles: Res<CharacterHandles>,
    time: Res<Time>) {

    for mut character in character_q.iter_mut() {
        character.0.move_timer.tick(time.delta());
        if character.0.move_timer.just_finished() {
            if let Some(move_target) = character.0.move_target {
                let x_diff = character.1.translation.x - move_target.0.x;
                let y_diff = character.1.translation.y - move_target.0.y;
                if x_diff.powf(2.) + y_diff.powf(2.) <= 100. {
                    character.1.translation = move_target.0.to_owned();

                    commands.entity(character.2).insert((
                        SpriteSheetBundle {
                            texture_atlas: character_handles.soviet_idle.clone(),
                            transform: *character.1,
                            ..default()
                        },
                        CharacterAnimations::SovietIdle.get_animation()
                    ));
                    character.0.move_target = None;
                    barrier_event_writer.send(SpawnMovePointsEvent(move_target.1));
                } else if x_diff == 0. {
                    if y_diff > 0. {
                        character.1.translation.y -= 5.;
                    } else {
                        character.1.translation.y += 5.;
                    }
                } else if y_diff == 0. {
                    if x_diff > 0. {
                        character.1.translation.x -= 5.;
                    } else {
                        character.1.translation.x += 5.;
                    }
                } else {
                    let proportion = x_diff.abs() / y_diff.abs();
                    let mut y: f32 = (50. / (proportion.powf(2.) + 1.)).powf(0.5);
                    let mut x = y * proportion;
                    if x_diff > 0. {
                        x *= -1.;
                    }
                    if y_diff > 0. {
                        y *= -1.;
                    }
                    character.1.translation.x += x;
                    character.1.translation.y += y;
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
    mut firing_q: Query<(Entity, &mut Firing, &Transform), With<Firing>>,
    time: Res<Time>,
) {
    for (entity, mut firing, transform) in firing_q.iter_mut() {
        firing.timer.tick(time.delta());
        if firing.timer.just_finished() {
            commands.entity(entity).remove::<Firing>();
            // inserting this replaces the old one
            commands.entity(entity).insert((
                SpriteSheetBundle {
                    texture_atlas: firing.texture_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        flip_x: firing.flip_x,
                        ..default()
                    },
                    transform: *transform,
                    ..default()
                },
                firing.animation.get_animation(),
            ));
        }
    }
}
