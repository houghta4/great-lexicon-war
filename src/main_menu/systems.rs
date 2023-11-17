use bevy::prelude::{NextState, ResMut};
use crate::main_menu::MainMenuState;

pub fn enter_main_menu(mut next_menu_state: ResMut<NextState<MainMenuState>>) {
    next_menu_state.set(MainMenuState::Main);
}