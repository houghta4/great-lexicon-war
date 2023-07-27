use bevy::prelude::*;

#[derive(Resource)]
pub struct Level(pub usize);

impl Default for Level {
    fn default() -> Self {
        Level(0)
    }
}
