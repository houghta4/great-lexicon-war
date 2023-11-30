use crate::game::word_match::components::{Word, WordTarget};
use bevy::prelude::*;
use rand::random;

/**
    Determine if a shot hit its target, based on distance, target being in cover, and gun accuracy
**/
pub fn determine_hit(distance: f32, in_cover: bool, gun_accuracy: f32) -> bool {
    let in_cover_multiplicand = if in_cover {
        1.
    } else {
        0.
    };
    let hit_chance = gun_accuracy * 100. / ((distance / 9.) + (50. * in_cover_multiplicand));
    println!("player hit chance: {} distance: {} actual distance: {} in cover: {}", hit_chance, distance, distance / 9., in_cover);
    random::<f32>() <= hit_chance
}

/**
    Spawn a given word for a given entity, with a black background for contrast
**/
pub fn spawn_word(builder: &mut ChildBuilder, word: &str, font: &Handle<Font>, word_target: WordTarget) {
    builder
        .spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::from_style(TextStyle {
                        font_size: 20.,
                        font: font.clone(),
                        color: Color::RED,
                    }),
                    TextSection::new(
                        word,
                        TextStyle {
                            font_size: 20.,
                            font: font.clone(),
                            color: Color::WHITE,
                        },
                    ),
                ]),
                transform: Transform::from_translation(Vec3::new(0., 60., 2.)),
                ..default()
            },
            Word(
                word_target,
                word.to_string(),
            ),
        ))
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    // keep 80 as minimum so health bar is always within the bounds
                    custom_size: Some(Vec2::new(f32::max(8. * word.len() as f32, 80.), 20.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                ..default()
            });
        });
}
