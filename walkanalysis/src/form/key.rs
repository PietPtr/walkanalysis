use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::form::{interval::Interval, note::Note, scale::Scale};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Quality {
    Major,
    Minor,
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Major => write!(f, "Major"),
            Quality::Minor => write!(f, "Minor"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Key {
    root: Note,
    quality: Quality,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} {}",
            self.root.sharp(),
            self.root.flat(),
            self.quality
        )
    }
}

impl Key {
    pub fn new(root: Note, quality: Quality) -> Key {
        Self { root, quality }
    }

    pub fn scale(&self) -> Scale {
        Scale {
            notes: match self.quality {
                Quality::Major => vec![
                    self.root,
                    self.root.add_interval(Interval::MajorSecond),
                    self.root.add_interval(Interval::MajorThird),
                    self.root.add_interval(Interval::PerfectFourth),
                    self.root.add_interval(Interval::PerfectFifth),
                    self.root.add_interval(Interval::MajorSixth),
                    self.root.add_interval(Interval::MajorSeventh),
                ],
                Quality::Minor => vec![
                    self.root,
                    self.root.add_interval(Interval::MajorSecond),
                    self.root.add_interval(Interval::MinorThird),
                    self.root.add_interval(Interval::PerfectFourth),
                    self.root.add_interval(Interval::PerfectFifth),
                    self.root.add_interval(Interval::MinorSixth),
                    self.root.add_interval(Interval::MinorSeventh),
                ],
            },
        }
    }

    pub fn role(&self, note: Note) -> Degree {
        let result = self
            .scale()
            .into_iter()
            .position(|n| n == note)
            .map(|index| Degree::from_note_index(index))
            .unwrap_or(Degree::Chromatic);

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Degree {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Chromatic,
}

impl Degree {
    fn from_note_index(index: usize) -> Self {
        match index {
            0 => Degree::First,
            1 => Degree::Second,
            2 => Degree::Third,
            3 => Degree::Fourth,
            4 => Degree::Fifth,
            5 => Degree::Sixth,
            6 => Degree::Seventh,
            _ => Degree::Chromatic,
        }
    }
}
