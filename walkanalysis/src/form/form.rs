use std::{fmt::Display, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::form::{chord::Chord, key::Key};

use super::key::WrittenKey;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Form {
    tempo: u32,
    key: WrittenKey,
    music: Vec<FormPiece>,
    // TODO: excuses: bars that are non-standard or may be interpreted more freely and are not checked?
}

impl Form {
    pub fn new(tempo: u32, key: WrittenKey, mut music: Vec<FormPiece>) -> Self {
        if music.get(0).cloned() != Some(FormPiece::CountOff) {
            music.insert(0, FormPiece::CountOff);
        }
        Self { tempo, key, music }
    }

    pub fn key(&self) -> WrittenKey {
        self.key
    }

    pub fn music(&self) -> &Vec<FormPiece> {
        &self.music
    }
}

// Defines a bar of 4/4 form
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FormPiece {
    /// Set the key of the piece, if absent, assumes C major.
    /// This is necessary to determine note roles
    Key(Key),
    /// expects 8 beats of counting of the tune
    CountOff,
    /// A bar where a single chord is played the whole time
    ChordBar(Chord),
    /// A bar with two chords held for a half note
    HalfBar(Chord, Chord),
    /// Where to break the line of chords
    LineBreak,
}

impl FormPiece {
    pub fn length_in_beats(&self) -> u32 {
        match self {
            FormPiece::Key(_) => 0,
            FormPiece::CountOff => 8,
            FormPiece::ChordBar(_) => 4,
            FormPiece::HalfBar(_, _) => 4,
            FormPiece::LineBreak => 0,
        }
    }
}

impl Display for FormPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormPiece::Key(key) => write!(f, "{}", key),
            FormPiece::CountOff => write!(f, "Count in"),
            FormPiece::ChordBar(chord) => write!(f, "𝅝 {}/{}", chord.sharp(), chord.flat()),
            FormPiece::HalfBar(chord1, chord2) => write!(
                f,
                "𝅗𝅥 {}/{} 𝅗𝅥 {}/{}",
                chord1.sharp(),
                chord1.flat(),
                chord2.sharp(),
                chord2.flat()
            ),
            FormPiece::LineBreak => write!(f, "\n"),
        }
    }
}

pub fn bar(chord: Chord) -> FormPiece {
    FormPiece::ChordBar(chord)
}

pub fn half_bar(chord1: Chord, chord2: Chord) -> FormPiece {
    FormPiece::HalfBar(chord1, chord2)
}

impl Form {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Form, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn length_in_beats(&self) -> u32 {
        self.music
            .iter()
            .fold(0, |acc, elem| acc + elem.length_in_beats())
    }
}
