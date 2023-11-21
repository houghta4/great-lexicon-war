use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::main_menu::components::Faction;

#[derive(Resource, Default, Deserialize, Serialize)]
pub struct SaveInfo {
	pub level: u32,
	pub settings: Settings,
	pub german_status: FactionStatus,
	pub soviet_status: FactionStatus
}

impl SaveInfo {
	pub fn get_faction_status(&self, faction: Faction) -> &FactionStatus {
		match faction {
			Faction::German => {
				&self.german_status
			},
			Faction::Soviet => {
				&self.soviet_status
			}
		}
	}
}

#[derive(Default, Deserialize, Serialize)]
pub struct Settings {
	pub music: u32
}

#[derive(Deserialize, Serialize)]
pub struct FactionStatus {
	pub progress: u32,
	pub loadout: LoadoutStatus,
	pub unlocks: Vec<String>,
	pub levels: Vec<LevelStatus>
}

impl Default for FactionStatus {
	fn default() -> Self {
		FactionStatus {
			progress: 0,
			loadout: LoadoutStatus::default(),
			unlocks: vec![],
			levels: vec![
				LevelStatus {
					id: 1,
					points: 0,
					locked: false,
					objectives: vec![]
				},
				LevelStatus {
					id: 2,
					points: 0,
					locked: false,
					objectives: vec![]
				},
				LevelStatus {
					id: 3,
					points: 0,
					locked: false,
					objectives: vec![]
				},
				LevelStatus {
					id: 4,
					points: 0,
					locked: false,
					objectives: vec![]
				},
				LevelStatus {
					id: 5,
					points: 0,
					locked: false,
					objectives: vec![]
				},
				LevelStatus {
					id: 6,
					points: 0,
					locked: false,
					objectives: vec![]
				},
				LevelStatus {
					id: 7,
					points: 0,
					locked: false,
					objectives: vec![]
				}
			]
		}
	}
}

#[derive(Default, Deserialize, Serialize)]
pub struct LoadoutStatus {
	pub gun: String,
	pub special: String
}

#[derive(Default, Deserialize, Serialize)]
pub struct LevelStatus {
	pub id: u32,
	pub points: u32,
	pub locked: bool,
	pub objectives: Vec<String>
}