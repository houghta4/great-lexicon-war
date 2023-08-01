use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Resource)]
pub struct WordBank {
    pub easy: Vec<String>,
    pub med: Vec<String>,
    pub hard: Vec<String>,
}

pub trait RandomWord {
    fn get_easy_word(&self) -> Option<String>;
    fn get_med_word(&self) -> Option<String>;
    fn get_hard_word(&self) -> Option<String>;
}

// TODO: Test if its worth to remove each word, or have a parallel vec to push to containing "used" words
impl RandomWord for WordBank {
    // not removing
    // clone to give ownership to caller
    fn get_easy_word(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.easy.choose(&mut rng).cloned()
    }
    fn get_med_word(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.med.choose(&mut rng).cloned()
    }
    fn get_hard_word(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.hard.choose(&mut rng).cloned()
    }
}

impl Default for WordBank {
    fn default() -> Self {
        WordBank {
            easy: Vec::with_capacity(100),
            med: Vec::with_capacity(100),
            hard: Vec::with_capacity(100),
        }
    }
}
