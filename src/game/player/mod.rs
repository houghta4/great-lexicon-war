use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use systems::*;

use crate::AppState;

use self::events::{PlayerHealEvent, PlayerReloadEvent, PlayerShotEvent};

use super::{enemy::events::EnemyShotEvent, InGameRunning};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<PlayerShotEvent>()
            .add_event::<PlayerReloadEvent>()
            .add_event::<PlayerHealEvent>()
            // On enter InGame state
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            // Systems
            .add_systems(
                Update,
                (
                    player_take_damage.run_if(on_event::<PlayerShotEvent>()),
                    player_shot_enemy.run_if(on_event::<EnemyShotEvent>()),
                    player_reload.run_if(on_event::<PlayerReloadEvent>()),
                    player_heal.run_if(on_event::<PlayerHealEvent>()),
                )
                    .in_set(InGameRunning),
            )
            // On exit InGame state
            .add_systems(OnExit(AppState::InGame), despawn_player);
    }
}
