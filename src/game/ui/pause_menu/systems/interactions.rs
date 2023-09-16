use bevy::prelude::*;

use crate::{
    game::{
        ui::pause_menu::components::{QuitButton, ResumeButton},
        InGameState,
    },
    AppState,
};
use crate::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::utils::ChangedWith;

pub fn interact_with_resume_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), ChangedWith<ResumeButton>>,
    mut next_game_state: ResMut<NextState<InGameState>>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_game_state.set(InGameState::Running);
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

pub fn interact_with_quit_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), ChangedWith<QuitButton>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_app_state.set(AppState::MainMenu);
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
