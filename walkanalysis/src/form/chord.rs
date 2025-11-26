use serde::{Deserialize, Serialize};

use crate::form::{
    interval::Interval,
    note::{Note, WrittenNote},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chord {
    pub notes: Vec<Note>,
}

impl Chord {
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    pub fn minor(root: Note) -> Self {
        let third = root.add_interval(Interval::MinorThird);
        let fifth = root.add_interval(Interval::PerfectFifth);
        Chord::new(vec![root, third, fifth])
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

    pub fn spell(&self) -> WrittenChord {
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
}

pub struct WrittenChord {
    notes: Vec<WrittenNote>,
}

impl WrittenChord {
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
