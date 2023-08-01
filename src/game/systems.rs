//! Things like play/pause systems should go here

use crate::game::resources::RandomWord;

use super::resources::WordBank;
use bevy::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn insert_word_bank(mut commands: Commands) {
    // 100 words per category for now
    let mut word_bank = WordBank::default();
    // cur_dir is always ~/great-lexicon-war/
    let reader = BufReader::new(
        File::open("assets/words/easy.txt").expect("Error reading easy words from file"),
    );
    // assuming one word per line
    for line in reader.lines() {
        if let Ok(word) = line {
            word_bank.easy.push(word);
        }
    }

    let reader = BufReader::new(
        File::open("assets/words/medium.txt").expect("Error reading medium words from file"),
    );
    for line in reader.lines() {
        if let Ok(word) = line {
            word_bank.med.push(word);
        }
    }

    let reader = BufReader::new(
        File::open("assets/words/hard.txt").expect("Error reading hard words from file"),
    );
    for line in reader.lines() {
        if let Ok(word) = line {
            word_bank.hard.push(word);
        }
    }

    commands.insert_resource(word_bank);
}

pub fn test_words(keyboard_input: Res<Input<KeyCode>>, words: Res<WordBank>) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        println!("easy: {}", words.get_easy_word().unwrap_or_default());
        println!("med: {}", words.get_med_word().unwrap_or_default());
        println!("hard: {}", words.get_hard_word().unwrap_or_default());
    }
}
