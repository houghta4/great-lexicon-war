use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera {}

// Used to animate a spritesheet from [first, last] instead of entire sheet
#[derive(Component)]
pub struct AnimationIndices(pub usize, pub usize);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
