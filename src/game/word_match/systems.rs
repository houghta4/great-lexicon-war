use crate::game::animations::events::CharacterMoveEvent;
use crate::game::enemy::events::EnemyShotEvent;
use crate::game::input::components::InputText;
use crate::game::player::components::Player;
use crate::game::player::events::{PlayerHealEvent, PlayerReloadEvent};
use crate::game::word_match::components::{Word, WordTarget};
use bevy::prelude::*;
use std::cmp;

//TODO: rewrite to be more efficient
/**
    Checks for matches between Words and user input
**/
#[allow(clippy::type_complexity)]
pub fn check_matches(
    mut input_text: Query<&mut Text, With<InputText>>,
    mut words: Query<(&mut Text, &Word), (With<Word>, Without<InputText>)>,
    mut enemy_event_writer: EventWriter<EnemyShotEvent>,
    mut reload_event_writer: EventWriter<PlayerReloadEvent>,
    mut heal_event_writer: EventWriter<PlayerHealEvent>,
    player_q: Query<&Player>,
    mut move_event_writer: EventWriter<CharacterMoveEvent>
) {
    let input_str = input_text.single_mut().sections[0].value.to_string();
    for (mut text, word) in &mut words {
        if input_str.is_empty() {
            if !text.sections[0].value.is_empty() {
                text.sections[0].value = "".to_string();
                text.sections[1].value = word.1.to_string();
            }
        } else if !text.sections[1].value.is_empty() {
            let mut completed = "".to_string();
            let mut remaining = "".to_string();

            let mut input_chars = input_str.chars();
            let mut target_chars = word.1.chars();

            for _n in 0..cmp::min(word.1.len(), input_str.len()) {
                if let (Some(target_char), Some(input_char)) =
                    (target_chars.next(), input_chars.next())
                {
                    if let (Some(target_lower), Some(input_lower)) = (
                        target_char.to_lowercase().next(),
                        input_char.to_lowercase().next(),
                    ) {
                        if target_lower == input_lower {
                            completed.push(target_char);
                        } else {
                            completed = "".to_string();
                            remaining = word.1.to_string();
                            break;
                        }
                    }
                }
            }

            if remaining.is_empty() && word.1.len() > input_str.len() {
                remaining = word.1.to_string()[input_str.len()..].to_string();
            }

            text.sections[0].value = completed;
            text.sections[1].value = remaining;
        }

        if text.sections[1].value.is_empty() {
            match word.0 {
                WordTarget::Enemy(id) => {
                    if let Ok(player) = player_q.get_single() {
                        // TODO: eventually check if ammo > gun's ammo consumption per shot
                        if player.ammo.0 > 0 {
                            enemy_event_writer.send(EnemyShotEvent(id));
                        }
                    }
                }
                WordTarget::Reload => {
                    reload_event_writer.send(PlayerReloadEvent);
                }
                WordTarget::Heal => {
                    heal_event_writer.send(PlayerHealEvent);
                }
                WordTarget::Move(id) => {
                    move_event_writer.send(CharacterMoveEvent {
                        character_id: 0,
                        target_id: id
                    });
                }
            }
            //TODO: probably should move the below elsewhere so its not edited in two places
            if input_str.len() > word.1.len() {
                input_text.single_mut().sections[0].value = input_str[word.1.len()..].to_string();
            } else {
                input_text.single_mut().sections[0].value = "".to_string();
            }
            break;
        }
    }
}
