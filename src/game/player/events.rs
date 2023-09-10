use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerShotEvent;

#[derive(Event)]
pub struct PlayerReloadEvent;

#[derive(Event)]
pub struct PlayerHealEvent;
