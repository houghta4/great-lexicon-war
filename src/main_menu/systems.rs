use bevy::prelude::*;
use crate::main_menu::MainMenuState;
use crate::main_menu::resources::SaveInfo;

pub fn enter_main_menu(mut next_menu_state: ResMut<NextState<MainMenuState>>) {
    next_menu_state.set(MainMenuState::Main);
}

pub fn load_save() -> SaveInfo {
    let app_data = std::env::var("LOCALAPPDATA").expect("No LOCALAPPDATA directory");
    std::fs::create_dir_all(format!("{}/GreatLexiconWar", app_data)).expect("Error creating save directory");
    match std::fs::read_to_string(format!("{}/GreatLexiconWar/save.json", app_data)) {
        Ok(json) => {
            serde_json::from_str(&json).expect("Unable to convert save json to object")
        },
        Err(_) => {
            SaveInfo::default()
        }
    }
}

pub fn save_save(save_info: Res<SaveInfo>) {
    let json = serde_json::to_string_pretty(save_info.into_inner()).expect("Unable to convert save info to json");
    let app_data = std::env::var("LOCALAPPDATA").expect("No LOCALAPPDATA directory");
    std::fs::write(format!("{}/GreatLexiconWar/save.json", app_data), json).expect("Unable to write save file");
}