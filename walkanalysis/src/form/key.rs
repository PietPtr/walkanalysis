use serde::{Deserialize, Serialize};

use crate::form::note::Note;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Quality {
    Major,
    Minor,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Key(Note, Quality);

impl Key {
    pub fn new(root: Note, quality: Quality) -> Key {
        Self(root, quality)
    }
}
