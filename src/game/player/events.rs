use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerShotEvent(pub f32);

#[derive(Event)]
pub struct PlayerReloadEvent;

#[derive(Event)]
pub struct PlayerHealEvent;
