mod components;
mod systems;

use crate::game::enemy::events::EnemyShotEvent;
use crate::game::player::events::PlayerReloadEvent;
use crate::game::ui::hud::systems::*;
use crate::game::InGameRunning;
use crate::AppState;
use bevy::prelude::OnEnter;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_ui)
            .add_systems(
                Update,
                (
                    update_health,
                    update_ammo,
                    update_reload_word.run_if(on_event::<EnemyShotEvent>()),
                    remove_reload_word.run_if(on_event::<PlayerReloadEvent>()),
                )
                    .in_set(InGameRunning),
            );
    }
}
