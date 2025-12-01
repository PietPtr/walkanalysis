use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::form::{
    interval::Interval,
    note::{Note, WrittenNote},
};

use super::note::Spelling;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub symbol: Option<String>,
}

impl Chord {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            notes,
            symbol: None,
        }
    }

    pub fn has_seventh(&self) -> bool {
        // TODO: big chord refactor where we define the notes not as a vec but as a hashmap of chordtone to note
        self.notes.len() > 3
    }

    pub fn spell(&self, spelling: Spelling) -> WrittenChord {
        match spelling {
            Spelling::Sharp => self.sharp(),
            Spelling::Flat => self.flat(),
        }
    }

    pub fn flat(&self) -> WrittenChord {
        WrittenChord {
            notes: self.notes.iter().map(|n| n.flat()).collect(),
        }
    }

    pub fn sharp(&self) -> WrittenChord {
        WrittenChord {
            notes: self.notes.iter().map(|n| n.sharp()).collect(),
        }
    }

    pub fn auto_spell(&self) -> WrittenChord {
        let sharp = self.sharp();
        let flat = self.flat();

        if sharp.is_spelled_correctly() {
            sharp
        } else if flat.is_spelled_correctly() {
            flat
        } else {
            unimplemented!("Cannot spell {}/{} automatically", sharp, flat)
        }
    }

    pub fn role(&self, note: Note) -> ChordTone {
        self.notes
            .iter()
            .position(|&n| n == note)
            .map(|index| ChordTone::from_note_index(index))
            .unwrap_or(ChordTone::NoChordTone)
    }

    pub fn note(&self, chord_tone: ChordTone) -> Option<Note> {
        self.notes.get(chord_tone.to_note_index()).copied()
    }

    pub fn symbol(&self) -> Option<String> {
        // start with root
        // if it has a minor third, add min
        // if it has a tritone, add b5
        // if it has a minor seventh, add 7
        // if it has a major seventh, add maj7

        if self.symbol.is_some() {
            return self.symbol.clone();
        }

        let interval = |note_index_bottom, note_index_top| {
            self.notes
                .get(note_index_bottom)
                .copied()
                .and_then(|root| {
                    self.notes
                        .get(note_index_top)
                        .copied()
                        .map(|third| (root, third))
                })
                .and_then(|(root, third)| Interval::find(root, third))
        };

        let third = interval(0, 1);
        let fifth = interval(0, 2);
        let seventh = interval(0, 3);

        let mut symbol = "".to_string();

        if third == Some(Interval::MinorThird) {
            symbol.push_str("m");
        }

        if seventh == Some(Interval::MinorSeventh) {
            symbol.push_str("7");
        }

        if fifth == Some(Interval::Tritone) {
            symbol.push_str("♭5");
        }

        if seventh == Some(Interval::MajorSeventh) {
            symbol.push_str("maj7");
        }
        Some(symbol)
    }

    pub fn spell_symbol(&self, spelling: Spelling) -> String {
        match spelling {
            Spelling::Sharp => self.sharp_symbol(),
            Spelling::Flat => self.flat_symbol(),
        }
    }

    pub fn flat_symbol(&self) -> String {
        format!(
            "{}{}",
            self.flat().root(),
            self.symbol().unwrap_or("".to_string())
        )
    }
    pub fn sharp_symbol(&self) -> String {
        format!(
            "{}{}",
            self.sharp().root(),
            self.symbol().unwrap_or("".to_string())
        )
    }
}

pub struct WrittenChord {
    notes: Vec<WrittenNote>,
}

impl WrittenChord {
    pub fn root(&self) -> WrittenNote {
        self.notes.get(0).copied().unwrap()
    }

    pub fn is_spelled_correctly(&self) -> bool {
        let Some(&root) = self.notes.first() else {
            return false;
        };

        let mut correct_spelling = vec![];
        let mut current_spelling = vec![];

        for (i, note) in self.notes.iter().enumerate() {
            current_spelling.push(note.name);
            if i == 0 {
                correct_spelling.push(root.name);
            } else {
                correct_spelling.push(correct_spelling.last().unwrap().spell_next())
            }
        }

        correct_spelling == current_spelling
    }
}

impl std::fmt::Display for WrittenChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for note in self.notes.iter() {
            write!(f, "{} ", note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordTone {
    Root,
    Third,
    Fifth,
    Seventh,
    NoChordTone,
}

impl ChordTone {
    fn from_note_index(index: usize) -> Self {
        match index {
            0 => ChordTone::Root,
            1 => ChordTone::Third,
            2 => ChordTone::Fifth,
            3 => ChordTone::Seventh,
            _ => ChordTone::NoChordTone,
        }
    }

    fn to_note_index(&self) -> usize {
        match self {
            ChordTone::Root => 0,
            ChordTone::Third => 1,
            ChordTone::Fifth => 2,
            ChordTone::Seventh => 3,
            ChordTone::NoChordTone => 99,
        }
    }
}

impl Display for ChordTone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChordTone::Root => write!(f, "Root"),
            ChordTone::Third => write!(f, "Third"),
            ChordTone::Fifth => write!(f, "Fifth"),
            ChordTone::Seventh => write!(f, "Seventh"),
            ChordTone::NoChordTone => write!(f, "Not a chord tone"),
        }
    }
}
