use bevy::prelude::*;

use super::components::*;
use crate::components::{AnimationIndices, AnimationTimer};

// Sometimes the edges are white. Possible issues: z-index fighting, need background
pub fn animate_sprite(
    time: Res<Time>,
    mut entity_q: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<AnimateSprite>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(indices, mut timer, mut sprite)| {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        });
}
