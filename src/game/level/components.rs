use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct LevelInfo {
    pub map: String,
    pub enemy_count: u32,
    pub spawn_rate: f32,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>,
}

impl Default for TiledMap {
    fn default() -> Self {
        TiledMap {
            width: 0,
            height: 0,
            tilewidth: 0,
            tileheight: 0,
            layers: Vec::new(),
        }
    }
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
