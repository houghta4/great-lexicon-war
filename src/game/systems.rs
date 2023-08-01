use bevy::prelude::*;

use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{
    resources::{RandomWord, WordBank},
    WordComplexity,
};

pub fn insert_word_bank(mut commands: Commands) {
    // 100 words per category for now
    let mut word_bank = WordBank::default();
    let mut rng = thread_rng();

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
    word_bank.easy.shuffle(&mut rng);

    let reader = BufReader::new(
        File::open("assets/words/medium.txt").expect("Error reading medium words from file"),
    );
    for line in reader.lines() {
        if let Ok(word) = line {
            word_bank.med.push(word);
        }
    }
    word_bank.med.shuffle(&mut rng);

    let reader = BufReader::new(
        File::open("assets/words/hard.txt").expect("Error reading hard words from file"),
    );
    for line in reader.lines() {
        if let Ok(word) = line {
            word_bank.hard.push(word);
        }
    }
    word_bank.hard.shuffle(&mut rng);

    commands.insert_resource(word_bank);
}

pub fn test_words(keyboard_input: Res<Input<KeyCode>>, mut words: ResMut<WordBank>) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        println!("easy: {}", words.get_word(WordComplexity::Easy));
        println!("med: {}", words.get_word(WordComplexity::Medium));
        println!("hard: {}", words.get_word(WordComplexity::Hard));
    }
}
