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

#[derive(Resource, Default)]
pub struct EnemySpawns {
    pub enemies: Vec<Vec2>,
}

#[derive(Resource, Default)]
pub struct EnemyHandles {
    pub soviet_idle: Handle<TextureAtlas>,
    pub soviet_walk: Handle<TextureAtlas>,
    pub soviet_fire: Handle<TextureAtlas>,
    pub german_walk: Handle<TextureAtlas>,
    pub german_fire: Handle<TextureAtlas>,
}
