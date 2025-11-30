use std::fmt::Display;

use crate::form::note::{Note, WrittenNote};

use super::note::Spelling;

#[derive(Debug, Clone)]
pub struct Scale {
    pub notes: Vec<Note>,
}

impl Scale {
    pub fn spell(&self, spelling: Spelling) -> WrittenScale {
        match spelling {
            Spelling::Sharp => self.sharp(),
            Spelling::Flat => self.flat(),
        }
    }

    pub fn sharp(&self) -> WrittenScale {
        WrittenScale {
            notes: self.notes.iter().map(|n| n.sharp()).collect(),
        }
    }
    pub fn flat(&self) -> WrittenScale {
        WrittenScale {
            notes: self.notes.iter().map(|n| n.flat()).collect(),
        }
    }
}

impl IntoIterator for Scale {
    type Item = Note;
    type IntoIter = std::vec::IntoIter<Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}

pub struct WrittenScale {
    pub notes: Vec<WrittenNote>,
}

impl Display for WrittenScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for note in self.notes.iter() {
            write!(f, "{} ", note)?;
        }
        Ok(())
    }
}
