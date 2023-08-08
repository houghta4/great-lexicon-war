use bevy::prelude::Event;

#[derive(Event)]
pub struct EnemyShotEvent(pub u32);

#[derive(Event)]
pub struct EnemyShotPlayerEvent(pub u32); // Do not need param if we don't care which enemy shot
