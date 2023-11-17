use core::fmt;
use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct FactionMenu;

#[derive(Component)]
pub struct CampaignMenu;

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component)]
pub struct LoadoutMenu;

#[derive(Component)]
pub struct MenuButton(pub ButtonType);

pub enum ButtonType {
    Play,
    Settings,
    Faction(Faction),
    Loadout(Faction),
    LevelSelect(usize),
    ReturnToMain,
    Quit
}

#[derive(Debug, Copy, Clone)]
pub enum Faction {
    Soviet,
    German
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
