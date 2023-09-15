use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use super::{word_match::components::Word, WordComplexity};

/// Collection of words to choose from at "random"
///
/// * Must always use as `ResMut<WordBank>` due to modification for word uniqueness
#[derive(Resource)]
pub struct WordBank {
    pub easy: Vec<String>,
    easy_ptr: usize,
    pub med: Vec<String>,
    med_ptr: usize,
    pub hard: Vec<String>,
    hard_ptr: usize,
    pub extreme: Vec<String>,
    extreme_ptr: usize,
}

#[derive(Resource, Default)]
pub struct CharacterHandles {
    pub soviet_idle: Handle<TextureAtlas>,
    pub soviet_walk: Handle<TextureAtlas>,
    pub soviet_fire: Handle<TextureAtlas>,
    pub german_walk: Handle<TextureAtlas>,
    pub german_fire: Handle<TextureAtlas>,
    pub barrier: Handle<TextureAtlas>
}

pub trait RandomWord {
    fn get_word<T: Component>(
        &mut self,
        complexity: WordComplexity,
        word_query: &Query<&Word, (With<Word>, Without<T>)>,
    ) -> String;
}

impl RandomWord for WordBank {
    fn get_word<T: Component>(
        &mut self,
        complexity: WordComplexity,
        word_q: &Query<&Word, (With<Word>, Without<T>)>,
    ) -> String {
        let mut random_word = match complexity {
            WordComplexity::Easy => self.get_easy_word(),
            WordComplexity::Medium => self.get_med_word(),
            WordComplexity::Hard => self.get_hard_word(),
            WordComplexity::Extreme => self.get_extreme_word(),
        };
        let mut dup = word_q.iter().any(|word| word.1 == random_word);
        while dup {
            random_word = match complexity {
                WordComplexity::Easy => self.get_easy_word(),
                WordComplexity::Medium => self.get_med_word(),
                WordComplexity::Hard => self.get_hard_word(),
                WordComplexity::Extreme => self.get_extreme_word(),
            };
            dup = word_q.iter().any(|word| word.1 == random_word);
        }
        random_word
    }
}

// Vecs will be shuffled on creation
impl WordBank {
    fn get_easy_word(&mut self) -> String {
        if self.easy_ptr >= self.easy.len() {
            let mut rng = thread_rng();
            self.easy.shuffle(&mut rng);
            self.easy_ptr = 0;
        }
        let word = self.easy[self.easy_ptr].clone();
        self.easy_ptr += 1;
        word
    }
    fn get_med_word(&mut self) -> String {
        if self.med_ptr >= self.med.len() {
            let mut rng = thread_rng();
            self.med.shuffle(&mut rng);
            self.med_ptr = 0;
        }
        let word = self.med[self.med_ptr].clone();
        self.med_ptr += 1;
        word
    }
    fn get_hard_word(&mut self) -> String {
        if self.hard_ptr >= self.hard.len() {
            let mut rng = thread_rng();
            self.hard.shuffle(&mut rng);
            self.hard_ptr = 0;
        }
        let word = self.hard[self.hard_ptr].clone();
        self.hard_ptr += 1;
        word
    }
    fn get_extreme_word(&mut self) -> String {
        if self.extreme_ptr >= self.extreme.len() {
            let mut rng = thread_rng();
            self.extreme.shuffle(&mut rng);
            self.extreme_ptr = 0;
        }
        let word = self.extreme[self.extreme_ptr].clone();
        self.extreme_ptr += 1;
        word
    }
}

impl Default for WordBank {
    fn default() -> Self {
        WordBank {
            easy: Vec::with_capacity(100),
            easy_ptr: 0,
            med: Vec::with_capacity(100),
            med_ptr: 0,
            hard: Vec::with_capacity(100),
            hard_ptr: 0,
            extreme: Vec::with_capacity(10),
            extreme_ptr: 0,
        }
    }
}
