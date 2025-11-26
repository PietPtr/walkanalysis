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

pub fn bar(chord: Chord) -> FormPiece {
    FormPiece::ChordBar(chord)
}

pub fn half_bar(chord: Chord) -> FormPiece {
    FormPiece::HalfBar(chord)
}
