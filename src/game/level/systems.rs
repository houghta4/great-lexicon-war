use bevy::prelude::*;
use crate::game::input::components::InputText;
use crate::game::level::components::{BarrierPoint, MapObjectClass, Tileset};
use crate::game::level::events::SpawnBarriersEvent;
use crate::game::resources::{RandomWord, WordBank};
use crate::game::utils::spawn_word;
use crate::game::word_match::components::{Word, WordTarget};
use crate::game::WordComplexity;

use super::{
    components::{LevelInfo, RenderedTile, TiledMap},
    events::LevelCompletedEvent,
    resources::Level,
};

pub fn setup_levels(mut commands: Commands) {
    // Lv 1
    commands.spawn(LevelInfo {
        map: "assets/maps/level_01.json".to_string(),
        spawn_rate: 10.0,
        enemies: vec![Vec2::new(0., -128.), Vec2::new(0., 128.), Vec2::new(0., 256.), Vec2::new(0., 600.), Vec2::new(700., 256.), Vec2::new(750., 300.)]
    });

    // Lv 2
    /*commands.spawn(LevelInfo {
        map: "assets/maps/level_02.json".to_string(),
        spawn_rate: 3.0,
        enemies: vec![Vec2::new(0., 0.)]
    });*/
    //TODO: make more levels
}

// Send event from current level, then increment
pub fn level_complete_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut level: ResMut<Level>, // may not work?
    mut level_complete_event: EventWriter<LevelCompletedEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Delete) {
        level.0 += 1;
        println!("Creating LevelCompletedEvent for Level({})", level.0);
        level_complete_event.send(LevelCompletedEvent(level.0));
    }
}

pub fn init_level(mut level_complete_event: EventWriter<LevelCompletedEvent>) {
    level_complete_event.send(LevelCompletedEvent(0));
}

fn parse_tiled_map(map_path: &str) -> Result<TiledMap, Box<dyn std::error::Error>> {
    let map_json = std::fs::read_to_string(map_path)?;
    serde_json::from_str(&map_json).map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
}

fn parse_tileset(tileset_path: &str) -> Result<Tileset, Box<dyn std::error::Error>> {
    let tileset_json = std::fs::read_to_string(tileset_path)?;
    serde_json::from_str(&tileset_json).map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
}

#[allow(clippy::too_many_arguments)]
pub fn render_level_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_complete_event_reader: EventReader<LevelCompletedEvent>,
    level_info_q: Query<&LevelInfo>,
    rendered_map_q: Query<Entity, With<RenderedTile>>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>
) {
    let font: Handle<Font> = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    for level in level_complete_event_reader.iter() {
        // clear out old map
        rendered_map_q.iter().for_each(|map| {
            commands.entity(map).despawn();
        });

        if let Some(level_info) = level_info_q.iter().nth(level.0) {
            let map_data = parse_tiled_map(&level_info.map).unwrap_or_else(|_| TiledMap::default());

            // These are atrocious. Fix later when we use (0, 0)
            let start_x = -400.;
            let start_y = 0.;

            let tile_size = Vec2::new(map_data.tilewidth as f32, map_data.tileheight as f32);
            let mut bundles: Vec<(SpriteSheetBundle, RenderedTile)> = vec![];

            // Sprite sheet
            let texture_handle: Handle<Image> =
                asset_server.load("sprites/terrain/spring_tileset.png");
            if let Ok(tileset_data) = parse_tileset("assets/sprites/terrain/spring_tileset.json") {
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle, Vec2::new(tileset_data.size, tileset_data.size), tileset_data.columns, tileset_data.rows, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                println!("starting to spawn layers");
                // Loop over each layer
                for (layer_count, layer) in map_data.layers.iter().enumerate() {
                    let data = &layer.data;
                    // For each layer we loop over the data
                    for y in 0..map_data.height {
                        for x in 0..map_data.width {
                            let idx = (y * map_data.width + x) as usize;
                            let position = Vec3::new(
                                start_x + x as f32 * tile_size.x,
                                start_y + (map_data.height - y - 1) as f32 * tile_size.y,
                                layer_count as f32,
                            );

                            let tile = (
                                SpriteSheetBundle {
                                    texture_atlas: texture_atlas_handle.clone(),
                                    sprite: TextureAtlasSprite::new(data[idx] as usize),
                                    transform: Transform::from_translation(position),
                                       // .with_scale(tile_scale),

                                    ..default()
                                },
                                RenderedTile,
                            );
                            println!("spawn tile");
                            bundles.push(tile);
                        }
                    }
                }

                let tree_texture_handle: Handle<Image> = asset_server.load("sprites/objects/trees.png");
                let stone_texture_handle: Handle<Image> = asset_server.load("sprites/objects/stones.png");
                let barrier_texture_handle: Handle<Image> = asset_server.load("sprites/objects/barriers.png");

                if let (Ok(tree_data), Ok(stone_data), Ok(barrier_data)) = (parse_tileset("assets/sprites/objects/trees.json"), parse_tileset("assets/sprites/objects/stones.json"), parse_tileset("assets/sprites/objects/barriers.json")) {
                    let tree_texture_atlas =
                        TextureAtlas::from_grid(tree_texture_handle, Vec2::new(tree_data.size, tree_data.size), tree_data.columns, tree_data.rows, None, None);
                    let tree_atlas_handle = texture_atlases.add(tree_texture_atlas);

                    let stone_texture_atlas =
                        TextureAtlas::from_grid(stone_texture_handle, Vec2::new(stone_data.size, stone_data.size), stone_data.columns, stone_data.rows, None, None);
                    let stone_atlas_handle = texture_atlases.add(stone_texture_atlas);

                    let barrier_texture_atlas =
                        TextureAtlas::from_grid(barrier_texture_handle, Vec2::new(barrier_data.size, barrier_data.size), barrier_data.columns, barrier_data.rows, None, None);
                    let barrier_atlas_handle = texture_atlases.add(barrier_texture_atlas);
                    let mut barrier_id = 0;
                    for object in map_data.objects {
                        let object_atlas_handle = match object.class {
                            MapObjectClass::Barrier(_) => &barrier_atlas_handle,
                            MapObjectClass::Tree => &tree_atlas_handle,
                            MapObjectClass::Stone => &stone_atlas_handle
                        };

                        match object.class {
                            MapObjectClass::Barrier(group_id) => {
                                let mut entity_commands = commands.spawn((
                                    SpriteSheetBundle {
                                        texture_atlas: object_atlas_handle.clone(),
                                        sprite: TextureAtlasSprite::new(object.id),
                                        transform: Transform::from_translation(Vec3::new(object.x, object.y, 4.)),
                                        ..default()
                                    },
                                    RenderedTile, //TODO: we ok with this here?
                                    BarrierPoint {
                                        group_id,
                                        id: barrier_id
                                    }
                                ));
                                if group_id == 0 {
                                    entity_commands.with_children(|builder| {
                                        spawn_word(builder, word_bank.get_word(WordComplexity::Easy, &word_q).as_str(), &font, WordTarget::Move(barrier_id));
                                    });
                                }
                                barrier_id += 1;
                            },
                            _ => {
                                //TODO: this could look awkward sometimes when the players feet are below the bottom of the object, adjust
                                bundles.push((
                                    SpriteSheetBundle {
                                        texture_atlas: object_atlas_handle.clone(),
                                        sprite: TextureAtlasSprite::new(object.id),
                                        transform: Transform::from_translation(Vec3::new(object.x, object.y, 4.)),
                                        ..default()
                                    },
                                    RenderedTile //TODO: we ok with this here?
                                ));
                            }
                        }
                    }
                }
            }

            commands.spawn_batch(bundles);
        }
    }
}

pub fn clear_map(mut commands: Commands, rendered_map_q: Query<Entity, With<RenderedTile>>) {
    rendered_map_q.iter().for_each(|map| {
        commands.entity(map).despawn();
    });
}

pub fn catch_spawn_barriers_event(
    mut commands: Commands,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
    asset_server: Res<AssetServer>,
    mut barrier_spawn_reader: EventReader<SpawnBarriersEvent>,
    mut barrier_q: Query<(Entity, &BarrierPoint, Option<&Children>), With<BarrierPoint>>) {

    let font = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");

    for barrier_event in barrier_spawn_reader.iter() {
        for (entity, barrier_point, children_opt) in barrier_q.iter_mut() {
            if barrier_point.group_id == barrier_event.0 {
                if let Some(children) = children_opt {
                    if let Some(child) = children.get(0) {
                        commands.entity(*child).despawn_recursive();
                    }
                }
            } else if barrier_point.group_id == barrier_event.0 + 1 {
                commands.entity(entity).with_children(|builder| {
                    spawn_word(builder, word_bank.get_word(WordComplexity::Easy, &word_q).as_str(), &font, WordTarget::Move(barrier_point.id));
                });
            }
        }
    }
}
