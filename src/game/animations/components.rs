use bevy::prelude::*;
use crate::components::{AnimationIndices, AnimationTimer};

// All animated sprites must use this
#[derive(Component)]
pub struct AnimateSprite(pub AnimationIndices, pub AnimationTimer);

#[derive(Component)]
pub struct MovableCharacter {
    pub id: u32,
    pub move_target: Option<(Vec3, u32)>,
    pub move_timer: Timer
}

#[allow(dead_code)]
pub enum CharacterAnimations {
    SovietIdle,
    SovietWalk,
    SovietFire,
    GermanWalk,
    GermanFire,
    GermanIdle
}

impl CharacterAnimations {
    pub fn get_animation(&self) -> AnimateSprite {
        let indicies = match *self {
            Self::SovietIdle => AnimationIndices(0, 9),
            Self::SovietWalk => AnimationIndices(0, 7),
            Self::SovietFire => AnimationIndices(0, 9),
            Self::GermanWalk => AnimationIndices(0, 7),
            Self::GermanFire => AnimationIndices(0, 7),
            Self::GermanIdle => AnimationIndices(0, 9)
        };
        let timer = match *self {
            Self::SovietFire => AnimationTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            Self::GermanFire => AnimationTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            _ => AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        };
        AnimateSprite(indicies, timer)
    }
}
