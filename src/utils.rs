use bevy::prelude::{Changed, Interaction, With};

pub type ChangedWith<T> = (Changed<Interaction>, With<T>);