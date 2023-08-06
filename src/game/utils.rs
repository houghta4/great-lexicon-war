use crate::game::word_match::components::{Word, WordTarget};
use bevy::prelude::*;

/**
    Spawn a given word for a given entity, with a black background for contrast
**/
pub fn spawn_word(builder: &mut ChildBuilder, word: &str, font: &Handle<Font>) {
    // TODO: find better way to go about this. Multiple fn will be calling this and having to take use WordBank and enemy word query each time seems redudant
    // Look into generating word here somehow
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
                transform: Transform::from_translation(Vec3::new(0., 15., 2.)),
                ..default()
            },
            Word(
                WordTarget::Enemy(builder.parent_entity().index()),
                word.to_string(),
            ),
        ))
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    // will need to scale this black bar to word size, keep 80 as minimum
                    custom_size: Some(Vec2::new(80.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                ..default()
            });
        });
}
