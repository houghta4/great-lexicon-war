use bevy::prelude::Event;

#[derive(Event)]
pub struct EnemyShotEvent(pub u32);
