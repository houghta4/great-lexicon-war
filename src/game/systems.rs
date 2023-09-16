use bevy::prelude::*;

use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use crate::game::resources::CharacterHandles;
use crate::game::SpriteSheetInfo;

use super::{
    input::components::InputText,
    resources::{RandomWord, WordBank},
    word_match::components::Word,
    InGameState, WordComplexity,
};

const SOVIET_IDLE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soviet_soldier/ppsh_idle.png",
    x: 128.0,
    y: 128.0,
    cols: 10,
    rows: 1,
};
const SOVIET_WALK: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soviet_soldier/ppsh_walking.png",
    x: 128.0,
    y: 128.0,
    cols: 8,
    rows: 1,
};
const SOVIET_FIRE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/soviet_soldier/ppsh_firing.png",
    x: 128.0,
    y: 128.0,
    cols: 10,
    rows: 1,
};
const GERMAN_WALK: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/german_soldier/mp40_walking.png",
    x: 128.0,
    y: 128.0,
    cols: 8,
    rows: 1,
};
const GERMAN_FIRE: SpriteSheetInfo = SpriteSheetInfo {
    path: "sprites/german_soldier/mp40_firing.png",
    x: 128.0,
    y: 128.0,
    cols: 10,
    rows: 1,
};

const BARRIER: SpriteSheetInfo = SpriteSheetInfo {
  path: "sprites/objects/barriers.png",
    x: 256.0,
    y: 256.0,
    cols: 1,
    rows: 1
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

/**
    Inits all texture handles for all character sprite sheets
**/
pub fn init_texture_atlas_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handles = CharacterHandles {
        soviet_idle: get_texture_atlas_handle(SOVIET_IDLE, &asset_server, &mut texture_atlases),
        soviet_walk: get_texture_atlas_handle(SOVIET_WALK, &asset_server, &mut texture_atlases),
        soviet_fire: get_texture_atlas_handle(SOVIET_FIRE, &asset_server, &mut texture_atlases),
        german_walk: get_texture_atlas_handle(GERMAN_WALK, &asset_server, &mut texture_atlases),
        german_fire: get_texture_atlas_handle(GERMAN_FIRE, &asset_server, &mut texture_atlases),
        barrier: get_texture_atlas_handle(BARRIER, &asset_server, &mut texture_atlases)
    };

    commands.insert_resource(handles);
}

fn get_texture_atlas_handle(
    cur_sprite: SpriteSheetInfo,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(cur_sprite.path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(cur_sprite.x, cur_sprite.y),
        cur_sprite.cols,
        cur_sprite.rows,
        None,
        None,
    );
    texture_atlases.add(texture_atlas)
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
            },
            _ => {}
        }
    }
}

pub fn monitor_state(game_state: Res<State<InGameState>>) {
    if game_state.is_changed() {
        println!("State -->> {:?}", game_state.get());
    }
}
