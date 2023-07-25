use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera {}

// Used to animate a spritesheet from [first, last] instead of entire sheet
#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
