use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct LevelInfo {
    pub map: String,
    pub spawn_rate: f32,
    pub enemies: Vec<Vec2>,
}

#[derive(Default, Component, Debug, Deserialize, Serialize)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct TiledLayer {
    pub name: String,
    pub data: Vec<u32>,
}

impl Default for TiledLayer {
    fn default() -> Self {
        TiledLayer {
            name: "default".into(),
            data: vec![],
        }
    }
}

#[derive(Component)]
pub struct RenderedTile;
