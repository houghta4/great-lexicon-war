use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub ammo: (usize, usize), // bullets left, mag size
    pub health: f32,          // [0, 100] %
    pub health_packs: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            ammo: (30, 30),
            health: 100.0,
            health_packs: 3,
        }
    }
}
