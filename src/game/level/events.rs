use bevy::prelude::Event;

#[derive(Event)]
pub struct LevelInitEvent;

#[derive(Event)]
pub struct LevelCompletedEvent;

#[derive(Event)]
pub struct SpawnMovePointsEvent(pub u32);

#[derive(Event)]
pub struct EnemyKilledEvent;

#[derive(Event)]
pub struct ProgressEvent;

#[derive(Event)]
pub struct TypoEvent;