use std::{collections::HashMap, fmt::Display};

use super::mistake::Mistake;

/// To what extent an analysis of a transcription conformes to the exercise
#[derive(Debug, Clone)]
pub struct Correction {
    pub amount_of_beats: usize,
    /// All beats where something incorrect was played
    /// Maps beat to mistake (mistakes also save the beat, so there's some double administration that needs to be done correctly)
    pub mistakes: HashMap<u32, Mistake>,
}

impl Correction {
    pub fn score(&self) -> f32 {
        1.0 - (self.mistakes.len() as f32 / self.amount_of_beats as f32)
    }
}

impl Display for Correction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mistakes = self.mistakes.clone();
        let mut sortable = mistakes.values().collect::<Vec<_>>();
        sortable.sort_by(|m1, m2| m1.beat.cmp(&m2.beat));

        for mistake in sortable.iter() {
            writeln!(f, "{}", mistake)?
        }
        writeln!(f, "{:.1}% correct.", self.score() * 100.)?;
        Ok(())
    }
}
