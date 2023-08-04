use bevy::prelude::Component;

#[allow(dead_code)] //TODO: remove
#[derive(PartialEq)]
pub enum WordTarget {
    Reload,
    Heal,
    Enemy(usize),
    Move(usize)
}

#[derive(Component)]
pub struct Word(pub WordTarget, pub String);
