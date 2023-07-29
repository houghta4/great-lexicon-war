use bevy::prelude::Event;

#[derive(Event)]
pub struct LevelCompletedEvent(pub usize);
