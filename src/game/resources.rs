use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use super::WordComplexity;

// Must alwways use as ResMut<WordBank>
#[derive(Resource)]
pub struct WordBank {
    pub easy: Vec<String>,
    easy_ptr: usize,
    pub med: Vec<String>,
    med_ptr: usize,
    pub hard: Vec<String>,
    hard_ptr: usize,
}
pub trait RandomWord {
    fn get_word(&mut self, complexity: WordComplexity) -> String;
}

impl RandomWord for WordBank {
    fn get_word(&mut self, complexity: WordComplexity) -> String {
        match complexity {
            WordComplexity::Easy => self.get_easy_word(),
            WordComplexity::Medium => self.get_med_word(),
            WordComplexity::Hard => self.get_hard_word(),
        }
    }
}

trait RandomWordHelper {
    fn get_easy_word(&mut self) -> String;
    fn get_med_word(&mut self) -> String;
    fn get_hard_word(&mut self) -> String;
}

// Vecs will be shuffled on creation
impl RandomWordHelper for WordBank {
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
        }
    }
}
