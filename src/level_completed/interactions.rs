use bevy::prelude::{BackgroundColor, Interaction, NextState, Query, Res, ResMut};
use crate::AppState;
use crate::game::level::resources::LevelInfo;
use crate::level_completed::components::LevelCompletedButton;
use crate::main_menu::resources::SaveInfo;
use crate::styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::utils::ChangedWith;

pub fn interact_with_level_complete_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), ChangedWith<LevelCompletedButton>>,
    level_info: Res<LevelInfo>,
    mut save_info: ResMut<SaveInfo>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                println!("setting {} to {}", level_info.get_id(), level_info.get_score());
                save_info.post_level_result(level_info.get_faction(), level_info.get_id(), level_info.get_score(), vec![]);
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