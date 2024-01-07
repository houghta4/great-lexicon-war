use bevy::prelude::*;
use crate::main_menu::components::Faction;

#[derive(Resource)]
pub struct LevelInfo {
	id: usize,
	faction: Faction,
	pub kills: usize,
	pub progress: usize,
	pub typos: usize,
	pub completed: bool
}

impl Default for LevelInfo {
	fn default() -> Self {
		LevelInfo {
			id: 0,
			faction: Faction::Soviet,
			kills: 0,
			progress: 0,
			typos: 0,
			completed: false
		}
	}
}

impl LevelInfo {
	pub fn get_score(&self) -> usize {
		println!("progress: {}, kills: {}, typos: {}", self.progress, self.kills, self.typos);
		self.progress * 100 + self.kills * 50 - self.typos * 25
	}

	pub fn get_id(&self) -> usize {
		self.id
	}

	pub fn get_faction(&self) -> Faction {
		self.faction
	}

	pub fn set_level(&mut self, id: usize, faction: Faction) {
		self.id = id;
		self.faction = faction;
		self.kills = 0;
		self.progress = 0;
		self.typos = 0;
		self.completed = false;
	}
}