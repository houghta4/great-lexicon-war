use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: usize, // Percentage [0, 100]
    pub direction: Vec2,
}

#[derive(Component)]
pub struct HealthBar;

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            health: 100,
            direction: Vec2::default(),
        }
    }
}