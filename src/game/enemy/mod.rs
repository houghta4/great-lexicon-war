use bevy::prelude::*;

mod components;
pub mod events;
mod resources;
mod systems;

use systems::*;

use crate::components::AnimationTimer;
use crate::AppState;
use crate::{components::AnimationIndices, game::enemy::events::EnemyShotEvent};

use self::events::EnemyShotPlayerEvent;
use self::resources::{EnemySpawnTimer, EnemySpawns};
use super::InGameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EnemyShotEvent>()
            .add_event::<EnemyShotPlayerEvent>()
            // Startup system
            .add_systems(Startup, init_texture_atlas_handles)
            // Systems
            .add_systems(Update, init_enemy_level_info)
            .add_systems(
                Update,
                (despawn_enemies, spawn_initial_enemies)
                    .chain()
                    .run_if(resource_exists_and_changed::<EnemySpawns>()),
            )
            .add_systems(
                Update,
                (
                    catch_shot_event,
                    enemy_shoot_player,
                    tick_and_replace_enemy_fire_timer,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            .add_systems(
                Update,
                (tick_enemy_spawn_timer, spawn_enemies_gradually)
                    .run_if(resource_exists::<EnemySpawnTimer>())
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            )
            // On Exit systems
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}

#[allow(dead_code)]
pub enum EnemyAnimations {
    SovietIdle,
    SovietWalk,
    SovietFire,
    GermanWalk,
    GermanFire,
}

impl EnemyAnimations {
    fn get_indices(&self) -> AnimationIndices {
        match *self {
            Self::SovietIdle => AnimationIndices(0, 9),
            Self::SovietWalk => AnimationIndices(0, 7),
            Self::SovietFire => AnimationIndices(0, 9),
            Self::GermanWalk => AnimationIndices(0, 7),
            Self::GermanFire => AnimationIndices(0, 7),
        }
    }
    fn get_timer(&self) -> AnimationTimer {
        match *self {
            Self::SovietFire => AnimationTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            Self::GermanFire => AnimationTimer(Timer::from_seconds(0.035, TimerMode::Repeating)),
            _ => AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}
