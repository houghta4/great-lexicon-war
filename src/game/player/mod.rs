use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use systems::*;

use crate::AppState;

use self::events::PlayerShotEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<PlayerShotEvent>()
            // On enter InGame state
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            // On exit InGame state
            .add_systems(OnExit(AppState::InGame), despawn_player);
            //.add_systems(Update, move_player.run_if(in_state(AppState::InGame)));
    }
}
