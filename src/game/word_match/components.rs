use bevy::prelude::Component;

#[allow(dead_code)] //TODO: remove
#[derive(PartialEq, Debug)]
pub enum WordTarget {
    Reload,
    Heal,
    Enemy(u32), // bevy stores entity ids as u32
    Move(u32)
}

#[derive(Component, Debug)]
pub struct Word(pub WordTarget, pub String);
