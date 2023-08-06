use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 3.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnCount {
    pub enemy_count: u32,
}

impl Default for EnemySpawnCount {
    fn default() -> Self {
        EnemySpawnCount { enemy_count: 1 }
    }
}
