use bevy::prelude::Event;

#[derive(Event)]
pub struct LevelInitEvent(pub usize);

#[derive(Event)]
pub struct LevelCompletedEvent;

#[derive(Event)]
pub struct SpawnMovePointsEvent(pub u32);
