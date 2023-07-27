use bevy::{prelude::*, window::PrimaryWindow};

use super::{
    components::{LevelInfo, RenderedTile, TiledMap},
    events::LevelCompletedEvent,
    resources::Level,
};

pub fn setup_levels(mut commands: Commands) {
    // Lv 1
    commands.spawn(LevelInfo {
        map: "assets/maps/level_01.json".to_string(),
        enemy_count: 10,
        spawn_rate: 10.0,
    });

    // Lv 2
    commands.spawn(LevelInfo {
        map: "assets/maps/level_02.json".to_string(),
        enemy_count: 10,
        spawn_rate: 10.0,
    });
}

// Send event from current level, then increment
pub fn level_complete_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut level: ResMut<Level>, // may not work?
    mut level_complete_event: EventWriter<LevelCompletedEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Delete) {
        println!("Sending level info event. Level({})", level.0);
        level_complete_event.send(LevelCompletedEvent);
        level.0 += 1;
    }
}

pub fn init_level(mut level_complete_event: EventWriter<LevelCompletedEvent>) {
    level_complete_event.send(LevelCompletedEvent);
}

fn parse_tiled_map(map_path: &str) -> Result<TiledMap, Box<dyn std::error::Error>> {
    let map_json = std::fs::read_to_string(map_path)?;
    serde_json::from_str(&map_json).map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
}

pub fn render_level_data(
    mut commands: Commands,
    // These will be needed once we have our sprite sheet
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_complete_event_reader: EventReader<LevelCompletedEvent>,
    level: Res<Level>,
    level_info_q: Query<&LevelInfo>,
    rendered_map_q: Query<Entity, With<RenderedTile>>,
    win_q: Query<&Window, With<PrimaryWindow>>,
) {
    for _ in level_complete_event_reader.iter() {
        // clear out old map
        rendered_map_q.iter().for_each(|map| {
            println!("Removing a map");
            commands.entity(map).despawn();
        });

        if let Some(level_info) = level_info_q.iter().nth(level.0) {
            println!("rendering level data");
            let win = win_q.get_single().unwrap();
            let start_x = win.width() / 2.0;
            let start_y = win.height() / 2.0;
            let map_data = parse_tiled_map(&level_info.map).unwrap_or_else(|_| TiledMap::default());

            let tile_size = Vec2::new(map_data.tilewidth as f32, map_data.tileheight as f32);
            let mut bundles: Vec<(SpriteBundle, RenderedTile)> = vec![];
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
                        println!("Position: {position}");
                        let sprite_color = match data[idx] {
                            0 => Color::WHITE,  // Empty tile
                            1 => Color::GREEN,  // Tile with ID 1
                            _ => Color::ORANGE, // Default color for unknown tiles
                        };

                        // will need to use atlas once we have a sprite sheet
                        let tile = (
                            SpriteBundle {
                                transform: Transform::from_translation(position),
                                sprite: Sprite {
                                    color: sprite_color,
                                    custom_size: Some(tile_size),
                                    ..default()
                                },
                                ..default()
                            },
                            RenderedTile,
                        );
                        bundles.push(tile);
                    }
                }
            }

            commands.spawn_batch(bundles);
        }
    }
}
