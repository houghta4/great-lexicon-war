mod components;
mod layout;
mod systems;

use crate::game::player::events::{PlayerHealEvent, PlayerReloadEvent};
use crate::game::ui::hud::layout::*;
use crate::game::ui::hud::systems::*;
use crate::game::{InGameRunning, InGameState};
use crate::AppState;
use bevy::prelude::OnEnter;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_ui)
            .add_systems(
                OnEnter(InGameState::Running),
                (update_heal_word, update_reload_word),
            )
            .add_systems(
                Update,
                (
                    update_health,
                    update_ammo,
                    update_health_bar,
                    update_health_packs,
                    update_reload_word.run_if(on_event::<PlayerReloadEvent>()),
                    update_heal_word.run_if(on_event::<PlayerHealEvent>()),
                )
                    .in_set(InGameRunning),
            );
    }
}
