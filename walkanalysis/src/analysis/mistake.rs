use std::fmt::Display;

use crate::form::{chord::ChordTone, note::Note};

use super::analysis::NoteAnalysis;

#[derive(Debug, Clone, Copy)]
pub struct Mistake {
    pub beat: u32,
    pub mistake: MistakeKind,
}

impl Display for Mistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}.{} | {}] {}",
            self.beat / 4 + 1,
            self.beat % 4 + 1,
            self.beat,
            self.mistake
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MistakeKind {
    WrongNote {
        played: Note,
        expected: Note,
    },
    ExpectedSilence {
        found: NoteAnalysis,
    },
    ExpectedNote {
        found: NoteAnalysis,
    },
    ExpectedChordTone {
        played_chord_tone: ChordTone,
        played_note: Note,
        expected_example: Note, // A random note that would've been correct
    },
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
            MistakeKind::ExpectedChordTone {
                played_chord_tone,
                played_note,
                expected_example,
            } => write!(
                f,
                "Expected any chord tone, found {:?} (found: {}, expected e.g.: {})",
                played_chord_tone,
                played_note.flat(),
                expected_example.flat()
            )?,
        }
        Ok(())
    }
}
