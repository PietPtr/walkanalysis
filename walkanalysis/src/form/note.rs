use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::form::{chord::Chord, interval::Interval};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Note {
    index: i32,
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.index.rem_euclid(12) == other.index.rem_euclid(12)
    }
}

impl From<i32> for Note {
    fn from(value: i32) -> Self {
        Note {
            index: value.rem_euclid(12),
        }
    }
}

impl Note {
    pub const fn new(note_name: NoteName, accidental: Accidental) -> Self {
        let mut index = match note_name {
            NoteName::A => 0,
            NoteName::B => 2,
            NoteName::C => 3,
            NoteName::D => 5,
            NoteName::E => 7,
            NoteName::F => 8,
            NoteName::G => 10,
        };

        index += accidental.value();

        Self { index }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn spell(&self, spelling: Spelling) -> WrittenNote {
        match spelling {
            Spelling::Sharp => self.sharp(),
            Spelling::Flat => self.flat(),
        }
    }

    pub fn sharp(&self) -> WrittenNote {
        let (name, accidental) = match self.index.rem_euclid(12) {
            0 => (NoteName::A, Accidental::Natural),
            1 => (NoteName::A, Accidental::Sharp),
            2 => (NoteName::B, Accidental::Natural),
            3 => (NoteName::C, Accidental::Natural),
            4 => (NoteName::C, Accidental::Sharp),
            5 => (NoteName::D, Accidental::Natural),
            6 => (NoteName::D, Accidental::Sharp),
            7 => (NoteName::E, Accidental::Natural),
            8 => (NoteName::F, Accidental::Natural),
            9 => (NoteName::F, Accidental::Sharp),
            10 => (NoteName::G, Accidental::Natural),
            11 => (NoteName::G, Accidental::Sharp),
            _ => unreachable!(),
        };

        WrittenNote { name, accidental }
    }

    pub fn flat(&self) -> WrittenNote {
        let (name, accidental) = match self.index.rem_euclid(12) {
            0 => (NoteName::A, Accidental::Natural),
            1 => (NoteName::B, Accidental::Flat),
            2 => (NoteName::B, Accidental::Natural),
            3 => (NoteName::C, Accidental::Natural),
            4 => (NoteName::D, Accidental::Flat),
            5 => (NoteName::D, Accidental::Natural),
            6 => (NoteName::E, Accidental::Flat),
            7 => (NoteName::E, Accidental::Natural),
            8 => (NoteName::F, Accidental::Natural),
            9 => (NoteName::G, Accidental::Flat),
            10 => (NoteName::G, Accidental::Natural),
            11 => (NoteName::A, Accidental::Flat),
            _ => unreachable!(),
        };
        WrittenNote { name, accidental }
    }

    pub fn add_steps(&self, steps: i32) -> Self {
        Self {
            index: (self.index + steps).rem_euclid(12),
        }
    }

    pub fn add_interval(&self, interval: Interval) -> Self {
        self.add_steps(interval.steps())
    }

    pub fn min(&self) -> Chord {
        let third = self.add_interval(Interval::MinorThird);
        let fifth = self.add_interval(Interval::PerfectFifth);
        let mut chord = Chord::new(vec![*self, third, fifth]);
        chord.symbol = Some("m".into());
        chord
    }

    pub fn maj(&self) -> Chord {
        let third = self.add_interval(Interval::MajorThird);
        let fifth = self.add_interval(Interval::PerfectFifth);
        let mut chord = Chord::new(vec![*self, third, fifth]);
        chord.symbol = Some("maj".into());
        chord
    }

    pub fn maj7(&self) -> Chord {
        let third = self.add_interval(Interval::MajorThird);
        let fifth = self.add_interval(Interval::PerfectFifth);
        let seventh = self.add_interval(Interval::MajorSeventh);
        let mut chord = Chord::new(vec![*self, third, fifth, seventh]);
        chord.symbol = Some("maj7".into());
        chord
    }

    pub fn min7(&self) -> Chord {
        let third = self.add_interval(Interval::MinorThird);
        let fifth = self.add_interval(Interval::PerfectFifth);
        let seventh = self.add_interval(Interval::MinorSeventh);
        let mut chord = Chord::new(vec![*self, third, fifth, seventh]);
        chord.symbol = Some("min7".into());
        chord
    }

    pub fn dominant7(&self) -> Chord {
        let third = self.add_interval(Interval::MajorThird);
        let fifth = self.add_interval(Interval::PerfectFifth);
        let seventh = self.add_interval(Interval::MinorSeventh);
        let mut chord = Chord::new(vec![*self, third, fifth, seventh]);
        chord.symbol = Some("7".into());
        chord
    }

    pub fn dim(&self) -> Chord {
        let third = self.add_interval(Interval::MinorThird);
        let fifth = self.add_interval(Interval::DiminishedFifth);
        let mut chord = Chord::new(vec![*self, third, fifth]);
        chord.symbol = Some("dim".into());
        chord
    }

    pub fn m7b5(&self) -> Chord {
        let third = self.add_interval(Interval::MinorThird);
        let fifth = self.add_interval(Interval::DiminishedFifth);
        let seventh = self.add_interval(Interval::MinorSeventh);
        let mut chord = Chord::new(vec![*self, third, fifth, seventh]);
        chord.symbol = Some("m7b5".into());
        chord
    }

    pub fn dim7(&self) -> Chord {
        let third = self.add_interval(Interval::MinorThird);
        let fifth = third.add_interval(Interval::MinorThird);
        let seventh = fifth.add_interval(Interval::MinorThird);
        let mut chord = Chord::new(vec![*self, third, fifth, seventh]);
        chord.symbol = Some("dim7".into());
        chord
    }

    /// Returns the note closest to that frequency, and the error.
    /// Error = 0 means spot on the note, -1 is 50 cents flat, +1 si 50 cents sharp.
    pub fn from_frequency(frequency: f32) -> (Self, f32) {
        if frequency <= 0. {
            todo!("Neatly handle the frequence 0Hz case");
        }

        let note_index_f = ((frequency / 55.0).log10() * 12.) / 2f32.log10();

        let error = (note_index_f + 0.5).rem_euclid(1.0) - 0.5;
        let note_index = note_index_f.round() as i32 % 12;

        (Self { index: note_index }, error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoteName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl NoteName {
    pub fn spell_next(&self) -> NoteName {
        match self {
            NoteName::A => NoteName::C,
            NoteName::B => NoteName::D,
            NoteName::C => NoteName::E,
            NoteName::D => NoteName::F,
            NoteName::E => NoteName::G,
            NoteName::F => NoteName::A,
            NoteName::G => NoteName::B,
        }
    }
}

impl std::fmt::Display for NoteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NoteName::A => "A",
            NoteName::B => "B",
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Spelling {
    Sharp,
    Flat,
}

impl Display for Spelling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Spelling::Sharp => "♯",
            Spelling::Flat => "♭",
        };
        write!(f, "{}", s)
    }
}

impl Accidental {
    pub const fn value(&self) -> i32 {
        match self {
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::Flat => -1,
        }
    }
}

impl std::fmt::Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Accidental::Natural => "",
            Accidental::Sharp => "♯",
            Accidental::Flat => "♭",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WrittenNote {
    pub name: NoteName,
    pub accidental: Accidental,
}
impl WrittenNote {
    pub(crate) fn unwrite(&self) -> Note {
        Note::new(self.name, self.accidental)
    }
}

impl Display for WrittenNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name, self.accidental)
    }
}

pub const A: Note = Note::new(NoteName::A, Accidental::Natural);
pub const A_FLAT: Note = Note::new(NoteName::A, Accidental::Flat);
pub const A_SHARP: Note = Note::new(NoteName::A, Accidental::Sharp);
pub const B: Note = Note::new(NoteName::B, Accidental::Natural);
pub const B_FLAT: Note = Note::new(NoteName::B, Accidental::Flat);
pub const B_SHARP: Note = Note::new(NoteName::B, Accidental::Sharp);
pub const C: Note = Note::new(NoteName::C, Accidental::Natural);
pub const C_FLAT: Note = Note::new(NoteName::C, Accidental::Flat);
pub const C_SHARP: Note = Note::new(NoteName::C, Accidental::Sharp);
pub const D: Note = Note::new(NoteName::D, Accidental::Natural);
pub const D_FLAT: Note = Note::new(NoteName::D, Accidental::Flat);
pub const D_SHARP: Note = Note::new(NoteName::D, Accidental::Sharp);
pub const E: Note = Note::new(NoteName::E, Accidental::Natural);
pub const E_FLAT: Note = Note::new(NoteName::E, Accidental::Flat);
pub const E_SHARP: Note = Note::new(NoteName::E, Accidental::Sharp);
pub const F: Note = Note::new(NoteName::F, Accidental::Natural);
pub const F_FLAT: Note = Note::new(NoteName::F, Accidental::Flat);
pub const F_SHARP: Note = Note::new(NoteName::F, Accidental::Sharp);
pub const G: Note = Note::new(NoteName::G, Accidental::Natural);
pub const G_FLAT: Note = Note::new(NoteName::G, Accidental::Flat);
pub const G_SHARP: Note = Note::new(NoteName::G, Accidental::Sharp);
