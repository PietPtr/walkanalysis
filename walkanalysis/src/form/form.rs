use std::{fmt::Display, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::form::{chord::Chord, key::Key};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Form {
    pub tempo: u32,
    pub music: Vec<FormPiece>,
}

// Defines a bar of 4/4 form
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FormPiece {
    /// Set the key of the piece, if absent, assumes C major.
    /// This is necessary to determine note roles
    Key(Key),
    /// Four beats of silence
    CountInBar,
    /// A bar where a single chord is played the whole time
    ChordBar(Chord),
    /// A bar with a chord held for a half note
    HalfBar(Chord),
}

impl FormPiece {
    pub fn length_in_beats(&self) -> usize {
        match self {
            FormPiece::Key(_) => 0,
            FormPiece::CountInBar => 4,
            FormPiece::ChordBar(_) => 4,
            FormPiece::HalfBar(_) => 2,
        }
    }
}

impl Display for FormPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormPiece::Key(key) => write!(f, "{}", key),
            FormPiece::CountInBar => write!(f, "Count in"),
            FormPiece::ChordBar(chord) => write!(f, "𝅝 {}/{}", chord.sharp(), chord.flat()),
            FormPiece::HalfBar(chord) => write!(f, "𝅗𝅥 {}/{}", chord.sharp(), chord.flat()),
        }
    }
}

pub fn bar(chord: Chord) -> FormPiece {
    FormPiece::ChordBar(chord)
}

pub fn half_bar(chord: Chord) -> FormPiece {
    FormPiece::HalfBar(chord)
}

impl Form {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Form, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn where_are_we(&self, beat_number: usize) -> Option<FormPiece> {
        // TODO: kinda inefficient
        let mut beat = 0;
        let mut last_piece = self.music.first().expect("Empty music!").clone();

        for piece in self.music.iter() {
            if beat >= beat_number {
                return Some(last_piece);
            }
            beat += piece.length_in_beats();
            last_piece = piece.clone();
        }

        return None;
    }
}
