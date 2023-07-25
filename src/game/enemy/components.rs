use bevy::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Component)]
pub struct EnemyWord {
    pub cur: usize, // Pointer to current letter for word match
    pub word: String,
}

impl Default for EnemyWord {
    fn default() -> Self {
        EnemyWord {
            cur: 0,
            word: "**default**".into(),
        }
    }
}

impl Display for EnemyWord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.word)
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: usize, // Percentage [0, 100]
    pub word: EnemyWord,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            health: 100,
            word: EnemyWord::default(),
        }
    }
}
