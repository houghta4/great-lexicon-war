use bevy::prelude::Event;

#[derive(Event)]
pub struct LevelCompletedEvent(pub usize);

#[derive(Event)]
pub struct SpawnBarriersEvent(pub u32);
