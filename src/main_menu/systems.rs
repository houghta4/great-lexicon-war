use bevy::prelude::*;
use crate::main_menu::MainMenuState;
use crate::main_menu::resources::SaveInfo;

pub fn enter_main_menu(mut next_menu_state: ResMut<NextState<MainMenuState>>) {
    next_menu_state.set(MainMenuState::Main);
}

pub fn load_save() -> SaveInfo {
    let app_data = std::env::var("LOCALAPPDATA").expect("No LOCALAPPDATA directory");
    std::fs::create_dir_all(format!("{}/GreatLexiconWar", app_data)).expect("Error creating save directory");
    match std::fs::read_to_string(format!("{}/GreatLexiconWar/save.toml", app_data)) {
        Ok(json) => {
            toml::from_str(&json).expect("Unable to convert save toml to object")
        },
        Err(_) => {
            SaveInfo::default()
        }
    }
}

pub fn save_save(save_info: Res<SaveInfo>) {
    let toml = toml::to_string_pretty(save_info.into_inner()).expect("Unable to convert save info to toml");
    let app_data = std::env::var("LOCALAPPDATA").expect("No LOCALAPPDATA directory");
    std::fs::write(format!("{}/GreatLexiconWar/save.toml", app_data), toml).expect("Unable to write save file");
}