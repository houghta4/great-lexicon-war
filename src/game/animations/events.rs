use bevy::prelude::Event;

#[derive(Event)]
pub struct CharacterMoveEvent {
    pub character_id: u32,
    pub target_id: u32
}