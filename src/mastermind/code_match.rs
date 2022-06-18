use crate::mastermind::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeMatch(pub [MatchLevels; 4]);

impl GuessString for CodeMatch {
    fn to_guess_string(&self) -> String {
        self.iter().map(MatchLevels::to_char).collect()
    }
    fn from_guess_string(guess: &str) -> Self {
        assert_eq!(guess.len(), 4, "Wrong string length");
        CodeMatch(
            guess
                .chars()
                .map(MatchLevels::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Prettify for CodeMatch {
    fn prettify(&self) -> String {
        itertools::Itertools::intersperse(self.iter().map(MatchLevels::prettify), " ".to_string())
            .collect()
    }
}

impl Deref for CodeMatch {
    type Target = [MatchLevels; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CodeMatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
