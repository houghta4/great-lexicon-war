use bevy::prelude::{Component, Event};

#[derive(Component)]
pub struct Word(pub usize, pub String);

#[derive(Event)]
pub struct WordEvent(pub usize);