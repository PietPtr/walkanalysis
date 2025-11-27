use std::fmt::Display;

use crate::{
    exercise::analysis::{Analysis, NoteAnalysis},
    form::note::Note,
};

pub mod analysis;
pub mod arpeggios_up;

pub trait Exercise {
    /// User facing explanation about what must be played for a perfect score
    const EXPLANATION: &str;
    /// Correction function: given an analysis of a transcription, how many beats fit
    /// both the form and the exercise?
    fn correct(&mut self, analysis: Analysis) -> Correction;
}

/// To what extent an analysis of a transcription conformes to the exercise
#[derive(Debug, Clone)]
pub struct Correction {
    amount_of_beats: usize,
    /// All beats where something incorrect was played
    mistakes: Vec<Mistake>,
}

impl Correction {
    pub fn score(&self) -> f32 {
        1.0 - (self.mistakes.len() as f32 / self.amount_of_beats as f32)
    }
}

impl Display for Correction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sortable = self.mistakes.clone();
        sortable.sort_by(|m1, m2| m1.beat.cmp(&m2.beat));

        for mistake in sortable.iter() {
            writeln!(f, "{}", mistake)?
        }
        writeln!(f, "{:.1}% correct.", self.score() * 100.)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Mistake {
    beat: u32,
    mistake: MistakeKind,
}

impl Display for Mistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}.{}] {}",
            self.beat / 4,
            self.beat % 4 + 1,
            self.mistake
        )
    }
}

#[derive(Debug, Clone)]
pub enum MistakeKind {
    WrongNote { played: Note, expected: Note },
    ExpectedSilence { found: NoteAnalysis },
    ExpectedNote { found: NoteAnalysis },
}

impl Display for MistakeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MistakeKind::WrongNote { played, expected } => write!(
                f,
                "Wrong note, played {} expected {}.",
                played.flat(),
                expected.flat()
            )?,
            MistakeKind::ExpectedSilence { found } => {
                write!(f, "Expected silence, found {:?}", found)?
            }
            MistakeKind::ExpectedNote { found } => {
                write!(f, "Expected some note, found {:?}", found)?
            }
        }
        Ok(())
    }
}
