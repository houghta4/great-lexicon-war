use bevy::{prelude::*, window::PrimaryWindow};
use crate::game::input::components::InputText;
use crate::game::level::components::BarrierPoint;
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
    commands.spawn(LevelInfo {
        map: "assets/maps/level_02.json".to_string(),
        spawn_rate: 3.0,
        enemies: vec![Vec2::new(0., 0.)]
    });
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

pub fn render_level_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_complete_event_reader: EventReader<LevelCompletedEvent>,
    level_info_q: Query<&LevelInfo>,
    rendered_map_q: Query<Entity, With<RenderedTile>>,
    win_q: Query<&Window, With<PrimaryWindow>>,
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>
) {
    let font = asset_server.load("fonts/fyodor/truetype/Fyodor-BoldCondensed.ttf");
    for level in level_complete_event_reader.iter() {
        // clear out old map
        rendered_map_q.iter().for_each(|map| {
            commands.entity(map).despawn();
        });

        if let Some(level_info) = level_info_q.iter().nth(level.0) {
            let win = win_q.get_single().unwrap();
            let map_data = parse_tiled_map(&level_info.map).unwrap_or_else(|_| TiledMap::default());

            // These are atrocious. Fix later when we use (0, 0)
            let start_x = win.width() / 2.0 - ((map_data.tilewidth * map_data.width) as f32 / 2.0);
            let start_y =
                win.height() / 2.0 - ((map_data.tileheight * map_data.height) as f32 / 2.0);

            // 960x960 with 32x32 tiles. 30 rows, 30 cols
            let tile_size = Vec2::new(map_data.tilewidth as f32, map_data.tileheight as f32);
            let tile_scale = Vec3::new(tile_size.x / 32.0, tile_size.y / 32.0, 0.0);
            let mut bundles: Vec<(SpriteSheetBundle, RenderedTile)> = vec![];

            // Sprite sheet
            let texture_handle: Handle<Image> =
                asset_server.load("tilesets/hyptosis_tile-art-batch-1.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 30, 30, None, None);

            let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                                transform: Transform::from_translation(position)
                                    .with_scale(tile_scale),

                                ..default()
                            },
                            RenderedTile,
                        );
                        bundles.push(tile);
                    }
                }
            }

            commands.spawn_batch(bundles);

            let mut barrier_id: u32 = 0;
            for group_id in 0..map_data.barriers.len() {
                println!("spawning group {}", group_id);
                let barrier_set = &map_data.barriers[group_id];
                for barrier in barrier_set {
                    let mut entity_commands = commands.spawn((SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK,
                            custom_size: Some(Vec2::new(20., 100.)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(barrier.x, barrier.y, 1.)),
                        ..default()
                    }, BarrierPoint {
                        group_id: group_id as u32,
                        id: barrier_id
                    }
                    ));
                    if group_id == 0 {
                        entity_commands.with_children(|builder| {
                            spawn_word(builder, word_bank.get_word(WordComplexity::Easy, &word_q).as_str(), &font, WordTarget::Move(barrier_id));
                        });
                    }
                    barrier_id += 1;
                }
            }
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
