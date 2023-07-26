use bevy::prelude::{Resource, Timer};

//BackspaceTimer to slow backspaces down to a reasonable pace
#[derive(Resource)]
pub struct BackspaceTimer(pub Timer);