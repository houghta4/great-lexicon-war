use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct LevelInfo {
    pub map: String,
    pub spawn_rate: f32,
    pub enemies: Vec<Vec2>
}

#[derive(Component, Serialize, Deserialize)]
pub struct Tileset {
    pub size: f32,
    pub columns: usize,
    pub rows: usize
}

#[derive(Component, Serialize, Deserialize)]
pub struct MapObject {
    pub class: MapObjectClass,
    pub id: usize,
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize)]
pub enum MapObjectClass {
    Barrier(u32),
    Objective(u32),
    Tree,
    Stone
}

#[derive(Default, Component, Deserialize, Serialize)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>,
    pub objects: Vec<MapObject>
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct TiledLayer {
    pub name: String,
    pub data: Vec<u32>,
}

#[derive(Component)]
pub struct MovePoint {
    pub group_id: u32,
    pub id: u32
}

//Added purely because Vec2 is not deserializable
#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32
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
