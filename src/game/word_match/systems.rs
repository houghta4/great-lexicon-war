use std::cmp;
use bevy::prelude::*;
use crate::game::input::components::InputText;
use crate::game::word_match::components::{Word, WordEvent};

//TODO: rewrite to be more efficient
/**
    Checks for matches between Words and user input
**/
pub fn check_matches(
    mut input_text: Query<&mut Text, With<InputText>>,
    mut words: Query<(&mut Text, &Word), (With<Word>, Without<InputText>)>,
    mut event_writer: EventWriter<WordEvent>
) {

    let input_str = input_text.single_mut().sections[0].value.to_string();
    for (mut text, word) in &mut words {
        if input_str.is_empty() {
            if text.sections[0].value.is_empty() {
                text.sections[0].value = "".to_string();
                text.sections[1].value = word.1.to_string();
            }
        } else if !text.sections[1].value.is_empty() {
            let mut completed = "".to_string();
            let mut remaining = "".to_string();

            let mut input_chars = input_str.chars();
            let mut target_chars = word.1.chars();

            for _n in 0..cmp::min(word.1.len(), input_str.len()) {
                let target_char = target_chars.next().unwrap();
                if target_char == input_chars.next().unwrap() {
                    completed.push(target_char);
                } else {
                    completed = "".to_string();
                    remaining = word.1.to_string();
                    break;
                }
            }

            if remaining.is_empty() && word.1.len() > input_str.len() {
                remaining = word.1.to_string()[input_str.len()..].to_string();
            }

            text.sections[0].value = completed;
            text.sections[1].value = remaining;
        }

        if text.sections[1].value.is_empty() {
            event_writer.send(WordEvent(word.0));
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

/**
    Example catcher for the event, will need to be processed by id somehow, or fire off different events, not sure
**/
pub fn catch_events(mut event_reader: EventReader<WordEvent>) {
    for event in event_reader.iter() {
        println!("event! {}", event.0);
    }
}