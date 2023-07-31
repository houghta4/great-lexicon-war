use bevy::prelude::*;
use crate::game::word_match::components::{Word, WordTarget};

/**
    Spawn a given word for a given entity, with a black background for contrast
**/
pub fn spawn_word(builder: &mut ChildBuilder, word: &str, font: &Handle<Font>) {
    builder.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::from_style(TextStyle {
                    font_size: 20.,
                    font: font.clone(),
                    color: Color::RED
                }),
                TextSection::new(
                    word,
                    TextStyle {
                        font_size: 20.,
                        font: font.clone(),
                        color: Color::WHITE
                    }
                )
            ]),
            transform: Transform::from_translation(Vec3::new(0., 15., 2.)),
            ..default()
        },
        Word(WordTarget::Enemy(1), word.to_string()))
    ).with_children(|builder| {
        builder.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(80.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                ..default()
            }
        );
    });
}