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

#[derive(Component)]
pub struct Firing {
    pub timer: Timer,
}

impl Default for Firing {
    fn default() -> Self {
        Firing {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}
