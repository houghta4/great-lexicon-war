use bevy::prelude::*;

// How will we change data depending on the level?
#[derive(Resource)]
pub struct Level(usize);

impl Default for Level {
    fn default() -> Self {
        Level(1)
    }
}
