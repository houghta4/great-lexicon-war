use bevy::prelude::*;

use crate::game::{
    ui::pause_menu::{components::ResumeButton, styles::*},
    InGameState,
};

pub fn interact_with_resume_button(
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >, // to change background color. not sure if this is the best way
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
