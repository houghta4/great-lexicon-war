use bevy::prelude::*;

use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{
    input::components::InputText,
    resources::{RandomWord, WordBank},
    word_match::components::Word,
    InGameState, WordComplexity,
};

pub fn insert_word_bank(mut commands: Commands) {
    // 100 words per category for now
    let mut word_bank = WordBank::default();
    let mut rng = thread_rng();

    // cur_dir is always ~/great-lexicon-war/
    let reader = BufReader::new(
        File::open("assets/words/easy.txt").expect("Error reading easy words from file"),
    );
    // assuming one word per line,
    for word in reader.lines().flatten() {
        word_bank.easy.push(word);
    }
    word_bank.easy.shuffle(&mut rng);

    let reader = BufReader::new(
        File::open("assets/words/medium.txt").expect("Error reading medium words from file"),
    );
    for word in reader.lines().flatten() {
        word_bank.med.push(word);
    }
    word_bank.med.shuffle(&mut rng);

    let reader = BufReader::new(
        File::open("assets/words/hard.txt").expect("Error reading hard words from file"),
    );
    for word in reader.lines().flatten() {
        word_bank.hard.push(word);
    }
    word_bank.hard.shuffle(&mut rng);

    let reader = BufReader::new(
        File::open("assets/words/extreme.txt").expect("Error reading extreme words from file"),
    );
    for word in reader.lines().flatten() {
        word_bank.extreme.push(word);
    }
    word_bank.extreme.shuffle(&mut rng);

    commands.insert_resource(word_bank);
}

pub fn test_words(
    keyboard_input: Res<Input<KeyCode>>,
    mut words: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<InputText>)>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        println!("easy: {}", words.get_word(WordComplexity::Easy, &word_q));
        println!("med: {}", words.get_word(WordComplexity::Medium, &word_q));
        println!("hard: {}", words.get_word(WordComplexity::Hard, &word_q));
        println!(
            "extreme: {}",
            words.get_word(WordComplexity::Extreme, &word_q)
        );
    }
}

pub fn pause_game(mut next_game_state: ResMut<NextState<InGameState>>) {
    next_game_state.set(InGameState::Paused);
}

// may want this on exit? might be useless with toggle
pub fn resume_game(mut next_game_state: ResMut<NextState<InGameState>>) {
    next_game_state.set(InGameState::Running);
}

pub fn toggle_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<InGameState>>,
    mut next_game_state: ResMut<NextState<InGameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            InGameState::Running => {
                next_game_state.set(InGameState::Paused);
            }
            InGameState::Paused => {
                next_game_state.set(InGameState::Running);
            }
        }
    }
}

pub fn monitor_state(game_state: Res<State<InGameState>>) {
    if game_state.is_changed() {
        println!("State -->> {:?}", game_state.get());
    }
}
