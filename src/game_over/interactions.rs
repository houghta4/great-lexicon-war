use bevy::prelude::{BackgroundColor, Interaction, NextState, Query, ResMut};
use crate::AppState;
use crate::game::level::resources::Level;
use crate::game_over::components::GameOverButton;
use crate::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::utils::ChangedWith;

pub fn interact_with_game_over_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), ChangedWith<GameOverButton>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut level: ResMut<Level>
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                level.0 += 1;
                next_app_state.set(AppState::InGame);
                //level_init_writer.send(LevelInitEvent(level.0)); TODO: way to enter a specific level
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}