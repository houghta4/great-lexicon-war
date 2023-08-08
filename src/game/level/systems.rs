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
        spawn_rate: 10.0,
        enemies: vec![Vec2::new(0., 0.), Vec2::new(0., 128.), Vec2::new(0., 256.)],
    });

    // Lv 2
    commands.spawn(LevelInfo {
        map: "assets/maps/level_02.json".to_string(),
        spawn_rate: 3.0,
        enemies: vec![Vec2::new(0., 0.)],
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
) {
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
        }
    }
}

pub fn clear_map(mut commands: Commands, rendered_map_q: Query<Entity, With<RenderedTile>>) {
    rendered_map_q.iter().for_each(|map| {
        commands.entity(map).despawn();
    });
}
